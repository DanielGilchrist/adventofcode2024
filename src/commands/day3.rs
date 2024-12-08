use regex::Regex;

use crate::{commands::command::Command, utils::file::read_input};

pub struct Day3(u8);

// https://adventofcode.com/2024/day/3
impl Command for Day3 {
    fn execute(&self) -> String {
        let input = read_input(3, 1);
        let total = match self.0 {
            1 => self.part1(&input),
            2 => self.part2(&input),
            _ => panic!("Invalid part {}!", self.0),
        };

        format!("{:?}", total)
    }
}

impl Day3 {
    pub fn new(part: u8) -> Self {
        Day3(part)
    }

    fn part1(&self, input: &str) -> i32 {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut total = 0;

        for (_, [x_str, y_str]) in re.captures_iter(input).map(|c| c.extract()) {
            let x: i32 = x_str.parse().unwrap();
            let y: i32 = y_str.parse().unwrap();

            total += x * y;
        }

        total
    }

    fn part2(&self, input: &str) -> i32 {
        0
    }
}
