use crate::{commands::command::Command, utils::file::read_input};

pub struct Day2(u8);

// https://adventofcode.com/2024/day/2
impl Command for Day2 {
    fn execute(&self) -> String {
        let input = read_input(2, 1);
        let rows = split_input(&input);

        let total = match self.0 {
            1 => self.part1(&rows),
            2 => self.part2(&rows),
            _ => panic!("Invalid part {}!", self.0),
        };

        format!("Total is {total}")
    }
}

impl Day2 {
    pub fn new(part: u8) -> Self {
        Day2(part)
    }

    fn part1(&self, rows: &[Vec<i32>]) -> i32 {
        let mut total = 0;

        for row in rows.iter() {
            let increasing = is_increasing(row);
            let index = unsafe_index(row, increasing);

            if index == -1 {
                total += 1
            }
        }

        total
    }

    fn part2(&self, rows: &[Vec<i32>]) -> i32 {
        0
    }
}

fn is_increasing(slice: &[i32]) -> bool {
    let first = slice[0];
    let second = slice[1];

    first <= second
}

fn unsafe_index(row: &[i32], increasing: bool) -> i32 {
    for (i, n) in row.iter().enumerate() {
        let first = n;
        let second = match row.get(i + 1) {
            Some(second) => second,
            None => return -1,
        };

        let diff = first - second;
        if (increasing && diff > 0) || (!increasing && diff < 0) || diff == 0 || diff.abs() > 3 {
            return i as i32 + 1;
        }
    }

    -1
}

fn split_input(input: &str) -> Vec<Vec<i32>> {
    let mut rows: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let row = line
            .split_whitespace()
            .map(|c| c.parse::<i32>().expect("Invalid number"))
            .collect::<_>();

        rows.push(row);
    }

    rows
}
