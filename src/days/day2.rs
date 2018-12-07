use std::cell::Ref;
use std::collections::{HashMap};

use crate::days::Day;
use crate::input::Input;

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
        let final_count = input
            .lines()
            .map(|line| {
                line.chars()
                    .fold(HashMap::new(), |mut chars, c| {
                        let sum = chars.entry(c).or_insert_with(|| 0);
                        *sum += 1;

                        chars
                    })
                    .values()
                    .fold((false, false), |counts, sum| {
                        (counts.0 || *sum == 2, counts.1 || *sum == 3)
                    })
            })
            .fold((0, 0), |sum, count| {
                (sum.0 + count.0 as i32, sum.1 + count.1 as i32)
            });

        final_count.0 * final_count.1
    }

    fn solve2(&self, input: Ref<String>) -> String {
        let mut best = 0;
        let mut result = String::new();
        for line in input.lines() {
            for line2 in input.lines() {
                if line == line2 {
                    continue;
                }

                let mut diff = 0;
                let same = line
                    .chars()
                    .zip(line2.chars())
                    .filter_map(|(c, c2)| {
                        if c != c2 {
                            diff += 1;

                            None
                        } else {
                            Some(c)
                        }
                    })
                    .collect::<String>();

                if best == 0 || diff < best {
                    best = diff;
                    result = same;
                }
            }
        }

        result
    }
}
