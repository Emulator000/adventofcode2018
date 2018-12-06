use std::cell::Ref;

use crate::days::Day;
use crate::input::{Input};

pub struct Day2 {
    input: Box<Input>,
}

impl Day for Day2 {
    fn solve(&self, part: usize) -> String {
        let input = self.input.get();
        match part {
            0 => format!("{}", self.solve1(input)),
            1 => format!("{}", self.solve2(input)),
            _ => "".into(),
        }
    }
}

impl Day2 {
    pub fn new() -> Self {
        Self {
            input: Box::new(Input::new(2)),
        }
    }

    fn solve1(&self, input: Ref<String>) -> i32 {
        0
    }

    fn solve2(&self, input: Ref<String>) -> i32 {
        0
    }
}
