use crate::Part;
use std::fs;

pub trait Day {
    fn part1(&self, input: &str);
    fn part2(&self, input: &str);

    fn run(&self, part: &Part, is_test: bool) {
        let filename = if is_test { "test.txt" } else { "input.txt" };
        let input = fs::read_to_string(filename).expect("Should be able to read input");

        match part {
            Part::P1 => self.part1(&input),
            Part::P2 => self.part2(&input),
            Part::Both => {
                self.part1(&input);
                self.part2(&input);
            }
        }
    }
}
