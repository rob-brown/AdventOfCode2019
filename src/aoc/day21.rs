use super::assert::*;
use super::intcode::Machine;
use std::io::stdin;

pub fn solve() {
    let initial = Machine::from_file("input/day21.txt");
    let mut machine = Machine::init(&initial.positions);
    let commands = [
    "NOT A J",
	"NOT C T",
	"AND D T",
	"OR T J",
    "WALK",
    ""
    ].join("\n").chars().map(|x| x as i64).rev().collect::<Vec<i64>>();

    machine.run(commands);
    assert_eq(Day::new(21, Part::A), 19354890, machine.values.pop_back().unwrap());

    let mut machine = Machine::init(&initial.positions);
    let commands = [
    "NOT A J",
	"AND D J",
	"NOT B T",
	"AND D T",
	"AND H T",
	"OR T J",
	"NOT C T",
	"AND D T",
	"AND E T",
	"OR T J",
	"NOT C T",
	"AND D T",
	"AND H T",
	"OR T J",
    "RUN",
    ""
    ].join("\n").chars().map(|x| x as i64).rev().collect::<Vec<i64>>();

    machine.run(commands);
    assert_eq(Day::new(21, Part::B), 1140664209, machine.values.pop_back().unwrap());
}
