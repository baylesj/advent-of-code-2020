use crate::loadable::LoadableFromFile;

#[derive(Clone, Copy)]
enum Operator {
    None,
    Add,
    Multiply,
    Subtract,
    Divide,
}

impl Default for Operator {
    fn default() -> Self {
        Operator::None
    }
}

#[derive(Default)]
struct State {
    last_value: i64,
    operator: Operator,
}

fn apply(left: i64, operator: Operator, right: i64) -> i64 {
    match operator {
        Operator::Add => left + right,
        Operator::Multiply => left * right,
        Operator::Subtract => left - right,
        Operator::Divide => left / right,
        Operator::None => right,
    }
}

fn apply_state(state: &mut State, right: i64) {
    state.last_value = apply(state.last_value, state.operator, right);
    state.operator = Operator::None;
}

fn reduce(expression: &str) -> i64 {
    let mut stack = vec![];
    let mut state = State::default();
    for t in expression.as_bytes() {
        match *t as char {
            '(' => {
                stack.push(state);
                state = State::default();
            }
            ')' => {
                let right = state.last_value;
                state = stack.pop().unwrap();
                apply_state(&mut state, right);
            }
            '*' => state.operator = Operator::Multiply,
            '+' => state.operator = Operator::Add,
            '-' => state.operator = Operator::Subtract,
            '/' => state.operator = Operator::Divide,
            ' ' => (),
            n => apply_state(&mut state, n.to_digit(10).unwrap().into()),
        }
    }
    state.last_value
}

fn part_one(expressions: &[String]) -> i64 {
    let mut sum = 0;
    for e in expressions {
        sum += reduce(&e);
    }
    sum
}

fn part_two() -> i64 {
    0
}

pub fn solve() -> String {
    let expressions = Vec::<String>::load("input/day_eighteen.txt");

    format!(
        "part one: {}, part two: {}",
        part_one(&expressions),
        part_two()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 50956598240016, part two: 0", solve());
    }

    #[test]
    fn test_examples() {
        assert_eq!(71, reduce("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(51, reduce("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(26, reduce("2 * 3 + (4 * 5)"));
        assert_eq!(437, reduce("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(12240, reduce("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(
            13632,
            reduce("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
