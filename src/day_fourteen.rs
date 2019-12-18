use lazy_static::lazy_static;
use num::integer::{gcd, lcm};
use regex::Regex;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io::BufReader;

const INPUT_FILENAME: &str = "input/day_fourteen.txt";
const ORE_KEY: &str = "ORE";

#[derive(Debug, Default, Eq)]
struct ReactionFactor {
    chemical: String,
    quantity: i64,
}

impl Hash for ReactionFactor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.chemical.hash(state);
    }
}

impl PartialEq for ReactionFactor {
    fn eq(&self, other: &Self) -> bool {
        self.chemical == other.chemical
    }
}

#[derive(Debug, Default, Eq)]
struct Reaction {
    inputs: HashMap<String, i64>,
    // TODO: multiple outputs?
    output: ReactionFactor,
}

impl Hash for Reaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for input in &self.inputs {
            input.hash(state);
        }
        self.output.hash(state);
    }
}

impl PartialEq for Reaction {
    fn eq(&self, other: &Self) -> bool {
        let num_inputs = self.inputs.len();
        if num_inputs != other.inputs.len() {
            return false;
        }
        if self.inputs != other.inputs {
            return false;
        }
        if self.output != other.output {
            return false;
        }
        true
    }
}

trait Scaling {
    fn scale(&mut self, factor: i64);
    fn reduce(&mut self);
}

impl Scaling for Reaction {
    fn scale(&mut self, factor: i64) {
        self.output.quantity *= factor;
        for v in self.inputs.values_mut() {
            *v *= factor;
        }
    }

    fn reduce(&mut self) {
        println!("Before reduction: {:#?}", self);
        let mut greatest: i64 = self.output.quantity;
        for v in self.inputs.values() {
            greatest = gcd(greatest, *v);
        }
        self.output.quantity /= greatest;
        for v in self.inputs.values_mut() {
            *v /= greatest;
        }
        println!("After reduction: {:#?}", self);
    }
}

fn parse_reactions(input_filename: &str) -> Vec<Reaction> {
    let file = File::open(input_filename).expect("Invalid filename");
    let reader = BufReader::new(file);

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"((?P<quantity>[0-9]+) (?P<chemical>[a-zA-Z]+))").unwrap();
    }

    let mut reactions = Vec::new();
    for line in reader.lines() {
        let mut reaction = Reaction::default();
        let l = line.expect("line should be valid");

        // TODO: refactor to not build vector?
        let mut factors: Vec<ReactionFactor> = RE
            .captures_iter(&l)
            .map(|c| ReactionFactor {
                chemical: c["chemical"].to_string(),
                quantity: c["quantity"].parse().expect("quantity is number"),
            })
            .collect();

        while factors.len() > 1 {
            let factor = factors.remove(0);
            reaction.inputs.insert(factor.chemical, factor.quantity);
        }
        reaction.output = factors.remove(0);
        reaction.reduce();
        reactions.push(reaction);
    }

    reactions
}

fn is_substitutable(reaction: &Reaction) -> bool {
    reaction.inputs.len() == 1 && reaction.inputs.contains_key(ORE_KEY)
}

fn substitute(substitute: &Reaction, reaction: &mut Reaction) {
    assert_eq!(substitute.inputs.len(), 1);
    assert!(substitute.inputs.contains_key(ORE_KEY));

    // find reactions where input.len() == 1 and input[0].chemical == ORE
    //     get reaction.output as FOO
    //     get reaction.input.quantity as BAR
    //     for every other reaction:
    //         for input in reaction.input:
    //             if input.chemical == FOO.chemical{
    //     find lcm input.quantity, BAR as L
    //     multiply input * L, BAR * L, FOO.quantity * L
    //     replace input chemical = FOO.chemical,
    //             input quantity = FOO.quantity * L
    //     if already exists:
    //             input quantity += already quantity
    //     replace
    //     reduce by common factors
    // }

    let factor_chemical: &str = &substitute.output.chemical;
    let factor_quantity = substitute.output.quantity;
    let mut ratio = substitute.inputs[ORE_KEY];

    if reaction.inputs.contains_key(factor_chemical) {
        let multiple = lcm(reaction.inputs[factor_chemical], factor_quantity);

        ratio *= multiple / factor_quantity;
        reaction.scale(multiple / reaction.inputs[factor_chemical]);
        reaction.inputs.remove(factor_chemical);
        if reaction.inputs.contains_key(ORE_KEY) {
            ratio += reaction.inputs[ORE_KEY];
        }
        reaction.inputs.insert(ORE_KEY.to_string(), ratio);
        reaction.reduce();
    }
}

fn reduce_to_ore_to_fuel(reactions: Vec<Reaction>) -> i64 {
    let mut nots = reactions;
    loop {
        let mut subs = Vec::new();
        let mut temps: Vec<Reaction> = Vec::new();
        while nots.len() > 0 {
            if is_substitutable(&nots[0]) {
                subs.push(nots.remove(0));
            } else {
                temps.push(nots.remove(0));
            }
        }

        println!("substituting this round: {:#?}", subs);
        for sub in subs {
            for n in temps.iter_mut() {
                substitute(&sub, n);
            }
        }

        println!("temps: {:#?}", temps);
        if temps.len() == 1 {
            temps[0].reduce();
            return *temps[0].inputs.iter().last().expect("has a last").1;
        }
        nots = temps;
    }
}

/* Algo example:
    10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL

    find reactions where input.len() == 1 and input[0].chemical == ORE
        get reaction.output as FOO
        get reaction.input.quantity as BAR
        for every other reaction:
            for input in reaction.input:
                if input.chemical == FOO.chemical{
                    find lcm input.quantity, BAR as L
                    multiply input * L, BAR * L, FOO.quantity * L
                    replace input chemical = FOO.chemical,
                            input quantity = FOO.quantity * L
                    if already exists:
                            input quantity += already quantity
                    replace
                    reduce by common factors
                }

    (1) 10 ORE => 10 A
                FOO = 10 A
                BAR = 10
        7 A, 1B => 1C
        input = 7 A
        lcm 7, 10 = 70
        reaction => 70 A + 10 B = 10 C (* lcm()/input.quantity = 10)
        FOO = 70 B (* lcm()/foo.quantity = 7)
        BAR = 70
        input chemical = ORE
        input quantity = 70
        reaction => 70 ORE + 10 B = 10 C

    (2) 1 ORE => 1 B
                FOO = 1 B
                BAR = 1
        70 ORE + 10 B = 10 C
        input = 10 B
        lcm 1, 10 = 10
        reaction = unchanged as LCM is one
        FOO = 10 B
        BAR = 10
        input chemical = ORE
        input quantity = 10 (BAR)
        when inserting in reaction, already exists, so add current quantity:
                70 + 10
        80 ORE = 10 C
        REDUCE by common factors
        8 ORE = 1 C
*/
pub fn part_one() -> i64 {
    let reactions = parse_reactions(INPUT_FILENAME);
    println!("Reaction list: {:#?}", reactions);

    reduce_to_ore_to_fuel(reactions)
}

pub fn solve() {
    println!("Day fourteen, part one: {}", part_one());
}
