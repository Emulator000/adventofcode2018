use crate::days::Day;
use crate::input::Input;

pub struct Day5 {
    input: Box<Input>,
}

impl Day for Day5 {
    fn solve(&self, part: usize) -> String {
        match part {
            0 => format!("{}", self.solve1()),
            1 => format!("{}", self.solve2()),
            _ => "".into(),
        }
    }
}

impl Day5 {
    pub fn new() -> Self {
        Self {
            input: Box::new(Input::new(5)),
        }
    }

    fn solve1(&self) -> usize {
        self.input
            .get()
            .chars()
            .fold(Vec::new(), |mut cs, c| {
                match if let Some(c2) = cs.last() {
                    c != *c2 && c.eq_ignore_ascii_case(c2)
                } else {
                    false
                } {
                    true => {
                        cs.pop();
                    }
                    false => {
                        cs.push(c);
                    }
                }

                cs
            })
            .iter()
            .count()
    }

    fn solve2(&self) -> usize {
        0
    }
}
