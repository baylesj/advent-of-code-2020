use std::collections::VecDeque;

#[derive(Debug)]
struct Game {
    current_cup_index: usize,
    cups: VecDeque<i64>,
    min: i64,
    max: i64,
}

impl Game {
    fn from_vec(cups: Vec<i64>) -> Self {
        let min = *cups.iter().min().unwrap();
        let max = *cups.iter().max().unwrap();

        Game {
            current_cup_index: 0,
            cups: VecDeque::from(cups),
            min: min,
            max: max,
        }
    }

    fn next_cup_index(&self, cup: usize) -> usize {
        if cup == self.max as usize - 1 {
            self.min as usize - 1
        } else {
            cup + 1
        }
    }

    fn next_cup_label(&self, cup: i64) -> i64 {
        if cup == self.min {
            self.max
        } else {
            cup - 1
        }
    }

    // Modulos the index by cup length and removes+unwraps.
    fn pop_cup(&mut self, index: usize) -> i64 {
        let r = if index >= self.cups.len() { 0 } else { index };
        self.cups.remove(r).unwrap()
    }

    fn do_move(&mut self) {
        let current_cup_label = self.cups[self.current_cup_index];

        let cups_to_move = vec![
            self.pop_cup(self.current_cup_index + 1),
            self.pop_cup(self.current_cup_index + 1),
            self.pop_cup(self.current_cup_index + 1),
        ];
        let mut destination_cup_label = self.next_cup_label(current_cup_label);
        while cups_to_move.contains(&destination_cup_label) {
            destination_cup_label = self.next_cup_label(destination_cup_label);
        }

        let destination_cup_index = self
            .cups
            .iter()
            .position(|&c| c == destination_cup_label)
            .unwrap();
        for &cup in cups_to_move.iter().rev() {
            self.cups.insert(destination_cup_index + 1, cup);
        }

        let current_cup_moved_index = self
            .cups
            .iter()
            .position(|&c| c == current_cup_label)
            .unwrap();
        self.current_cup_index = self.next_cup_index(current_cup_moved_index);
    }

    fn do_move_n(&mut self, n: usize) {
        for _ in 0..n {
            self.do_move()
        }
    }

    // Currently uses the fact that this is only 10 long, so won't work for
    // part two.
    fn print(&self) -> i64 {
        let one_index = self.cups.iter().position(|&c| c == 1).unwrap();

        let mut out: i64 = 0;
        for i in one_index + 1..self.cups.len() {
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
    let mut game = Game::from_vec(vec![9, 1, 6, 4, 3, 8, 2, 7, 5]);
    game.do_move_n(100);
    game.print()
}

fn _part_two() -> i64 {
    const CAPACITY: usize = 1000000;
    let mut cups = Vec::<i64>::with_capacity(CAPACITY);
    cups.extend(vec![9, 1, 6, 5, 3, 8, 2, 7, 5]);
    for i in cups.len()..CAPACITY {
        cups.push(i as i64 + 1);
    }

    let mut game = Game::from_vec(cups);
    game.do_move_n(10000000);
    game.print()
}

pub fn solve() -> String {
    format!(
        "part one: {}, part two: {}",
        part_one(),
        0 /*part_two()*/
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 39564287, part two: 0", solve());
    }

    #[test]
    fn test_example() {
        let mut game = Game::from_vec(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
        game.do_move_n(10);
        assert_eq!(92658374, game.print());

        game.do_move_n(90);
        assert_eq!(67384529, game.print());
    }
}
