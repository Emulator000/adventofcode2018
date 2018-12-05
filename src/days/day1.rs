use days::Day;
use inputs::{Input, day1::InputDay1};

pub struct Day1 {
    input: Box<Input>,
}

impl Day for Day1 {
    fn solve(&self) {
        self.input.get();
    }
}

impl Day1 {
    pub fn new() -> Self {
        Self {
            input: Box::new(InputDay1::new()),
        }
    }
}
