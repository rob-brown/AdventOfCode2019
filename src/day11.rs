use core::cmp::max;
use core::cmp::min;
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

enum Color {
    Black,
    White,
}

impl Color {
    fn to_int(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    fn from_int(int: i64) -> Self {
        match int {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!(),
        }
    }
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn run(positions: &Vec<i64>, start_color: Color) -> HashMap<(i32, i32), Color> {
    let mut panels: HashMap<(i32, i32), Color> = HashMap::new();
    let mut machine = Machine::init(&positions);
    let mut current = (0, 0);
    let mut direction = Direction::Up;

    panels.insert((0, 0), start_color);

    while machine.halted == false {
        let color = panels.get(&current).unwrap_or(&Color::Black);

        machine.run_intcode(vec![color.to_int()]);

        let new_color = machine.values.pop().unwrap();
        let turn = machine.values.pop().unwrap();

        let (dir, point) = match (direction, turn) {
            (Direction::Up, 0) => (Direction::Left, (current.0 - 1, current.1)),
            (Direction::Up, 1) => (Direction::Right, (current.0 + 1, current.1)),
            (Direction::Left, 0) => (Direction::Down, (current.0, current.1 + 1)),
            (Direction::Left, 1) => (Direction::Up, (current.0, current.1 - 1)),
            (Direction::Down, 0) => (Direction::Right, (current.0 + 1, current.1)),
            (Direction::Down, 1) => (Direction::Left, (current.0 - 1, current.1)),
            (Direction::Right, 0) => (Direction::Up, (current.0, current.1 - 1)),
            (Direction::Right, 1) => (Direction::Down, (current.0, current.1 + 1)),
            _ => panic!(),
        };

        panels.insert(current, Color::from_int(new_color));

        direction = dir;
        current = point;
    }

    panels
}

pub fn solve() {
    let positions: Vec<i64> = vec![
        3, 8, 1005, 8, 337, 1106, 0, 11, 0, 0, 0, 104, 1, 104, 0, 3, 8, 1002, 8, -1, 10, 101, 1,
        10, 10, 4, 10, 108, 1, 8, 10, 4, 10, 1002, 8, 1, 28, 2, 1, 15, 10, 2, 2, 10, 10, 1, 1107,
        0, 10, 2, 1105, 18, 10, 3, 8, 102, -1, 8, 10, 1001, 10, 1, 10, 4, 10, 1008, 8, 1, 10, 4,
        10, 101, 0, 8, 67, 1, 1003, 4, 10, 2, 1007, 14, 10, 1006, 0, 64, 3, 8, 102, -1, 8, 10,
        1001, 10, 1, 10, 4, 10, 1008, 8, 0, 10, 4, 10, 102, 1, 8, 100, 2, 102, 15, 10, 3, 8, 1002,
        8, -1, 10, 1001, 10, 1, 10, 4, 10, 108, 0, 8, 10, 4, 10, 1001, 8, 0, 125, 2, 1003, 7, 10,
        1006, 0, 10, 2, 1007, 13, 10, 2, 103, 14, 10, 3, 8, 102, -1, 8, 10, 1001, 10, 1, 10, 4, 10,
        1008, 8, 1, 10, 4, 10, 101, 0, 8, 163, 1006, 0, 5, 3, 8, 1002, 8, -1, 10, 1001, 10, 1, 10,
        4, 10, 1008, 8, 0, 10, 4, 10, 102, 1, 8, 188, 1, 1101, 2, 10, 1006, 0, 82, 3, 8, 1002, 8,
        -1, 10, 101, 1, 10, 10, 4, 10, 1008, 8, 0, 10, 4, 10, 101, 0, 8, 217, 1, 1109, 1, 10, 1,
        109, 9, 10, 1, 1009, 9, 10, 1006, 0, 41, 3, 8, 102, -1, 8, 10, 1001, 10, 1, 10, 4, 10,
        1008, 8, 1, 10, 4, 10, 102, 1, 8, 254, 2, 104, 1, 10, 2, 8, 15, 10, 3, 8, 1002, 8, -1, 10,
        1001, 10, 1, 10, 4, 10, 1008, 8, 1, 10, 4, 10, 101, 0, 8, 284, 1, 1107, 11, 10, 3, 8, 102,
        -1, 8, 10, 101, 1, 10, 10, 4, 10, 108, 0, 8, 10, 4, 10, 101, 0, 8, 309, 2, 1001, 10, 10,
        1006, 0, 49, 101, 1, 9, 9, 1007, 9, 1058, 10, 1005, 10, 15, 99, 109, 659, 104, 0, 104, 1,
        21101, 937267929896, 0, 1, 21101, 0, 354, 0, 1106, 0, 458, 21102, 1, 936995566336, 1,
        21102, 1, 365, 0, 1106, 0, 458, 3, 10, 104, 0, 104, 1, 3, 10, 104, 0, 104, 0, 3, 10, 104,
        0, 104, 1, 3, 10, 104, 0, 104, 1, 3, 10, 104, 0, 104, 0, 3, 10, 104, 0, 104, 1, 21101,
        3263269979, 0, 1, 21101, 0, 412, 0, 1106, 0, 458, 21102, 1, 46174071899, 1, 21101, 0, 423,
        0, 1106, 0, 458, 3, 10, 104, 0, 104, 0, 3, 10, 104, 0, 104, 0, 21101, 825544561428, 0, 1,
        21102, 446, 1, 0, 1105, 1, 458, 21102, 1, 867966018404, 1, 21101, 457, 0, 0, 1106, 0, 458,
        99, 109, 2, 21202, -1, 1, 1, 21102, 40, 1, 2, 21101, 489, 0, 3, 21102, 1, 479, 0, 1105, 1,
        522, 109, -2, 2106, 0, 0, 0, 1, 0, 0, 1, 109, 2, 3, 10, 204, -1, 1001, 484, 485, 500, 4, 0,
        1001, 484, 1, 484, 108, 4, 484, 10, 1006, 10, 516, 1102, 0, 1, 484, 109, -2, 2105, 1, 0, 0,
        109, 4, 2102, 1, -1, 521, 1207, -3, 0, 10, 1006, 10, 539, 21101, 0, 0, -3, 21201, -3, 0, 1,
        22101, 0, -2, 2, 21101, 1, 0, 3, 21102, 558, 1, 0, 1105, 1, 563, 109, -4, 2105, 1, 0, 109,
        5, 1207, -3, 1, 10, 1006, 10, 586, 2207, -4, -2, 10, 1006, 10, 586, 22102, 1, -4, -4, 1106,
        0, 654, 22101, 0, -4, 1, 21201, -3, -1, 2, 21202, -2, 2, 3, 21102, 1, 605, 0, 1105, 1, 563,
        22101, 0, 1, -4, 21102, 1, 1, -1, 2207, -4, -2, 10, 1006, 10, 624, 21102, 1, 0, -1, 22202,
        -2, -1, -2, 2107, 0, -3, 10, 1006, 10, 646, 21201, -1, 0, 1, 21102, 646, 1, 0, 106, 0, 521,
        21202, -2, -1, -2, 22201, -4, -2, -4, 109, -5, 2106, 0, 0,
    ];

    let panels = run(&positions, Color::Black);

    // 2129
    println!("Day 9:A = {}", panels.len());

    let panels = run(&positions, Color::White);

    let mut min_x = 10_000;
    let mut max_x = -10_000;
    let mut min_y = 10_000;
    let mut max_y = -10_000;

    for ((x, y), _) in panels.iter() {
        min_x = min(*x, min_x);
        max_x = max(*x, max_x);
        min_y = min(*y, min_y);
        max_y = max(*y, max_y);
    }

    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            if let Some(Color::White) = panels.get(&(x, y)) {
                print!("O");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!("Day 11:B = PECKRGZL");
}
