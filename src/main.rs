mod days;
mod input;

use crate::days::{day1::Day1, day2::Day2, Day};

fn main() {
    let day1: Box<Day> = Box::new(Day1::new());
    println!("Day 1: {}, {}\n", day1.solve(0), day1.solve(1));

    let day2: Box<Day> = Box::new(Day2::new());
    println!("Day 2: {}, {}\n", day2.solve(0), day2.solve(1));
}
