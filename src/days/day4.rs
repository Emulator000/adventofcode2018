use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;

use text_io::Error;

use crate::days::Day;
use crate::input::Input;

#[derive(Debug)]
struct Line {
    id: Option<u32>,
    begin_shift: bool,
    falls_asleep: bool,
    wakes_up: bool,
}

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

    fn solve1(&self) -> i64 {
        let mut logs: IndexMap<i64, Line> = IndexMap::new();
        for row in self.input.get().lines() {
            let (date, action) = row.split_at(19);

            logs.insert(
                date.chars()
                    .filter(|c| *c >= '0' && *c <= '9')
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0),
                match Line::begin_shift(action) {
                    Ok(line) => line,
                    _ => {
                        if let Some(line) = Line::falls_asleep(action) {
                            line
                        } else if let Some(line) = Line::wakes_up(action) {
                            line
                        } else {
                            unreachable!()
                        }
                    }
                },
            );
        }

        logs.sort_keys();

        let mut current_gard = None;
        let mut current_time = 0;

        let mut sleepings = HashMap::new();
        for (time, line) in logs {
            if line.id.is_some() {
                current_gard = line.id;
            }

            if let Some(id) = current_gard {
                if line.falls_asleep {
                    current_time = time;
                } else if line.wakes_up {
                    let sum = time - current_time;

                    let sleeping = sleepings.entry(id).or_insert_with(|| (0, HashSet::new()));
                    sleeping.0 += sum;
                    sleeping.1.insert(current_time..time);
                }
            }
        }

        if let Some((id, (_, ranges))) = sleepings.iter().max_by_key(|(_, (hours, _))| hours) {
            let mut minutes = HashSet::new();
            for range in ranges {
                for time in range.start..range.end {
                    let time = Self::time_to_minutes(time);
                    if !minutes.insert(time) {
                        return *id as i64 * time;
                    }
                }
            }

            0
        } else {
            0
        }
    }

    fn solve2(&self) -> i32 {
        0
    }

    fn time_to_minutes(time: i64) -> i64 {
        format!("{}", time)
            .chars()
            .skip(10)
            .take(2)
            .collect::<String>()
            .parse()
            .unwrap_or(0)
    }
}

impl Line {
    fn begin_shift(input: &str) -> Result<Line, Error> {
        let id: u32;
        let action: String;

        try_scan!(input.bytes() => "Guard #{} {}", id, action);

        Ok(Self {
            id: Some(id),
            begin_shift: true,
            falls_asleep: false,
            wakes_up: false,
        })
    }

    fn falls_asleep(input: &str) -> Option<Line> {
        if input == "falls asleep" {
            Some(Self {
                id: None,
                begin_shift: false,
                falls_asleep: true,
                wakes_up: false,
            })
        } else {
            None
        }
    }

    fn wakes_up(input: &str) -> Option<Line> {
        if input == "wakes up" {
            Some(Self {
                id: None,
                begin_shift: false,
                falls_asleep: false,
                wakes_up: true,
            })
        } else {
            None
        }
    }
}
