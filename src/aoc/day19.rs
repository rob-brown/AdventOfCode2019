use super::assert::*;
use super::intcode::Machine;
use std::collections::HashSet;

pub fn solve() {
    let mut machine = Machine::from_file("input/day19.txt");

    // machine.run(vec![0, 0]);
    // println!("value: {:?}", machine.values);

    let mut count = 0;

    for y in 0..50 {
        for x in 0..50 {
            // println!("({}, {})", x, y);
            machine.run(vec![y, x]);
            // println!("values: {:?}", machine.values);
            // machine.values.clear();
            if machine.values.pop_front() == Some(1) {
                println!("Hit");
                count += 1;
            }
        }
    }

    println!("count: {}", count);
}
