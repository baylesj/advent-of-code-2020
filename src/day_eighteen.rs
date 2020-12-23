use crate::loadable::LoadableFromFile;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
enum Token {
    None,
    Add,
    Multiply,
    Number(i64),
    StartExpression,
    EndExpression,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::None => write!(f, " "),
            Token::Add => write!(f, "+"),
            Token::Multiply => write!(f, "*"),
            Token::Number(i) => write!(f, "{}", i),
            Token::StartExpression => write!(f, "("),
            Token::EndExpression => write!(f, ")"),
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::None
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '(' => Token::StartExpression,
            ')' => Token::EndExpression,
            '*' => Token::Multiply,
            '+' => Token::Add,
            ' ' => Token::None,
            n => Token::Number(n.to_digit(10).unwrap().into()),
        }
    }
}

impl Token {
    fn is_operator(&self) -> bool {
        *self == Token::Add || *self == Token::Multiply
    }

    fn priority(&self) -> i64 {
        match self {
            Token::Add => 1000,
            Token::Multiply => 1,
            // Things that are operators do not really have a priority.
            _ => 0,
        }
    }
}

#[derive(Default, Debug)]
struct Node {
    value: Token,
    children: Vec<Node>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Default)]
struct State {
    last_value: i64,
    operator: Token,
}

fn apply(left: i64, operator: Token, right: i64) -> i64 {
    match operator {
        Token::Add => left + right,
        Token::Multiply => left * right,
        Token::None => right,
        _ => panic!(),
    }
}

fn apply_state(state: &mut State, right: i64) {
    state.last_value = apply(state.last_value, state.operator, right);
    state.operator = Token::None;
}

fn reduce(expression: &str) -> i64 {
    let mut stack = vec![];
    let mut state = State::default();
    for t in expression.as_bytes() {
        let token = Token::from(*t as char);
        if token.is_operator() {
            state.operator = token;
        } else if token == Token::StartExpression {
            stack.push(state);
            state = State::default();
        } else if token == Token::EndExpression {
            let right = state.last_value;
            state = stack.pop().unwrap();
            apply_state(&mut state, right);
        } else if let Token::Number(n) = token {
            apply_state(&mut state, n);
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

fn infix_to_postfix(expression: &str) -> Vec<Token> {
    let mut stack = vec![];
    let mut postfix = vec![];
    for t in expression.as_bytes() {
        let token = Token::from(*t as char);
        if token == Token::None {
            continue;
        }

        if token == Token::StartExpression {
            stack.push(token);
        } else if token == Token::EndExpression {
            while let Some(s) = stack.pop() {
                if s == Token::StartExpression {
                    break;
                }
                postfix.push(s);
            }
        } else if token.is_operator() {
            while !stack.is_empty() && token.priority() <= stack.last().unwrap().priority() {
                assert_ne!(*stack.last().unwrap(), Token::StartExpression);
                postfix.push(stack.pop().unwrap());
            }
            stack.push(token);
        } else {
            assert!(matches!(token, Token::Number { .. }));
            postfix.push(token);
        }
    }

    postfix.extend(stack.iter().rev());
    postfix
}

fn evaluate_postfix(postfix: &[Token]) -> i64 {
    let mut stack = vec![];
    for t in postfix {
        if let Token::Number(n) = t {
            stack.push(*n);
        } else {
            let left = stack.pop().unwrap();
            let right = stack.pop().unwrap();
            if *t == Token::Add {
                stack.push(left + right);
            } else {
                assert_eq!(*t, Token::Multiply);
                stack.push(left * right);
            }
        }
    }
    stack.pop().unwrap()
}

fn reduce_with_priority(expression: &str) -> i64 {
    evaluate_postfix(&infix_to_postfix(&expression))
}

fn part_two(expressions: &[String]) -> i64 {
    let mut sum = 0;
    for e in expressions {
        sum += reduce_with_priority(&e);
    }
    sum
}

pub fn solve() -> String {
    let expressions = Vec::<String>::load("input/day_eighteen.txt");

    format!(
        "part one: {}, part two: {}",
        part_one(&expressions),
        part_two(&expressions)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            "part one: 50956598240016, part two: 535809575344339",
            solve()
        );
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

    #[test]
    fn test_examples_part_two() {
        assert_eq!(51, reduce_with_priority("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(46, reduce_with_priority("2 * 3 + (4 * 5)"));
        assert_eq!(1445, reduce_with_priority("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            669060,
            reduce_with_priority("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            23340,
            reduce_with_priority("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
