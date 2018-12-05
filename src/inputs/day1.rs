use std::cell::{Ref, RefCell};

use inputs::{self, Input};

#[derive(Deserialize)]
pub struct InputDay1 {
    parts: Vec<RefCell<String>>,
}

impl Input for InputDay1 {
    fn get(&self, part: usize) -> Ref<String> {
        self.parts[part].borrow()
    }
}


impl InputDay1 {
    pub fn new() -> Self {
        toml::from_str(&inputs::load_file("inputs/day1.toml")).unwrap()
    }
}
