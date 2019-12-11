mod day_eight;
mod day_five;
mod day_four;
mod day_one;
mod day_six;
mod day_three;
mod day_two;

fn main() {
    println!("Day one: {}", day_one::solve());
    println!("Day two: {}", day_two::solve());
    println!("Day three: {}", day_three::solve());
    println!("Day four: {}", day_four::solve());
    println!("Day five: {}", day_five::solve());
    println!("Day six: {}", day_six::solve());
    day_eight::solve();
}
