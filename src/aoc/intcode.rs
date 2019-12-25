use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, VecDeque};

fn mode_arg1(op_code: i64) -> i64 {
    (op_code % 1000) / 100
}

fn mode_arg2(op_code: i64) -> i64 {
    (op_code % 10_000) / 1000
}

fn mode_arg3(op_code: i64) -> i64 {
    (op_code % 100_000) / 10_000
}

#[derive(Debug)]
pub struct Machine {
    pub values: VecDeque<i64>,
    pub ip: usize,
    pub halted: bool,
    pub positions: Vec<i64>,
    pub relative_base: i64,
    pub extended_memory: HashMap<usize, i64>,
}

impl Machine {
    pub fn init(positions: &Vec<i64>) -> Self {
        Machine::new(VecDeque::new(), 0, false, positions.clone(), 0, HashMap::new())
    }

    pub fn from_file(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let program: Vec<i64> = contents
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        Machine::init(&program)
    }

    pub fn new(
        values: VecDeque<i64>,
        ip: usize,
        halted: bool,
        positions: Vec<i64>,
        relative_base: i64,
        extended_memory: HashMap<usize, i64>,
    ) -> Self {
        Self {
            values,
            ip,
            halted,
            positions,
            relative_base,
            extended_memory,
        }
    }

    pub fn run(&mut self, inputs: Vec<i64>) {
        let mut inputs = inputs;
        let mut output = VecDeque::new();

        loop {
            let op_code = self.read(self.ip);

            match op_code % 100 {
                // Add
                1 => {
                    let arg1 = self.lookup(mode_arg1(op_code), 1);
                    let arg2 = self.lookup(mode_arg2(op_code), 2);
                    self.write_param(mode_arg3(op_code), 3, arg1 + arg2);
                    self.ip += 4;
                }

                // Multiply
                2 => {
                    let arg1 = self.lookup(mode_arg1(op_code), 1);
                    let arg2 = self.lookup(mode_arg2(op_code), 2);
                    self.write_param(mode_arg3(op_code), 3, arg1 * arg2);
                    self.ip += 4;
                }

                // Read input
                3 => {
                    if let Some(input) = inputs.pop() {
                        self.write_param(mode_arg1(op_code), 1, input);
                        self.ip += 2;
                    } else {
                        // Yield if no input available
                        break;
                    }
                }

                // Write output
                4 => {
                    let value = self.lookup(mode_arg1(op_code), 1);
                    output.push_back(value);
                    self.ip += 2;
                }

                // Jump if true
                5 => {
                    let arg1 = self.lookup(mode_arg1(op_code), 1);
                    let arg2 = self.lookup(mode_arg2(op_code), 2) as usize;
                    self.ip = if arg1 == 0 { self.ip + 3 } else { arg2 };
                }

                // Jump if false
                6 => {
                    let arg1 = self.lookup(mode_arg1(op_code), 1);
                    let arg2 = self.lookup(mode_arg2(op_code), 2) as usize;
                    self.ip = if arg1 == 0 { arg2 } else { self.ip + 3 };
                }

                // Less than
                7 => {
                    let arg1 = self.lookup(mode_arg1(op_code), 1);
                    let arg2 = self.lookup(mode_arg2(op_code), 2);
                    let value = if arg1 < arg2 { 1 } else { 0 };
                    self.write_param(mode_arg3(op_code), 3, value);
                    self.ip += 4;
                }

                // Equal
                8 => {
                    let arg1 = self.lookup(mode_arg1(op_code), 1);
                    let arg2 = self.lookup(mode_arg2(op_code), 2);
                    let value = if arg1 == arg2 { 1 } else { 0 };
                    self.write_param(mode_arg3(op_code), 3, value);
                    self.ip += 4;
                }

                // Adjust relative base
                9 => {
                    let arg1 = self.lookup(mode_arg1(op_code), 1);
                    self.relative_base += arg1;
                    self.ip += 2;
                }

                // Halt
                99 => {
                    self.halted = true;
                    break;
                }

                x => panic!("Invalid command {}", x),
            }
        }

        self.values = output;
    }

    fn address(&self, mode: i64, arg_number: usize) -> usize {
        match mode {
            // Position
            0 => self.read(self.ip + arg_number as usize) as usize,

            // Value
            1 => self.ip + arg_number,

            // Relative
            2 => {
                let offset = self.read(self.ip + arg_number as usize);
                (self.relative_base + offset) as usize
            }

            _ => panic!("Unknown mode {}", mode),
        }
    }

    fn lookup(&self, mode: i64, arg_number: usize) -> i64 {
        self.read(self.address(mode, arg_number))
    }

    fn write_param(&mut self, mode: i64, arg_number: usize, value: i64) {
        let offset = self.address(mode, arg_number);
        self.write(offset, value)
    }

    fn write(&mut self, offset: usize, value: i64) {
        if offset < self.positions.len() {
            self.positions[offset] = value;
        } else {
            self.extended_memory.insert(offset, value);
        }
    }

    fn read(&self, offset: usize) -> i64 {
        if offset < self.positions.len() {
            self.positions[offset]
        } else {
            *self.extended_memory.get(&offset).unwrap_or(&0)
        }
    }
}
