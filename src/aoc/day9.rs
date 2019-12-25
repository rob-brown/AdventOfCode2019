use super::assert::*;
use super::intcode::Machine;

pub fn solve() {
    let initial = Machine::from_file("input/day9.txt");
    let mut machine = Machine::init(&initial.positions);
    machine.run(vec![1]);

    assert_eq(Day::new(9, Part::A), 3_906_448_201, machine.values[0]);

    let mut machine = Machine::init(&initial.positions);
    machine.run(vec![2]);

    assert_eq(Day::new(9, Part::B), 59_785, machine.values[0]);
}
