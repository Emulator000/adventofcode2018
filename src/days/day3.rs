use std::collections::{HashMap, HashSet};

use crate::days::Day;
use crate::input::Input;

pub struct Day3 {
    input: Box<Input>,
}

type Claims = HashMap<(i32, i32), (i32, u32)>;
type IDs = HashSet<u32>;
type Result = (Claims, IDs);

impl Day for Day3 {
    fn solve(&self, part: usize) -> String {
        let solves = self.solves();
        match part {
            0 => format!("{}", self.solve1(&solves)),
            1 => format!("{}", self.solve2(&solves)),
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

    fn solves(&self) -> Result {
        let mut claims = Claims::new();
        let mut ids = IDs::new();
        for line in self.input.get().lines() {
            let id: u32;
            let x: i32;
            let y: i32;
            let width: i32;
            let height: i32;

            scan!(line.bytes() => "#{} @ {},{}: {}x{}", id, x, y, width, height);

            for y in y..y + height {
                for x in x..x + width {
                    let claim = claims.entry((x, y)).or_insert_with(|| (0, id));

                    if claim.0 >= 1 {
                        ids.insert(claim.1);
                        ids.insert(id);
                    }

                    claim.0 += 1;
                    claim.1 = id;
                }
            }
        }

        (claims, ids)
    }

    fn solve1(&self, result: &Result) -> usize {
        result
            .0
            .values()
            .filter(|(collisions, _)| *collisions > 1)
            .count()
    }

    fn solve2(&self, result: &Result) -> u32 {
        let ids = result
            .0
            .values()
            .map(|(_, id)| *id)
            .collect::<HashSet<u32>>();
        if let Some(okay) = ids.difference(&result.1).last() {
            *okay
        } else {
            0
        }
    }
}
