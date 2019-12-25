use super::assert::*;
use super::intcode::Machine;
use std::collections::VecDeque;

const MACHINE_COUNT: i64 = 50;

#[derive(Debug)]
struct Packet {
    address: i64,
    value: i64,
}

impl Packet {
    fn new(address: i64, value: i64) -> Self {
        Self {address, value}
    }
}

pub fn solve() {
    let initial = Machine::from_file("input/day23.txt");
    let mut machines: Vec<Machine> = Vec::new();
    let mut messages: VecDeque<Packet> = VecDeque::new();
    let mut prev_y: Option<i64> = None;
    let mut nat: Option<(i64, i64)> = None;
    let mut part_1_solved = false;

    for n in 0..MACHINE_COUNT {
        let mut m = Machine::init(&initial.positions);
        m.run(vec![n as i64]);
        machines.push(m);
    }

    loop {
        while let Some(packet) = messages.pop_front() {
            let m = &mut machines[packet.address as usize];
            let input = vec![packet.value];
            m.run(input);

            let mut iterator = m.values.iter();

            while let Some(address) = iterator.next() {
                let x = iterator.next().unwrap();
                let y = iterator.next().unwrap();

                if *address == 255 {
                    if part_1_solved == false {
                        part_1_solved = true;
                        assert_eq(Day::new(23, Part::A), 22_659, *y);
                    }
                    nat = Some((*x, *y));
                } else {
                    messages.push_back(Packet::new(*address, *x));
                    messages.push_back(Packet::new(*address, *y));
                }
            }
        }

        if let Some((x, y)) = nat.take() {
            if Some(y) == prev_y {
                assert_eq(Day::new(23, Part::B), 17_429, y);
                break;
            } else {
                prev_y = Some(y);
            }

            messages.push_back(Packet::new(0, x));
            messages.push_back(Packet::new(0, y));
        } else {
            for n in 0..MACHINE_COUNT {
                messages.push_back(Packet::new(n, -1));
            }
        }
    }
}
