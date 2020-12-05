use ansi_term::Color;
use ansi_term::Style;
use std::collections::HashSet;
use std::env;
use std::time::Instant;

use advent_of_code::{
    day_eight, day_eleven, day_fifteen, day_five, day_four, day_fourteen, day_nine, day_one,
    day_seven, day_seventeen, day_six, day_sixteen, day_ten, day_thirteen, day_three, day_twelve,
    day_two,
};

const DAYS: [&dyn Fn() -> String; 17] = [
    &day_one::solve,
    &day_two::solve,
    &day_three::solve,
    &day_four::solve,
    &day_five::solve,
    &day_six::solve,
    &day_seven::solve,
    &day_eight::solve,
    &day_nine::solve,
    &day_ten::solve,
    &day_eleven::solve,
    &day_twelve::solve,
    &day_thirteen::solve,
    &day_fourteen::solve,
    &day_fifteen::solve,
    &day_sixteen::solve,
    &day_seventeen::solve,
];

fn log_elapsed(last: &mut Instant, day: usize) {
    let now = Instant::now();
    let time_elapsed = now.duration_since(*last).as_millis();

    let log_message = format!("\ttime elapsed for day {}:", day);
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
    let day_fragment = style.paint(format!("Day {}", i));
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
