use crate::loadable::LoadableFromFile;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq, Copy, Clone, Debug)]
enum GameState {
    Running,
    PlayerOneWon,
    PlayerTwoWon,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Running
    }
}

#[derive(Default, Clone, Debug)]
struct Game {
    player_one_deck: VecDeque<i32>,
    player_two_deck: VecDeque<i32>,
    state: GameState,
    previous_configurations: HashSet<u64>,
}

// We need to ignore |previous_configurations| for hashing.
impl Hash for Game {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player_one_deck.hash(state);
        self.player_two_deck.hash(state);
    }
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
            game.player_two_deck
                .push_back(l.unwrap().parse::<i32>().unwrap());
        }
        game
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl Game {
    fn find_score(&self) -> i64 {
        let winning_deck = if self.state == GameState::PlayerOneWon {
            &self.player_one_deck
        } else {
            // Scores are not calculated during the game.
            assert_eq!(self.state, GameState::PlayerTwoWon);
            &self.player_two_deck
        };
        // Spot 0 is the top of the deck by convention.
        let mut score = 0;
        for i in 1..winning_deck.len() + 1 {
            score += i as i64 * winning_deck[winning_deck.len() - i] as i64;
        }
        score
    }

    fn play_until_done(&mut self) -> i64 {
        while self.state == GameState::Running {
            let one_played = self.player_one_deck.pop_front().unwrap();
            let two_played = self.player_two_deck.pop_front().unwrap();
            if one_played > two_played {
                // winner takes the cards, with theirs first.
                self.player_one_deck.push_back(one_played);
                self.player_one_deck.push_back(two_played);
            } else {
                self.player_two_deck.push_back(two_played);
                self.player_two_deck.push_back(one_played);
            }

            if self.player_one_deck.is_empty() {
                self.state = GameState::PlayerTwoWon;
            }
            if self.player_two_deck.is_empty() {
                self.state = GameState::PlayerOneWon;
            }
        }
        self.find_score()
    }

    fn play_until_done_with_recursion(&mut self) -> i64 {
        while self.state == GameState::Running {
            let game_hash = calculate_hash(self);

            // Instant win for player one.
            if self.previous_configurations.contains(&game_hash) {
                self.state = GameState::PlayerOneWon;
                break;
            }
            self.previous_configurations.insert(game_hash);

            if self.player_one_deck.is_empty() {
                self.state = GameState::PlayerTwoWon;
                break;
            }
            if self.player_two_deck.is_empty() {
                self.state = GameState::PlayerOneWon;
                break;
            }

            let one_played = self.player_one_deck.pop_front().unwrap();
            let two_played = self.player_two_deck.pop_front().unwrap();
            let round_state;
            // We now determine the winner of this round through a subgame.
            if one_played <= self.player_one_deck.len() as i32
                && two_played <= self.player_two_deck.len() as i32
            {
                let mut recurse_game = Game::default();
                recurse_game.player_one_deck = self.player_one_deck.clone();
                recurse_game.player_one_deck.truncate(one_played as usize);
                recurse_game.player_two_deck = self.player_two_deck.clone();
                recurse_game.player_two_deck.truncate(two_played as usize);
                recurse_game.play_until_done_with_recursion();
                round_state = recurse_game.state;
            } else if one_played > two_played {
                round_state = GameState::PlayerOneWon;
            } else {
                round_state = GameState::PlayerTwoWon;
            }

            if round_state == GameState::PlayerOneWon {
                // winner takes the cards, with theirs first.
                self.player_one_deck.push_back(one_played);
                self.player_one_deck.push_back(two_played);
            // TODO: need to deal with ties??
            } else {
                self.player_two_deck.push_back(two_played);
                self.player_two_deck.push_back(one_played);
            }
            if self.player_one_deck.is_empty() {
                self.state = GameState::PlayerTwoWon;
                break;
            } else if self.player_two_deck.is_empty() {
                self.state = GameState::PlayerOneWon;
                break;
            }
        }
        self.find_score()
    }
}

fn part_one(game: &mut Game) -> i64 {
    game.play_until_done()
}

fn part_two(game: &mut Game) -> i64 {
    game.play_until_done_with_recursion()
}

pub fn solve() -> String {
    let mut game = Game::load("input/day_twenty_two.txt");
    let mut game_two = game.clone();
    format!(
        "part one: {}, part two: {}",
        part_one(&mut game),
        part_two(&mut game_two)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let game = Game::load("input/day_twenty_two_example.txt");
        assert_eq!(VecDeque::from(vec![9, 2, 6, 3, 1]), game.player_one_deck);
        assert_eq!(VecDeque::from(vec![5, 8, 4, 7, 10]), game.player_two_deck);
    }

    #[test]
    fn test_find_score() {
        let game = Game {
            player_one_deck: VecDeque::from(vec![]),
            player_two_deck: VecDeque::from(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]),
            previous_configurations: HashSet::<u64>::default(),
            state: GameState::PlayerTwoWon,
        };
        assert_eq!(306, game.find_score());
    }

    #[test]
    fn test_example() {
        let mut game = Game::load("input/day_twenty_two_example.txt");
        let mut game_two = game.clone();
        assert_eq!(306, part_one(&mut game));
        assert_eq!(291, part_two(&mut game_two));
    }

    #[test]
    fn test_solve() {
        assert_eq!("part one: 35818, part two: 34771", solve());
    }
}
