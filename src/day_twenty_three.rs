use std::collections::VecDeque;

#[derive(Debug)]
struct Game {
    current_cup_index: usize,
    cups: VecDeque<i64>,
}

impl Game {
    const MIN_CUP: i64 = 1;
    const MAX_CUP: i64 = 9;

    fn next_cup_index(cup: usize) -> usize {
        if cup == Game::MAX_CUP as usize - 1 {
            Game::MIN_CUP as usize - 1
        } else {
            cup + 1
        }
    }

    fn next_cup_label(cup: i64) -> i64 {
        if cup == Game::MIN_CUP {
            Game::MAX_CUP
        } else {
            cup - 1
        }
    }

    // Modulos the index by cup length and removes+unwraps.
    fn pop_cup(&mut self, index: usize) -> i64 {
        self.cups.remove(index % self.cups.len()).unwrap()
    }

    fn do_move(&mut self) {
        println!("starting with game state: {:?}", self);
        let current_cup_label = self.cups[self.current_cup_index];

        let cups_to_move = vec![
            self.pop_cup(self.current_cup_index + 1),
            self.pop_cup(self.current_cup_index + 1),
            self.pop_cup(self.current_cup_index + 1),
        ];
        println!("picked up: {:?}", cups_to_move);
        let mut destination_cup_label = Self::next_cup_label(current_cup_label);
        while cups_to_move.contains(&destination_cup_label) {
            destination_cup_label = Self::next_cup_label(destination_cup_label);
        }

        let destination_cup_index = self
            .cups
            .iter()
            .position(|&c| c == destination_cup_label)
            .unwrap();
        println!(
            "picked destination cup {} at index: {}",
            destination_cup_label, destination_cup_index
        );
        for &cup in cups_to_move.iter().rev() {
            self.cups.insert(destination_cup_index + 1, cup);
        }
        println!("inserted cups: {:?}", self.cups);
        self.current_cup_index = Self::next_cup_index(self.current_cup_index);
    }

    fn do_move_n(&mut self, n: usize) {
        for _ in 0..n {
            self.do_move()
        }
    }

    // Currently uses the fact that this is only 10 long.
    fn print(&self) -> i64 {
        let one_index = self.cups.iter().position(|&c| c == 1).unwrap();

        let mut out: i64 = 0;
        for i in one_index..self.cups.len() {
            out *= 10;
            out += self.cups[i];
        }
        for i in 0..one_index {
            out *= 10;
            out += self.cups[i];
        }
        out
    }
}

fn part_one() -> i64 {
    let mut game = Game {
        current_cup_index: 0,
        cups: VecDeque::from(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]),
    };
    game.do_move_n(10);
    game.print()
}

fn part_two() -> i64 {
    0
}

pub fn solve() -> String {
    format!("part one: {}, part two: {}", part_one(), part_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 0, part two: 0", solve());
    }

    #[test]
    fn test_example() {
        let mut game = Game {
            current_cup_index: 0,
            cups: VecDeque::from(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]),
        };

        game.do_move_n(10);
        assert_eq!(92658374, game.print());

        game.do_move_n(90);
        assert_eq!(67384529, game.print());
    }
}
