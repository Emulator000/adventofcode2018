use std::cell::{Ref, RefCell};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Input {
    input: RefCell<String>,
}

impl Input {
    pub fn new(day: usize) -> Self {
        Self {
            input: RefCell::new(Self::load_file(format!("inputs/day{}.txt", day))),
        }
    }

    pub fn get(&self) -> Ref<String> {
        self.input.borrow()
    }

    fn load_file<S: AsRef<str>>(path: S) -> String {
        let path = Path::new(path.as_ref());
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        let mut configuration = String::new();
        match file.read_to_string(&mut configuration) {
            Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
            Ok(_) => print!("{} loaded correctly.\n", display),
        }

        configuration
    }
}
