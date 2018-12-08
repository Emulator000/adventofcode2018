use std::collections::{HashMap, HashSet};

use crate::days::Day;
use crate::input::Input;

pub struct Day4 {
    input: Box<Input>,
}

impl Day for Day4 {
    fn solve(&self, part: usize) -> String {
        match part {
            0 => format!("{}", self.solve1()),
            1 => format!("{}", self.solve2()),
            _ => "".into(),
        }
    }
}

impl Day4 {
    pub fn new() -> Self {
        Self {
            input: Box::new(Input::new(4)),
        }
    }

    fn solve1(&self) -> i32 {
        0
    }

    fn solve2(&self) -> i32 {
        0
    }
}
