use crate::Part;

pub trait Day {
    fn part1(&self);
    fn part2(&self);

    fn run(&self, part: &Part) {
        match part {
            Part::P1 => self.part1(),
            Part::P2 => self.part2(),
            Part::Both => {
                self.part1();
                self.part2();
            }
        }
    }
}
