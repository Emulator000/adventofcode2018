extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod inputs;
mod days;

use days::{Day, day1::Day1};

fn main() {
    let day1: Box<Day> = Box::new(Day1::new());
    day1.solve();
}
