use std::collections::HashMap;

fn mode_arg1(op_code: i64) -> i64 {
    (op_code % 1000) / 100
}

fn mode_arg2(op_code: i64) -> i64 {
    (op_code % 10_000) / 1000
}

fn mode_arg3(op_code: i64) -> i64 {
    (op_code % 100_000) / 10_000
}

struct Machine {
    values: Vec<i64>,
    ip: usize,
    halted: bool,
    positions: Vec<i64>,
    relative_base: i64,
    extended_memory: HashMap<usize, i64>,
}

impl Machine {
    fn init(positions: &Vec<i64>) -> Self {
        Machine::new(Vec::new(), 0, false, positions.clone(), 0, HashMap::new())
    }

    fn new(
        values: Vec<i64>,
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

    fn run_intcode(&mut self, inputs: Vec<i64>) {
        let mut inputs = inputs;
        let mut output = Vec::new();

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
                    output.push(value);
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

        output.reverse();

        self.values = output;
    }
}

pub fn solve() {
    let positions: Vec<i64> = vec![
        1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1101, 0, 3, 1000, 109,
        988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008, 1000,
        2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104, 0, 99,
        4, 17, 104, 0, 99, 0, 0, 1102, 1, 39, 1013, 1102, 1, 21, 1018, 1101, 0, 336, 1027, 1102, 1,
        38, 1012, 1101, 534, 0, 1025, 1101, 539, 0, 1024, 1101, 0, 380, 1023, 1102, 1, 23, 1014,
        1102, 29, 1, 1000, 1102, 24, 1, 1019, 1102, 1, 28, 1011, 1101, 339, 0, 1026, 1101, 31, 0,
        1005, 1102, 36, 1, 1017, 1102, 26, 1, 1007, 1102, 1, 407, 1028, 1101, 387, 0, 1022, 1101,
        0, 30, 1001, 1101, 34, 0, 1010, 1102, 1, 32, 1006, 1101, 0, 1, 1021, 1102, 27, 1, 1008,
        1102, 22, 1, 1004, 1102, 1, 20, 1015, 1101, 0, 37, 1016, 1101, 0, 0, 1020, 1102, 1, 398,
        1029, 1101, 25, 0, 1009, 1101, 0, 35, 1003, 1101, 33, 0, 1002, 109, 27, 1206, -6, 197,
        1001, 64, 1, 64, 1105, 1, 199, 4, 187, 1002, 64, 2, 64, 109, -22, 2107, 26, 3, 63, 1005,
        63, 217, 4, 205, 1105, 1, 221, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 17, 21107, 40, 39,
        -8, 1005, 1014, 241, 1001, 64, 1, 64, 1105, 1, 243, 4, 227, 1002, 64, 2, 64, 109, -8, 1206,
        6, 261, 4, 249, 1001, 64, 1, 64, 1106, 0, 261, 1002, 64, 2, 64, 109, -7, 2108, 24, 0, 63,
        1005, 63, 281, 1001, 64, 1, 64, 1105, 1, 283, 4, 267, 1002, 64, 2, 64, 109, 11, 21102, 41,
        1, -3, 1008, 1015, 42, 63, 1005, 63, 303, 1105, 1, 309, 4, 289, 1001, 64, 1, 64, 1002, 64,
        2, 64, 109, 1, 1205, 2, 327, 4, 315, 1001, 64, 1, 64, 1105, 1, 327, 1002, 64, 2, 64, 109,
        10, 2106, 0, -2, 1106, 0, 345, 4, 333, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -15, 21102,
        42, 1, 3, 1008, 1017, 42, 63, 1005, 63, 367, 4, 351, 1105, 1, 371, 1001, 64, 1, 64, 1002,
        64, 2, 64, 109, -1, 2105, 1, 10, 1001, 64, 1, 64, 1105, 1, 389, 4, 377, 1002, 64, 2, 64,
        109, 24, 2106, 0, -9, 4, 395, 1001, 64, 1, 64, 1105, 1, 407, 1002, 64, 2, 64, 109, -30,
        1208, -2, 32, 63, 1005, 63, 427, 1001, 64, 1, 64, 1106, 0, 429, 4, 413, 1002, 64, 2, 64,
        109, 2, 1201, 0, 0, 63, 1008, 63, 27, 63, 1005, 63, 449, 1106, 0, 455, 4, 435, 1001, 64, 1,
        64, 1002, 64, 2, 64, 109, 5, 21107, 43, 44, 0, 1005, 1014, 473, 4, 461, 1106, 0, 477, 1001,
        64, 1, 64, 1002, 64, 2, 64, 109, -16, 1202, 3, 1, 63, 1008, 63, 33, 63, 1005, 63, 501,
        1001, 64, 1, 64, 1106, 0, 503, 4, 483, 1002, 64, 2, 64, 109, 10, 1207, -4, 21, 63, 1005,
        63, 523, 1001, 64, 1, 64, 1106, 0, 525, 4, 509, 1002, 64, 2, 64, 109, 11, 2105, 1, 5, 4,
        531, 1106, 0, 543, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -8, 21101, 44, 0, 5, 1008, 1016,
        47, 63, 1005, 63, 563, 1106, 0, 569, 4, 549, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -13,
        2102, 1, 8, 63, 1008, 63, 34, 63, 1005, 63, 593, 1001, 64, 1, 64, 1105, 1, 595, 4, 575,
        1002, 64, 2, 64, 109, 8, 1208, -1, 31, 63, 1005, 63, 617, 4, 601, 1001, 64, 1, 64, 1106, 0,
        617, 1002, 64, 2, 64, 109, -8, 2108, 33, 4, 63, 1005, 63, 635, 4, 623, 1105, 1, 639, 1001,
        64, 1, 64, 1002, 64, 2, 64, 109, 10, 1202, -1, 1, 63, 1008, 63, 26, 63, 1005, 63, 665, 4,
        645, 1001, 64, 1, 64, 1105, 1, 665, 1002, 64, 2, 64, 109, -9, 2107, 30, 1, 63, 1005, 63,
        685, 1001, 64, 1, 64, 1105, 1, 687, 4, 671, 1002, 64, 2, 64, 109, 25, 1205, -4, 703, 1001,
        64, 1, 64, 1105, 1, 705, 4, 693, 1002, 64, 2, 64, 109, -19, 2101, 0, -5, 63, 1008, 63, 26,
        63, 1005, 63, 725, 1105, 1, 731, 4, 711, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 6, 1207,
        -2, 26, 63, 1005, 63, 749, 4, 737, 1105, 1, 753, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
        -10, 21108, 45, 46, 9, 1005, 1010, 769, 1105, 1, 775, 4, 759, 1001, 64, 1, 64, 1002, 64, 2,
        64, 109, -10, 1201, 10, 0, 63, 1008, 63, 30, 63, 1005, 63, 801, 4, 781, 1001, 64, 1, 64,
        1106, 0, 801, 1002, 64, 2, 64, 109, 21, 21108, 46, 46, 3, 1005, 1015, 819, 4, 807, 1106, 0,
        823, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -4, 2102, 1, -3, 63, 1008, 63, 31, 63, 1005,
        63, 849, 4, 829, 1001, 64, 1, 64, 1106, 0, 849, 1002, 64, 2, 64, 109, -5, 2101, 0, 1, 63,
        1008, 63, 22, 63, 1005, 63, 875, 4, 855, 1001, 64, 1, 64, 1105, 1, 875, 1002, 64, 2, 64,
        109, 17, 21101, 47, 0, -3, 1008, 1017, 47, 63, 1005, 63, 897, 4, 881, 1105, 1, 901, 1001,
        64, 1, 64, 4, 64, 99, 21101, 0, 27, 1, 21102, 1, 915, 0, 1105, 1, 922, 21201, 1, 38480, 1,
        204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21101, 0, 942, 0,
        1106, 0, 922, 21202, 1, 1, -1, 21201, -2, -3, 1, 21101, 957, 0, 0, 1105, 1, 922, 22201, 1,
        -1, -2, 1106, 0, 968, 22101, 0, -2, -2, 109, -3, 2105, 1, 0,
    ];

    let mut machine = Machine::init(&positions);
    machine.run_intcode(vec![1]);

    // 3906448201
    println!("Day 9:A = {}", machine.values.pop().unwrap());

    let mut machine = Machine::init(&positions);
    machine.run_intcode(vec![2]);

    // 59785
    println!("Day 9:A = {}", machine.values.pop().unwrap());
}
