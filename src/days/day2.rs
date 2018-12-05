use days::Day;
use inputs::{day2::InputDay2, Input};

pub struct Day2 {
    input: Box<Input>,
}

impl Day for Day2 {
    fn solve(&self, part: usize) -> String {
        match part {
            0 => format!("{}", self.solve1()),
            1 => format!("{}", self.solve2()),
            _ => "".into(),
        }
    }
}

impl Day2 {
    pub fn new() -> Self {
        Self {
            input: Box::new(InputDay2::new()),
        }
    }

    fn solve1(&self) -> i32 {
        0
    }

    fn solve2(&self) -> i32 {
        0
    }
}
