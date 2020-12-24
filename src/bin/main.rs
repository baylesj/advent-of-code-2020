use ansi_term::Color;
use ansi_term::Style;
use std::collections::HashSet;
use std::env;
use std::time::Instant;

use advent_of_code;

const DAYS: [&dyn Fn() -> String; 25] = [
    &advent_of_code::day_one::solve,
    &advent_of_code::day_two::solve,
    &advent_of_code::day_three::solve,
    &advent_of_code::day_four::solve,
    &advent_of_code::day_five::solve,
    &advent_of_code::day_six::solve,
    &advent_of_code::day_seven::solve,
    &advent_of_code::day_eight::solve,
    &advent_of_code::day_nine::solve,
    &advent_of_code::day_ten::solve,
    &advent_of_code::day_eleven::solve,
    &advent_of_code::day_twelve::solve,
    &advent_of_code::day_thirteen::solve,
    &advent_of_code::day_fourteen::solve,
    &advent_of_code::day_fifteen::solve,
    &advent_of_code::day_sixteen::solve,
    &advent_of_code::day_seventeen::solve,
    &advent_of_code::day_eighteen::solve,
    &advent_of_code::day_nineteen::solve,
    &advent_of_code::day_twenty::solve,
    &advent_of_code::day_twenty_one::solve,
    &advent_of_code::day_twenty_two::solve,
    &advent_of_code::day_twenty_three::solve,
    &advent_of_code::christmas_eve::solve,
    &advent_of_code::christmas_day::solve,
];

fn log_elapsed(last: &mut Instant, day: usize) {
    let now = Instant::now();
    let time_elapsed = now.duration_since(*last).as_millis();

    let log_message;
    if day == 24 {
        log_message = "\ttime elapsed for christmas eve:".to_owned();
    } else if day == 25 {
        log_message = "\ttime elapsed for christmas day:".to_owned();
    } else {
        log_message = format!("\ttime elapsed for day {}:", day);
    }
    let log_time = format!("{}ms", time_elapsed);

    let color: Color;
    if time_elapsed < 150 {
        color = Color::Green;
    } else if time_elapsed < 500 {
        color = Color::Yellow;
    } else {
        color = Color::Red;
    }
    let style = Style::new().italic();
    println!("{} {}", style.paint(log_message), color.paint(log_time));
    *last = now;
}

fn run_day(i: usize, now: &mut Instant) {
    let style = Style::new().bold();

    let day_name;
    if i == 24 {
        day_name = "Christmas Eve".to_owned();
    } else if i == 25 {
        day_name = "Christmas Day".to_owned();
    } else {
        day_name = format!("Day {}", i);
    }
    let day_fragment = style.paint(day_name);
    println!("{}: {}", day_fragment, DAYS[i - 1]());
    log_elapsed(now, i);
}

fn main() {
    let mut run_all = false;
    let args: HashSet<i64> = env::args()
        .map(|a| {
            if a == "-a" {
                run_all = true;
            }
            a.parse::<i64>().unwrap_or(0)
        })
        .filter(|a| *a > 0)
        .collect();

    let mut now = Instant::now();
    if run_all {
        for i in 0..DAYS.len() {
            run_day(i + 1, &mut now);
        }
    } else {
        for arg in args.iter() {
            run_day(*arg as usize, &mut now);
        }
    }
}
