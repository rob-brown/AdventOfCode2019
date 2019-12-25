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

    let mut inputs: Vec<i64> = amp1.values.iter().cloned().collect();
    inputs.push(sequence[1]);
    amp2.run(inputs);

    let mut inputs: Vec<i64> = amp2.values.iter().cloned().collect();
    inputs.push(sequence[2]);
    amp3.run(inputs);

    let mut inputs: Vec<i64> = amp3.values.iter().cloned().collect();
    inputs.push(sequence[3]);
    amp4.run(inputs);

    let mut inputs: Vec<i64> = amp4.values.iter().cloned().collect();
    inputs.push(sequence[4]);
    amp5.run(inputs);

    // Loop until halt
    while amp5.halted == false {
        amp1.run(amp5.values.iter().cloned().collect());
        amp2.run(amp1.values.iter().cloned().collect());
        amp3.run(amp2.values.iter().cloned().collect());
        amp4.run(amp3.values.iter().cloned().collect());
        amp5.run(amp4.values.iter().cloned().collect());
    }

    amp5.values[0]
}

pub fn solve() {
    let initial = Machine::from_file("input/day7.txt");
    let mut data = [0, 1, 2, 3, 4];
    let mut maximum = max(0, run_sequence(data.to_vec(), &initial.positions));

    while data.next_permutation() {
        maximum = max(maximum, run_sequence(data.to_vec(), &initial.positions));
    }

    assert_eq(Day::new(7, Part::A), 38_500, maximum);

    let mut data = [5, 6, 7, 8, 9];
    let mut maximum = max(0, run_streaming_sequence(data.to_vec(), &initial.positions));

    while data.next_permutation() {
        maximum = max(
            maximum,
            run_streaming_sequence(data.to_vec(), &initial.positions),
        );
    }

    assert_eq(Day::new(7, Part::B), 33_660_560, maximum);
}
