use std::collections::HashSet;

use days::Day;
use inputs::{day1::InputDay1, Input};

pub struct Day1 {
    input: Box<Input>,
}

impl Day for Day1 {
    fn solve(&self, part: usize) -> String {
        match part {
            0 => format!("{}", self.solve1()),
            1 => format!("{}", self.solve2()),
            _ => "".into(),
        }
    }
}

impl Day1 {
    pub fn new() -> Self {
        Self {
            input: Box::new(InputDay1::new()),
        }
    }

    fn solve1(&self) -> i32 {
        self.input
            .get(0)
            .split("\n")
            .map(|num| match num.parse() {
                Ok(num) => num,
                Err(_) => 0,
            })
            .sum()
    }

    fn solve2(&self) -> i32 {
        let mut numbers = HashSet::new();

        let mut sum = 0;
        self.input
            .get(0)
            .split("\n")
            .cycle()
            .all(|num| match num.parse() {
                Ok::<i32, _>(num) => numbers.insert({
                    sum = sum + num;

                    sum
                }),
                Err(_) => false,
            });

        sum
    }
}
