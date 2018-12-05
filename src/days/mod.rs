pub mod day1;
pub mod day2;

pub trait Day {
    fn solve(&self, part: usize) -> String;
}
