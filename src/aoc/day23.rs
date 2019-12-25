use super::assert::*;
use super::intcode::Machine;
use std::collections::VecDeque;

const MACHINE_COUNT: i64 = 50;

#[derive(Debug)]
struct Packet {
    address: i64,
    value: i64,
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

            for p in m.values.rchunks_exact(3) {
                let address = p[2];
                let x = p[1];
                let y = p[0];

                if address == 255 {
                    if part_1_solved == false {
                        part_1_solved = true;
                        assert_eq(Day::new(23, Part::A), 22_659, y);
                    }
                    nat = Some((x, y));
                } else {
                    messages.push_back(Packet { address, value: x });
                    messages.push_back(Packet { address, value: y });
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

            messages.push_back(Packet {
                address: 0,
                value: x,
            });
            messages.push_back(Packet {
                address: 0,
                value: y,
            });
        } else {
            for n in 0..MACHINE_COUNT {
                messages.push_back(Packet {
                    address: n,
                    value: -1,
                });
            }
        }
    }
}
