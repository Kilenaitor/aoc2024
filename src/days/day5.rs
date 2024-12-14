use crate::days::day::Day;
use std::collections::{HashMap, VecDeque};

type Rules<'a> = HashMap<&'a str, Vec<&'a str>>;

pub struct Day5 {}

impl Day5 {
    fn valid_middle_number(&self, pages: &Vec<&str>, precedence_rules: &Rules) -> Option<u16> {
        let mut pages_to_add: VecDeque<&str> = VecDeque::new();
        for page in pages {
            pages_to_add.push_back(page);
        }

        loop {
            let page_to_add = pages_to_add.pop_front().unwrap();

            if let Some(preceding_pages) = precedence_rules.get(page_to_add) {
                for preceding_page in preceding_pages {
                    if pages_to_add.contains(preceding_page) {
                        return None;
                    }
                }
            }

            if pages_to_add.len() == 0 {
                break;
            }
        }

        Some(pages[pages.len() / 2].parse::<u16>().unwrap())
    }

    fn repair(&self, pages: &Vec<&str>, precedence_rules: &Rules) -> u16 {
        let mut pages_to_add: VecDeque<&str> = VecDeque::new();
        for page in pages {
            pages_to_add.push_back(page);
        }

        let mut corrected_update: Vec<&str> = Vec::new();

        loop {
            let (index, page_to_add) = pages_to_add
                .iter()
                .enumerate()
                .find(|&page_with_iter| {
                    let (_, page) = page_with_iter;
                    if let Some(preceding_pages) = precedence_rules.get(page) {
                        for preceding_page in preceding_pages {
                            if pages_to_add.contains(preceding_page) {
                                return false;
                            }
                        }
                        true
                    } else {
                        true
                    }
                })
                .unwrap();

            corrected_update.push(page_to_add);
            pages_to_add.remove(index);

            if pages_to_add.len() == 0 {
                break;
            }
        }

        corrected_update[corrected_update.len() / 2]
            .parse::<u16>()
            .unwrap()
    }
}

impl Day for Day5 {
    fn part1(&self, input: &str) {
        let (rules, updates) = input.split_once("\n\n").unwrap();

        let mut precedence_rules = HashMap::new();
        for rule in rules.lines() {
            let (first, second) = rule.split_once("|").unwrap();
            precedence_rules
                .entry(second)
                .and_modify(|pages: &mut Vec<&str>| pages.push(first))
                .or_insert(vec![first]);
        }

        let result: u16 = updates
            .lines()
            .map(|update| update.split(",").collect())
            .filter_map(|update| self.valid_middle_number(&update, &precedence_rules))
            .reduce(|total, number| total + number)
            .unwrap();

        println!("{}", result);
    }

    fn part2(&self, input: &str) {
        let (rules, updates) = input.split_once("\n\n").unwrap();

        let mut precedence_rules = HashMap::new();
        for rule in rules.lines() {
            let (first, second) = rule.split_once("|").unwrap();
            precedence_rules
                .entry(second)
                .and_modify(|pages: &mut Vec<&str>| pages.push(first))
                .or_insert(vec![first]);
        }

        let result = updates
            .lines()
            .map(|update| update.split(",").collect())
            .filter(|update| {
                self.valid_middle_number(&update, &precedence_rules)
                    .is_none()
            })
            .map(|update| self.repair(&update, &precedence_rules))
            .reduce(|total, number| total + number)
            .unwrap();

        println!("{}", result);
    }
}
