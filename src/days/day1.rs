use std::cell::{Ref};

use days::Day;
use inputs::{Input, day1::InputDay1};

pub struct Day1 {
    input: Box<Input>,
}

impl Day for Day1 {
    fn solve(&self, part: usize) -> String {
        let input = self.input.get(0);
        match part {
            0 => format!("{}", self.solve1(input)),
            1 => format!("{}", self.solve2(input)),
            _ => "".into()
        }
    }
}

impl Day1 {
    pub fn new() -> Self {
        Self {
            input: Box::new(InputDay1::new()),
        }
    }

    fn solve1(&self, input: Ref<String>) -> i32 {
        input.split("\n").map(|num| {
            match num.parse() {
                Ok(num) => {
                    num
                },
                Err(_) => 0,
            }
        }).sum()
    }

    fn solve2(&self, input: Ref<String>) -> i32 {
        0
    }
}
