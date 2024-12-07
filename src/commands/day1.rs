use crate::{commands::command::Command, utils::file::read_input};

pub struct Day1(u8);

// https://adventofcode.com/2024/day/1
impl Command for Day1 {
    fn execute(&self) -> String {
        let input = read_input(1, 1);
        let (mut left, mut right) = split_input(&input);

        let total = match self.0 {
            1 => self.part1(&mut left, &mut right),
            2 => self.part2(&left, &right),
            _ => panic!("Invalid part {}!", self.0),
        };

        format!("Total is {total}")
    }
}

impl Day1 {
    pub fn new(part: u8) -> Self {
        Day1(part)
    }

    fn part1(&self, left: &mut [i32], right: &mut [i32]) -> i32 {
        left.sort();
        right.sort();

        let mut total = 0;

        for (i, n) in left.iter().enumerate() {
            let right_n = right[i];
            total += (n - right_n).abs();
        }

        total
    }

    fn part2(&self, left: &[i32], right: &[i32]) -> i32 {
        let mut total = 0;

        for n in left.iter() {
            total += n * right.iter().filter(|&right_n| right_n == n).count() as i32;
        }

        total
    }
}

fn split_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let left_value: i32 = parts[0].parse().expect("Invalid number on the left");
            let right_value: i32 = parts[1].parse().expect("Invalid number on the right");

            left.push(left_value);
            right.push(right_value);
        }
    }

    (left, right)
}
