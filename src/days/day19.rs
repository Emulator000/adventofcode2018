use crate::days::Day;
use crate::input::Input;

pub struct Day19 {
    input: Box<Input>,
}

impl Day for Day19 {
    fn solve(&self, part: usize) -> String {
        match part {
            0 => format!("{}", self.solve1()),
            1 => format!("{}", self.solve2()),
            _ => "".into(),
        }
    }
}

impl Day19 {
    const MAX_REGISTER: usize = 5;

    pub fn new() -> Self {
        Self {
            input: Box::new(Input::new(19)),
        }
    }

    fn solve1(&self) -> usize {
        let lines = self.input.get();
        let mut lines = lines.lines();
        let mut ip: usize = lines.next().expect("ip not found!").replace("#ip ", "").parse().unwrap();

        let mut registers: [usize; Self::MAX_REGISTER + 1] = [0; Self::MAX_REGISTER + 1];
        for line in lines {
            let operation: String;
            let value: usize;
            let pointer: usize;
            let address: usize;

            scan!(line.bytes() => "{} {} {} {}", operation, value, pointer, address);

            if address > Self::MAX_REGISTER || pointer > Self::MAX_REGISTER {
                continue;
            }

            match operation.as_str() {
                "addr" => {

                }
                "addi" => {

                }
                "mulr" => {

                }
                "muli" => {

                }
                "banr" => {

                }
                "bani" => {

                }
                "borr" => {

                }
                "bori" => {

                }
                "setr" => {
                    registers[address] = value;
                    registers[ip] += pointer;
                }
                "seti" => {
                    registers[address] = value;
                    registers[ip] += pointer;

                    ip += 1;
                }
                "gtir" => {

                }
                "gtri" => {

                }
                "gtrr" => {

                }
                "eqir" => {

                }
                "eqri" => {

                }
                "eqrr" => {

                }
                _ => {}
             }

            println!("{:?}", registers);
        }

        0
    }

    fn solve2(&self) -> usize {
        0
    }
}
