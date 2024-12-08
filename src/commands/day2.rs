use std::cmp::Ordering;

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
        let mut total = 0;

        for row in rows.iter() {
            let mut is_safe = false;

            let increasing = is_increasing(row);
            if unsafe_index(row, increasing) == -1 {
                total += 1;
                continue;
            }

            for i in 0..row.len() {
                let unsafe_removed = remove_at_index(row, i);
                let new_increasing = is_increasing(&unsafe_removed);
                let new_index = unsafe_index(&unsafe_removed, new_increasing);

                if new_index == -1 {
                    total += 1;
                    is_safe = true;
                    break;
                }
            }

            if is_safe {
                continue;
            }
        }

        total
    }
}

fn remove_at_index(vec: &[i32], index: usize) -> Vec<i32> {
    vec.iter()
        .enumerate()
        .filter(|&(i, _)| i != index)
        .map(|(_, &value)| value)
        .collect()
}

fn is_increasing(slice: &[i32]) -> bool {
    let mut increasing_count = 0;
    let mut decreasing_count = 0;

    for window in slice.windows(2) {
        match window[0].cmp(&window[1]) {
            Ordering::Less => increasing_count += 1,
            Ordering::Greater => decreasing_count += 1,
            Ordering::Equal => (),
        }
    }

    increasing_count > decreasing_count
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
