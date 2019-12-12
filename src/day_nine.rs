

const INPUT_FILENAME: &str = "input/day_nine.txt"


pub fn part_one() -> i64 {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_one() {
      // Copy of itsef
        //assert_eq!(43210, part_one("input/day_nine_sample_one.txt"));
    }

    #[test]
    fn part_one_sample_two() {
      // a sixteen digit number
        // assert_eq!(
        //     139629729,
        //     part_one("input/day_nine_sample_two.txt")
        // );
    }

    #[test]
    fn part_one_sample_three() {
      assert_eq!(
        1125899906842624,
        part_one("input/day_nine_sample_three.txt")
    );
    }
}