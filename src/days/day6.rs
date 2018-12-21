use std::collections::HashSet;

use crate::days::Day;
use crate::input::Input;

pub struct Day6 {
    input: Box<Input>,
}

impl Day for Day6 {
    fn solve(&self, part: usize) -> String {
        match part {
            0 => format!("{}", self.solve1()),
            1 => format!("{}", self.solve2()),
            _ => "".into(),
        }
    }
}

impl Day6 {
    pub fn new() -> Self {
        Self {
            input: Box::new(Input::new(6)),
        }
    }

    fn solve1(&self) -> i32 {
        let coordinates = self.input.get().lines().map(|coords| {
            let coords = coords.split(',').collect::<Vec<&str>>();

            (
                coords.get(0).unwrap_or(&"").trim().parse().unwrap_or(0),
                coords.get(1).unwrap_or(&"").trim().parse().unwrap_or(0)
            )
        }).collect::<HashSet<(i32, i32)>>();

        let maxs = coordinates.iter().fold((0, 0), |mut value, (x, y)| {
            if *x > value.0 {
                value.0 = *x;
            }

            if *y > value.1 {
                value.1 = *y;
            }

            value
        });

//        coordinates.iter().zip(coordinates.iter().skip(1)).map(Self::manhattan_distance).max().unwrap_or(0)

        coordinates.iter().map(|coordinate| {
            (coordinate.0 - maxs.0).abs() + (coordinate.1 - maxs.1).abs()
        }).max().unwrap_or(0)
    }

    fn solve2(&self) -> usize {
        0
    }
}
