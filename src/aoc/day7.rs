use super::assert::*;
use super::intcode::Machine;
use core::cmp::max;
use permutohedron::LexicalPermutation;

fn run_sequence(sequence: Vec<i64>, positions: &Vec<i64>) -> i64 {
    let mut value = 0;

    for s in sequence {
        let mut machine = Machine::init(&positions);
        machine.run(vec![value, s]);
        value = machine.values[0];
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
    amp1.run(vec![0, sequence[0]]);

    let mut inputs: Vec<i64> = amp1.values.clone();
    inputs.push(sequence[1]);
    amp2.run(inputs);

    let mut inputs: Vec<i64> = amp2.values.clone();
    inputs.push(sequence[2]);
    amp3.run(inputs);

    let mut inputs: Vec<i64> = amp3.values.clone();
    inputs.push(sequence[3]);
    amp4.run(inputs);

    let mut inputs: Vec<i64> = amp4.values.clone();
    inputs.push(sequence[4]);
    amp5.run(inputs);

    // Loop until halt
    while amp5.halted == false {
        amp1.run(amp5.values.clone());
        amp2.run(amp1.values.clone());
        amp3.run(amp2.values.clone());
        amp4.run(amp3.values.clone());
        amp5.run(amp4.values.clone());
    }

    amp5.values[0]
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

    assert_eq(Day::new(7, Part::A), 38_500, maximum);

    let mut data = [5, 6, 7, 8, 9];
    let mut maximum = max(0, run_streaming_sequence(data.to_vec(), &positions));

    while data.next_permutation() {
        maximum = max(maximum, run_streaming_sequence(data.to_vec(), &positions));
    }

    assert_eq(Day::new(7, Part::B), 33_660_560, maximum);
}
