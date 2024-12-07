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
            let mut safe = true;
            let mut increasing = true;
            let mut set_increasing = false;

            for pair in row.windows(2) {
                let first = pair.first().expect("This should never happen");
                let second = pair.last().unwrap_or(&-1);

                if second == &-1 {
                    continue;
                }

                if !set_increasing {
                    if first > second {
                        increasing = false;
                    }

                    set_increasing = true;
                }

                let diff = first - second;
                if (increasing && diff > 0)
                    || (!increasing && diff < 0)
                    || diff == 0
                    || diff.abs() > 3
                {
                    safe = false;
                    continue;
                }
            }

            if safe {
                total += 1;
            }
        }

        total
    }

    fn part2(&self, rows: &[Vec<i32>]) -> i32 {
        0
    }
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
