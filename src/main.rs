#![feature(test)]
#![feature(range_contains)]
extern crate test;
#[macro_use]
extern crate text_io;

mod days;
mod input;

use crate::days::{day1::Day1, day2::Day2, day3::Day3, Day};

fn main() {
    let day1: Box<Day> = Box::new(Day1::new());
    println!("Day 1: {}, {}\n", day1.solve(0), day1.solve(1));

    let day2: Box<Day> = Box::new(Day2::new());
    println!("Day 2: {}, {}\n", day2.solve(0), day2.solve(1));

    let day3: Box<Day> = Box::new(Day3::new());
    println!("Day 3: {}, {}\n", day3.solve(0), day3.solve(1));
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::days::{day1::Day1, day2::Day2, day3::Day3, Day};

    fn day1() -> Box<Day> {
        Box::new(Day1::new())
    }

    fn day2() -> Box<Day> {
        Box::new(Day2::new())
    }

    fn day3() -> Box<Day> {
        Box::new(Day3::new())
    }

    #[bench]
    fn day1_1(bencher: &mut Bencher) {
        let day = day1();
        bencher.iter(|| day.solve(0));
    }

    #[bench]
    fn day1_2(bencher: &mut Bencher) {
        let day = day1();
        bencher.iter(|| day.solve(1));
    }

    #[bench]
    fn day2_1(bencher: &mut Bencher) {
        let day = day2();
        bencher.iter(|| day.solve(0));
    }

    #[bench]
    fn day2_2(bencher: &mut Bencher) {
        let day = day2();
        bencher.iter(|| day.solve(1));
    }

    #[bench]
    fn day3_1(bencher: &mut Bencher) {
        let day = day3();
        bencher.iter(|| day.solve(0));
    }

    #[bench]
    fn day3_2(bencher: &mut Bencher) {
        let day = day3();
        bencher.iter(|| day.solve(1));
    }
}
