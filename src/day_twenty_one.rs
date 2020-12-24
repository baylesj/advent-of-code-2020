use crate::loadable::LoadableFromFile;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::VecDeque;

#[derive(Default)]
struct Game {
    player_one_deck: VecDeque<i32>,
    player_two_deck: VecDeque<i32>
}

impl LoadableFromFile for Game {
    fn load(filename: &str) -> Self {
        let file = File::open(filename).expect("Invalid filename");
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut game = Game::default();
        assert_eq!("Player 1:", lines.next().unwrap().unwrap());
        while let Ok(card_id) = lines.next().unwrap().unwrap_or_default().parse::<i32>() {
            game.player_one_deck.push_back(card_id);
        }

        assert_eq!("Player 2:", lines.next().unwrap().unwrap());
        while let Some(l) = lines.next() {
            game.player_two_deck.push_back(l.unwrap().parse::<i32>().unwrap());
        }
        game
    }
}

impl Game {
    fn find_score(&self) -> i64 {
        let winning_deck = if self.player_one_deck.len() > 0 { &self.player_one_deck } else { &self.player_two_deck};
        // Spot 0 is the top of the deck by convention.
        let mut score = 0;
        for i in 1..winning_deck.len() + 1 {
            score += i as i64 * winning_deck[winning_deck.len() - i] as i64;
        }
        score
    }
}

fn part_one() -> i64 {
    0
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
    fn test_load() {
        let game = Game::load("input/day_twenty_one_example.txt");
        assert_eq!(
            VecDeque::from(vec![9, 2, 6, 3, 1])
            , game.player_one_deck
        );
        assert_eq!(
            VecDeque::from(vec![5, 8, 4, 7, 10])
            , game.player_two_deck
        );
    }

    #[test]
    fn test_find_score() {
        let game = Game {
            player_one_deck: VecDeque::from(vec![]),
            player_two_deck: VecDeque::from(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1])
        };
        assert_eq!(306, game.find_score());
    }

    #[test]
    fn test_solve() {
        assert_eq!("part one: 0, part two: 0", solve());
    }
}
