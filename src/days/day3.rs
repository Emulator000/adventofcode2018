use std::collections::HashMap;

use crate::days::Day;
use crate::input::Input;

pub struct Day3 {
    input: Box<Input>,
}

impl Day for Day3 {
    fn solve(&self, part: usize) -> String {
        match part {
            0 => format!("{}", self.solve1()),
            1 => format!("{}", self.solve2()),
            _ => "".into(),
        }
    }
}

impl Day3 {
    pub fn new() -> Self {
        Self {
            input: Box::new(Input::new(3)),
        }
    }

    fn solve1(&self) -> i32 {
        0
    }

    fn solve2(&self) -> i32 {
        0
    }
}
