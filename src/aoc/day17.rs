use super::assert::*;
use super::intcode::Machine;
use std::collections::HashSet;

fn is_intersection(point: (i32, i32), map: &HashSet<(i32, i32)>) -> bool {
    let (x, y) = point;
    let up = (x, y - 1);
    let down = (x, y + 1);
    let left = (x - 1, y);
    let right = (x + 1, y);
    map.contains(&point)
        && map.contains(&up)
        && map.contains(&down)
        && map.contains(&left)
        && map.contains(&right)
}

pub fn solve() {
    let mut machine = Machine::from_file("input/day17.txt");
    machine.run(vec![]);

    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let mut robot = (0, 0);

    for c in machine.values {
        match c as u8 as char {
            '#' => {
                map.insert((x, y));
                x += 1;
            }

            '\n' => {
                x = 0;
                y += 1;
            }

            '^' => {
                robot = (x, y);
                map.insert((x, y));
                x += 1;
            }

            _ => {
                x += 1;
            }
        }
    }

    let sum = map
        .iter()
        .filter(|&p| is_intersection(*p, &map))
        .map(|(x, y)| x * y)
        .sum();

    assert_eq(Day::new(17, Part::A), 6052, sum);
}
