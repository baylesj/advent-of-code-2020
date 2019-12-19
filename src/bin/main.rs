use ansi_term::Color;
use ansi_term::Style;
use std::collections::HashSet;
use std::env;
use std::time::Instant;

use advent_of_code::{
    day_eight, day_eleven, day_fifteen, day_five, day_four, day_fourteen, day_nine, day_one,
    day_seven, day_six, day_sixteen, day_ten, day_thirteen, day_three, day_twelve, day_two,
};

fn log_elapsed(last: &Instant, day: i64) -> Instant {
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
    now
}

// TODO: clean this up, overly procedural.
fn main() {
    let args: HashSet<i64> = env::args().map(|a| a.parse::<i64>().unwrap_or(0)).collect();
    let mut now = Instant::now();
    if args.contains(&1) {
        day_one::solve();
        now = log_elapsed(&now, 1);
    }
    if args.contains(&2) {
        day_two::solve();
        now = log_elapsed(&now, 2);
    }
    if args.contains(&3) {
        day_three::solve();
        now = log_elapsed(&now, 3);
    }
    if args.contains(&4) {
        day_four::solve();
        now = log_elapsed(&now, 4);
    }
    if args.contains(&5) {
        day_five::solve();
        now = log_elapsed(&now, 5);
    }
    if args.contains(&6) {
        day_six::solve();
        now = log_elapsed(&now, 6);
    }
    if args.contains(&7) {
        day_seven::solve();
        now = log_elapsed(&now, 7);
    }
    if args.contains(&8) {
        day_eight::solve();
        now = log_elapsed(&now, 8);
    }
    if args.contains(&9) {
        day_nine::solve();
        now = log_elapsed(&now, 9);
    }
    if args.contains(&10) {
        day_ten::solve();
        now = log_elapsed(&now, 10);
    }
    if args.contains(&11) {
        day_eleven::solve();
        now = log_elapsed(&now, 11);
    }
    if args.contains(&12) {
        day_twelve::solve();
        now = log_elapsed(&now, 12);
    }
    if args.contains(&13) {
        day_thirteen::solve();
        now = log_elapsed(&now, 13);
    }
    if args.contains(&14) {
        day_fourteen::solve();
        now = log_elapsed(&now, 14);
    }
    if args.contains(&15) {
        day_fifteen::solve();
        now = log_elapsed(&now, 15);
    }
    if args.contains(&16) {
        day_sixteen::solve();
        log_elapsed(&now, 16);
    }
}
