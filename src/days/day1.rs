use std::collections::HashSet;

use days::Day;
use inputs::{Input, day1::InputDay1};

pub struct Day1 {
    input: Box<Input>,
    numbers: HashSet<i32>,
}

impl Day for Day1 {
    fn solve(&mut self, part: usize) -> String {
        match part {
            0 => format!("{}", self.solve1()),
            1 => format!("{}", self.solve2()),
            _ => "".into()
        }
    }
}

impl Day1 {
    pub fn new() -> Self {
        Self {
            input: Box::new(InputDay1::new()),
            numbers: HashSet::new(),
        }
    }

    fn solve1(&self) -> i32 {
        self.input.get(0).split("\n").map(|num| {
            match num.parse() {
                Ok(num) => {
                    num
                },
                Err(_) => 0,
            }
        }).sum()
    }

    fn solve2(&mut self) -> i32 {
        let input = self.input.get(0);

        let mut sum = 0;
        'outer: loop {
            for num in input.split("\n") {
                match num.parse() {
                    Ok::<i32, _>(num) => {
                        sum = sum + num;

                        if self.numbers.contains(&sum) {
                            break 'outer;
                        }

                        self.numbers.insert(sum);
                    },
                    Err(_) => {},
                }
            }
        }

        sum
    }
}
