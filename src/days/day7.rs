use crate::days::day::Day;
use std::collections::VecDeque;

pub struct Day7 {}

impl Day7 {
    fn parse_input(&self, line: &str) -> (f64, VecDeque<u32>) {
        let (total_string, numbers_string) = line.split_once(": ").unwrap();
        let mut numbers = numbers_string
            .split(" ")
            .map(|number| number.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        numbers.reverse();
        (total_string.parse::<f64>().unwrap(), numbers.into())
    }

    fn can_make_total(&self, remaining: f64, mut remaining_numbers: VecDeque<u32>) -> bool {
        if remaining_numbers.len() == 0 {
            remaining == 0.0
        } else {
            let next_value = remaining_numbers.pop_front().unwrap();
            if next_value as f64 > remaining {
                false
            } else {
                self.can_make_total(remaining - next_value as f64, remaining_numbers.clone())
                    || self.can_make_total(remaining / next_value as f64, remaining_numbers.clone())
            }
        }
    }
}

impl Day for Day7 {
    fn part1(&self, input: &str) {
        let result = input
            .lines()
            .map(|line| {
                let (total, numbers) = self.parse_input(line);
                if self.can_make_total(total, numbers.clone()) {
                    total as u64
                } else {
                    0
                }
            })
            .reduce(|total, next| total + next)
            .unwrap();

        println!("{}", result);
    }

    fn part2(&self, input: &str) {}
}
