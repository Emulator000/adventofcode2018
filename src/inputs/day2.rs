use std::cell::{Ref, RefCell};

use inputs::{self, Input};

#[derive(Deserialize)]
pub struct InputDay2 {
    parts: Vec<RefCell<String>>,
}

impl Input for InputDay2 {
    fn get(&self, part: usize) -> Ref<String> {
        self.parts[part].borrow()
    }
}

impl InputDay2 {
    pub fn new() -> Self {
        toml::from_str(&inputs::load_file("inputs/day2.toml")).unwrap()
    }
}
