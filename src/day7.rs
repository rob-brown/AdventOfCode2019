use core::cmp::max;
use permutohedron::LexicalPermutation;

fn mode_arg1(op_code: i64) -> i64 {
    (op_code % 1000) / 100
}

fn mode_arg2(op_code: i64) -> i64 {
    (op_code % 10_000) / 1000
}

fn lookup(positions: &Vec<i64>, mode: i64, value: i64) -> i64 {
    match mode {
        0 => positions[value as usize],

        1 => value,

        _ => panic!("Unknown mode {}", mode),
    }
}

struct Machine {
    values: Vec<i64>,
    ip: usize,
    halted: bool,
    positions: Vec<i64>,
}

impl Machine {
    fn init(positions: &Vec<i64>) -> Self {
        Machine::new(Vec::new(), 0, false, positions.clone())
    }

    fn new(values: Vec<i64>, ip: usize, halted: bool, positions: Vec<i64>) -> Self {
        Self {
            values,
            ip,
            halted,
            positions,
        }
    }
}

fn run_intcode(machine: Machine, inputs: Vec<i64>) -> Machine {
    let mut ip = machine.ip;
    let mut inputs = inputs;
    let mut output = Vec::new();
    let mut halted = false;
    let mut positions = machine.positions;

    loop {
        let op_code = positions[ip];
        match op_code % 100 {
            // Add
            1 => {
                let arg1 = lookup(&positions, mode_arg1(op_code), positions[ip + 1]);
                let arg2 = lookup(&positions, mode_arg2(op_code), positions[ip + 2]);
                let dest = positions[ip + 3] as usize;
                positions[dest] = arg1 + arg2;
                ip += 4;
            }

            // Multiply
            2 => {
                let arg1 = lookup(&positions, mode_arg1(op_code), positions[ip + 1]);
                let arg2 = lookup(&positions, mode_arg2(op_code), positions[ip + 2]);
                let dest = positions[ip + 3] as usize;
                positions[dest] = arg1 * arg2;
                ip += 4;
            }

            // Read input
            3 => {
                if let Some(input) = inputs.pop() {
                    let a = positions[ip + 1] as usize;
                    positions[a] = input;
                    ip += 2;
                } else {
                    // Yield if no input available
                    break;
                }
            }

            // Write output
            4 => {
                let value = lookup(&positions, mode_arg1(op_code), positions[ip + 1]);
                output.push(value);
                ip += 2;
            }

            // Jump if true
            5 => {
                let arg1 = lookup(&positions, mode_arg1(op_code), positions[ip + 1]);
                let arg2 = lookup(&positions, mode_arg2(op_code), positions[ip + 2]) as usize;
                ip = if arg1 == 0 { ip + 3 } else { arg2 };
            }

            // Jump if false
            6 => {
                let arg1 = lookup(&positions, mode_arg1(op_code), positions[ip + 1]);
                let arg2 = lookup(&positions, mode_arg2(op_code), positions[ip + 2]) as usize;
                ip = if arg1 == 0 { arg2 } else { ip + 3 };
            }

            // Less than
            7 => {
                let arg1 = lookup(&positions, mode_arg1(op_code), positions[ip + 1]);
                let arg2 = lookup(&positions, mode_arg2(op_code), positions[ip + 2]);
                let dest = positions[ip + 3] as usize;
                positions[dest] = if arg1 < arg2 { 1 } else { 0 };
                ip += 4;
            }

            // Equal
            8 => {
                let arg1 = lookup(&positions, mode_arg1(op_code), positions[ip + 1]);
                let arg2 = lookup(&positions, mode_arg2(op_code), positions[ip + 2]);
                let dest = positions[ip + 3] as usize;
                positions[dest] = if arg1 == arg2 { 1 } else { 0 };
                ip += 4;
            }

            // Halt
            99 => {
                halted = true;
                break;
            }

            x => panic!("Invalid command {}", x),
        }
    }

    output.reverse();

    Machine::new(output, ip, halted, positions)
}

fn run_sequence(sequence: Vec<i64>, positions: &Vec<i64>) -> i64 {
    let mut value = 0;

    for s in sequence {
        let machine = Machine::init(&positions);
        value = run_intcode(machine, vec![value, s]).values.pop().unwrap();
    }

    value
}

fn run_streaming_sequence(sequence: Vec<i64>, positions: &Vec<i64>) -> i64 {
    let mut amp1 = Machine::init(&positions);
    let mut amp2 = Machine::init(&positions);
    let mut amp3 = Machine::init(&positions);
    let mut amp4 = Machine::init(&positions);
    let mut amp5 = Machine::init(&positions);

    // Run and initialize
    amp1 = run_intcode(amp1, vec![0, sequence[0]]);

    let mut inputs: Vec<i64> = amp1.values.clone();
    inputs.push(sequence[1]);
    amp2 = run_intcode(amp2, inputs);

    let mut inputs: Vec<i64> = amp2.values.clone();
    inputs.push(sequence[2]);
    amp3 = run_intcode(amp3, inputs);

    let mut inputs: Vec<i64> = amp3.values.clone();
    inputs.push(sequence[3]);
    amp4 = run_intcode(amp4, inputs);

    let mut inputs: Vec<i64> = amp4.values.clone();
    inputs.push(sequence[4]);
    amp5 = run_intcode(amp5, inputs);

    // Loop until halt
    while amp5.halted == false {
        amp1 = run_intcode(amp1, amp5.values.clone());
        amp2 = run_intcode(amp2, amp1.values.clone());
        amp3 = run_intcode(amp3, amp2.values.clone());
        amp4 = run_intcode(amp4, amp3.values.clone());
        amp5 = run_intcode(amp5, amp4.values.clone());
    }

    amp5.values.pop().unwrap()
}

pub fn solve() {
    let positions: Vec<i64> = vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 47, 64, 89, 110, 191, 272, 353, 434, 99999, 3,
        9, 101, 4, 9, 9, 102, 3, 9, 9, 101, 5, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 5, 9, 4, 9, 99, 3, 9,
        101, 2, 9, 9, 102, 5, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 1001, 9, 5, 9, 102, 4, 9, 9,
        1001, 9, 5, 9, 1002, 9, 2, 9, 1001, 9, 3, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 101, 4, 9, 9,
        1002, 9, 4, 9, 1001, 9, 4, 9, 4, 9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101,
        1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4,
        9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3,
        9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101,
        1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9,
        4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9,
        1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002,
        9, 2, 9, 4, 9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9,
        4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9,
        2, 9, 4, 9, 99,
    ];

    let mut data = [0, 1, 2, 3, 4];
    let mut maximum = max(0, run_sequence(data.to_vec(), &positions));

    while data.next_permutation() {
        maximum = max(maximum, run_sequence(data.to_vec(), &positions));
    }

    println!("Day 7:A = {}", maximum);

    let mut data = [5, 6, 7, 8, 9];
    let mut maximum = max(0, run_streaming_sequence(data.to_vec(), &positions));

    while data.next_permutation() {
        maximum = max(maximum, run_streaming_sequence(data.to_vec(), &positions));
    }

    // 33660560
    println!("Day 7:B = {}", maximum);
}
