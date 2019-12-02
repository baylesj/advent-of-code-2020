mod day;
mod day_one;

fn main() {
  let days: [day::Day; 1] = [day_one::DayOne::new()];
  for d in &days {
    println!("{}", d.solve());
  }
}
