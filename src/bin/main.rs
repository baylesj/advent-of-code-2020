use ansi_term::Color;
use ansi_term::Style;
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
    let mut now = Instant::now();
    day_one::solve();
    now = log_elapsed(&now, 1);
    day_two::solve();
    now = log_elapsed(&now, 2);
    day_three::solve();
    now = log_elapsed(&now, 3);
    day_four::solve();
    now = log_elapsed(&now, 4);
    day_five::solve();
    now = log_elapsed(&now, 5);
    day_six::solve();
    now = log_elapsed(&now, 6);
    day_seven::solve();
    now = log_elapsed(&now, 7);
    day_eight::solve();
    now = log_elapsed(&now, 8);
    day_nine::solve();
    now = log_elapsed(&now, 9);
    day_ten::solve();
    now = log_elapsed(&now, 10);
    day_eleven::solve();
    now = log_elapsed(&now, 11);
    day_twelve::solve();
    now = log_elapsed(&now, 12);
    day_thirteen::solve();
    now = log_elapsed(&now, 13);
    day_fourteen::solve();
    now = log_elapsed(&now, 14);
    day_fifteen::solve();
    now = log_elapsed(&now, 15);
    day_sixteen::solve();
    now = log_elapsed(&now, 16);
}
