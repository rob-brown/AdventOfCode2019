use super::assert::*;
use super::intcode::Machine;
use std::io::stdin;

pub fn solve() {
    let mut machine = Machine::from_file("input/day21.txt");
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
}
