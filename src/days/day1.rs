use std::cell::Ref;
use std::collections::HashSet;

use crate::days::Day;
use crate::input::Input;

pub struct Day1 {
    input: Box<Input>,
}

impl Day for Day1 {
    fn solve(&self, part: usize) -> String {
        let input = self.input.get();
        match part {
            0 => format!("{}", self.solve1(input)),
            1 => format!("{}", self.solve2(input)),
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

    fn solve1(&self, input: Ref<String>) -> i32 {
        input
            .split("\n")
            .map(|num| match num.parse() {
                Ok(num) => num,
                Err(_) => 0,
            })
            .sum()
    }

    fn solve2(&self, input: Ref<String>) -> i32 {
        let mut numbers = HashSet::new();

        let mut sum = 0;
        input.split("\n").cycle().all(|num| match num.parse() {
            Ok::<i32, _>(num) => numbers.insert({
                sum += num;

                sum
            }),
            Err(_) => true,
        });

        sum
    }
}
