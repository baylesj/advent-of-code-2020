use std::ops::Range;

type Password = i32;

fn meets_criteria(password: Password) -> bool {
    let s: String = password.to_string();

    let mut it = s.chars();
    let mut last_char: char = it.next().expect("password must be len >= 1");
    let mut has_one_repeating = false;
    while let Some(c) = it.next() {
        if c == last_char {
            has_one_repeating = true;
            // Don't need to set last_char, it hasn't changed.
            continue;
        }
        if c.to_digit(10) < last_char.to_digit(10) {
            return false;
        }
        last_char = c;
    }
    has_one_repeating
}

fn meets_criteria_strict(password: Password) -> bool {
    let s: String = password.to_string();

    let mut it = s.chars();
    let mut last_char: char = it.next().expect("password must be len >= 1");
    let mut has_one_repeating = false;
    let mut duplicate_count: i32 = 0;

    while let Some(c) = it.next() {
        if c == last_char {
            duplicate_count += 1;
            // Don't need to set last_char, it hasn't changed.
            continue;
        } else {
            if duplicate_count == 1 {
                has_one_repeating = true;
            }
            duplicate_count = 0;
        }

        if c.to_digit(10) < last_char.to_digit(10) {
            return false;
        }
        last_char = c;
    }
    has_one_repeating || (duplicate_count == 1)
}

pub fn solve() -> String {
    const PASSWORD_RANGE: Range<Password> = 138307..654505;

    let mut password_count: i64 = 0;
    let mut strict_password_count: i64 = 0;
    for password in PASSWORD_RANGE {
        if meets_criteria(password) {
            password_count += 1;
        }

        if meets_criteria_strict(password) {
            strict_password_count += 1;
        }
    }

    format!(
        "part one: {} passwords, part two: {} passwords",
        password_count, strict_password_count
    )
}
