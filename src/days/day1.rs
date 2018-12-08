use std::collections::HashSet;

use crate::days::Day;
use crate::input::Input;

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
            input: Box::new(Input::new(1)),
        }
    }

    fn solve1(&self) -> i32 {
        self.input
            .get()
            .lines()
            .map(|num| num.parse().unwrap_or(0))
            .sum()
    }

    fn solve2(&self) -> i32 {
        let mut numbers = HashSet::new();

        let mut sum = 0;
        self.input
            .get()
            .lines()
            .cycle()
            .all(|num| numbers.insert({
                sum += num.parse().unwrap_or(0);

                sum
            }));

        sum
    }
}
