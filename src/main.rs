#![feature(test)]
#![feature(range_contains)]
extern crate test;
#[macro_use]
extern crate text_io;

mod days;
mod input;

use crate::days::{day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5, day6::Day6, Day};

fn main() {
    let day1: Box<Day> = Box::new(Day1::new());
    println!("Day 1: {}, {}\n", day1.solve(0), day1.solve(1));

    let day2: Box<Day> = Box::new(Day2::new());
    println!("Day 2: {}, {}\n", day2.solve(0), day2.solve(1));

    let day3: Box<Day> = Box::new(Day3::new());
    println!("Day 3: {}, {}\n", day3.solve(0), day3.solve(1));

    let day4: Box<Day> = Box::new(Day4::new());
    println!("Day 4: {}, {}\n", day4.solve(0), day4.solve(1));

    let day5: Box<Day> = Box::new(Day5::new());
    println!("Day 5: {}, {}\n", day5.solve(0), day5.solve(1));

    let day6: Box<Day> = Box::new(Day6::new());
    println!("Day 6: {}, {}\n", day6.solve(0), day6.solve(1));
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[bench]
    fn day1_1(bencher: &mut Bencher) {
        let day = Box::new(Day1::new());
        bencher.iter(|| day.solve(0));
    }

    #[bench]
    fn day1_2(bencher: &mut Bencher) {
        let day = Box::new(Day1::new());
        bencher.iter(|| day.solve(1));
    }

    #[bench]
    fn day2_1(bencher: &mut Bencher) {
        let day = Box::new(Day2::new());
        bencher.iter(|| day.solve(0));
    }

    #[bench]
    fn day2_2(bencher: &mut Bencher) {
        let day = Box::new(Day2::new());
        bencher.iter(|| day.solve(1));
    }

    #[bench]
    fn day3_1(bencher: &mut Bencher) {
        let day = Box::new(Day3::new());
        bencher.iter(|| day.solve(0));
    }

    #[bench]
    fn day3_2(bencher: &mut Bencher) {
        let day = Box::new(Day3::new());
        bencher.iter(|| day.solve(1));
    }

    #[bench]
    fn day4_1(bencher: &mut Bencher) {
        let day = Box::new(Day4::new());
        bencher.iter(|| day.solve(0));
    }

    #[bench]
    fn day4_2(bencher: &mut Bencher) {
        let day = Box::new(Day4::new());
        bencher.iter(|| day.solve(1));
    }

    #[bench]
    fn day5_1(bencher: &mut Bencher) {
        let day = Box::new(Day5::new());
        bencher.iter(|| day.solve(0));
    }

    #[bench]
    fn day5_2(bencher: &mut Bencher) {
        let day = Box::new(Day5::new());
        bencher.iter(|| day.solve(1));
    }

    #[bench]
    fn day6_1(bencher: &mut Bencher) {
        let day = Box::new(Day6::new());
        bencher.iter(|| day.solve(0));
    }

    #[bench]
    fn day6_2(bencher: &mut Bencher) {
        let day = Box::new(Day6::new());
        bencher.iter(|| day.solve(1));
    }
}
