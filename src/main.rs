mod day_one;

fn main() {
  let days = vec![day_one::DayOne];
  for day in &days {
    day.solve();
  }
}
