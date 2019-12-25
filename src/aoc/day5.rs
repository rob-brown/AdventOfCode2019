use super::assert::*;
use super::intcode::Machine;

pub fn solve() {
    let initial = Machine::from_file("input/day5.txt");
    let mut machine = Machine::init(&initial.positions);
    machine.run(vec![1]);
    assert_eq(Day::new(5, Part::A), 11_049_715, machine.values[0]);

    let mut machine = Machine::init(&initial.positions);
    machine.run(vec![5]);
    assert_eq(Day::new(5, Part::B), 2_140_710, machine.values[0]);
}
