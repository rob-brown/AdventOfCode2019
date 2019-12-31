use super::assert::*;
use super::intcode::Machine;

pub fn in_beam(point: (i64, i64), machine: &Machine) -> bool {
    let mut machine = Machine::init(&machine.positions);
    machine.run(vec![point.1, point.0]);

    machine.values.pop_front() == Some(1)
}

pub fn solve() {
    let machine = Machine::from_file("input/day19.txt");
    let mut count = 0;

    for y in 0..50 {
        for x in 0..50 {
            if in_beam((x, y), &machine) {
                count += 1;
            }
        }
    }

    assert_eq(Day::new(19, Part::A), 171, count);

    let mut current = (0, 100);

    loop {
        let bottom_left = (current.0, current.1 + 99);

        if in_beam(bottom_left, &machine) {
            let top_right = (current.0 + 99, current.1);

            if in_beam(top_right, &machine) {
                break;
            } else {
                current = (current.0, current.1 + 1);
            }
        } else {
            current = (current.0 + 1, current.1);
        }
    }

    let result = current.0 * 10_000 + current.1;

    assert_eq(Day::new(19, Part::B), 9_741_242, result);
}
