use super::intcode::Machine;
use std::collections::{HashMap, HashSet};
use priority_queue::PriorityQueue;
use core::cmp::{Ord, Ordering, min};

const SCREEN_WIDTH: i32 = 20;
const SCREEN_HEIGHT: i32 = 20;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn to_int(&self) -> i32 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }
}

#[allow(dead_code)]
fn reset_screen() {
    print!("\x1B[{}D\x1B[{}A", SCREEN_WIDTH + 1, SCREEN_HEIGHT + 1);
}

#[allow(dead_code)]
fn clear_screen() {
    for _ in 0..(SCREEN_HEIGHT * 2 + 1) {
        print!("\x1B[1A\x1B[2K");
    }
}

#[allow(dead_code)]
fn print_screen(screen: &HashMap<(i32, i32), i32>) {
    for y in -SCREEN_HEIGHT..SCREEN_HEIGHT {
        for x in -SCREEN_WIDTH..SCREEN_WIDTH {
            match screen.get(&(x, y)).unwrap_or(&-1) {
                // Wall
                0 => print!("#"),

                // Clear
                1 => print!("."),

                // Destination
                2 => print!("X"),

                _ => print!(" "),
            }
        }
        println!();
    }

    println!();
}

// fn turn_left(direction: i32) -> i32 {
//     match direction {
//         1 => 4,
//         2 => 1,
//         3 => 2,
//         4 => 3,
//         _ => panic!(),
//     }
// }
//
// fn turn_right(direction: i32) -> i32 {
//     match direction {
//         1 => 2,
//         2 => 3,
//         3 => 4,
//         4 => 1,
//         _ => panic!(),
//     }
// }

fn move_forward(point: (i32, i32), direction: Direction) -> (i32, i32) {
    match direction {
        Direction::North => (point.0, point.1 - 1),
        Direction::South => (point.0, point.1 + 1),
        Direction::East => (point.0 + 1, point.1),
        Direction::West => (point.0 - 1, point.1),
    }
}

fn step(machine: &mut Machine, direction: Direction) -> i32 {
    machine.run(vec![direction.to_int() as i64]);
    machine.values.pop().unwrap() as i32
}

// Used to turn max priority queue to min priority queue.
#[derive(PartialEq, Eq, Debug)]
struct Priority {
    value: i32,
}

impl Priority {
    fn new(value: i32) -> Self {
        Self {value}
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve() {
    let positions: Vec<i64> = vec![
        3,1033,1008,1033,1,1032,1005,1032,31,1008,1033,2,1032,1005,1032,58,1008,1033,3,1032,1005,1032,81,1008,1033,4,1032,1005,1032,104,99,102,1,1034,1039,101,0,1036,1041,1001,1035,-1,1040,1008,1038,0,1043,102,-1,1043,1032,1,1037,1032,1042,1105,1,124,102,1,1034,1039,1001,1036,0,1041,1001,1035,1,1040,1008,1038,0,1043,1,1037,1038,1042,1105,1,124,1001,1034,-1,1039,1008,1036,0,1041,1001,1035,0,1040,1001,1038,0,1043,1002,1037,1,1042,1105,1,124,1001,1034,1,1039,1008,1036,0,1041,102,1,1035,1040,101,0,1038,1043,1001,1037,0,1042,1006,1039,217,1006,1040,217,1008,1039,40,1032,1005,1032,217,1008,1040,40,1032,1005,1032,217,1008,1039,3,1032,1006,1032,165,1008,1040,33,1032,1006,1032,165,1101,0,2,1044,1105,1,224,2,1041,1043,1032,1006,1032,179,1101,1,0,1044,1106,0,224,1,1041,1043,1032,1006,1032,217,1,1042,1043,1032,1001,1032,-1,1032,1002,1032,39,1032,1,1032,1039,1032,101,-1,1032,1032,101,252,1032,211,1007,0,37,1044,1106,0,224,1102,1,0,1044,1105,1,224,1006,1044,247,101,0,1039,1034,101,0,1040,1035,1001,1041,0,1036,1002,1043,1,1038,101,0,1042,1037,4,1044,1106,0,0,42,4,15,10,25,91,86,34,69,14,50,9,24,24,54,10,18,63,17,2,88,36,31,60,20,13,20,76,94,25,41,36,78,3,39,17,94,10,25,22,16,67,72,31,47,15,25,66,8,17,54,8,89,67,29,28,92,11,54,14,4,64,78,28,80,66,6,70,36,56,13,63,17,19,83,17,27,29,34,54,4,93,24,71,6,66,22,21,92,93,39,4,31,76,72,25,74,89,18,62,18,27,57,35,83,39,14,23,95,2,79,25,97,86,13,79,1,34,90,81,29,45,31,38,67,17,92,32,31,50,1,42,81,1,2,87,7,52,74,20,85,22,32,47,16,77,96,28,14,74,22,55,15,75,44,29,19,8,73,2,54,18,26,64,95,21,98,48,25,36,11,78,77,5,16,70,18,10,76,51,51,10,25,43,56,12,13,48,8,17,68,10,64,25,93,42,3,52,24,72,99,23,54,13,44,17,15,8,68,59,15,95,61,9,50,8,51,23,8,39,13,95,64,12,28,56,90,1,62,27,12,60,6,5,18,24,13,99,12,18,92,97,7,56,22,48,91,34,87,32,98,20,89,74,16,51,84,21,46,14,23,52,17,57,12,50,17,97,23,99,11,21,68,21,61,89,13,45,64,89,18,36,40,35,90,9,1,3,81,33,32,83,99,97,34,4,46,31,21,90,62,14,93,11,22,99,51,70,88,51,2,4,29,36,35,48,17,25,30,69,34,3,39,89,31,89,33,30,88,77,18,30,67,17,40,61,19,40,85,26,23,49,22,41,30,13,79,6,34,40,33,43,49,84,19,78,43,10,74,18,61,15,22,51,86,2,78,11,33,92,24,88,27,24,44,2,97,4,4,49,72,93,24,65,79,21,60,33,46,36,22,15,87,33,78,2,49,70,7,78,78,11,14,64,41,61,41,6,1,49,35,78,47,65,14,66,10,86,76,2,32,88,3,24,14,87,9,95,32,19,4,10,67,60,15,19,53,47,24,29,65,5,95,35,1,70,16,43,53,11,64,17,34,84,74,65,30,18,58,2,35,48,38,33,46,16,87,27,12,79,11,88,35,7,5,35,67,83,38,6,17,56,82,13,45,32,30,67,25,62,7,43,63,9,36,14,58,53,25,98,12,38,78,13,63,93,33,11,54,9,66,32,79,62,47,28,6,67,31,53,71,2,30,59,12,90,59,67,2,58,52,1,30,51,49,22,89,88,27,19,41,27,13,19,76,5,82,58,12,49,51,17,15,73,35,25,74,90,29,14,96,83,69,11,18,14,10,40,93,35,31,35,36,58,36,16,48,7,66,98,31,47,34,47,33,5,28,82,88,1,30,80,95,32,87,2,19,91,74,74,19,8,25,63,65,51,30,14,41,98,99,21,90,15,91,3,31,74,27,31,77,28,74,4,27,88,82,11,54,35,52,13,88,71,93,20,82,18,36,68,33,83,1,18,5,42,46,29,62,10,78,67,9,84,48,22,33,74,36,53,58,31,5,8,55,10,24,49,34,81,1,4,86,5,25,2,75,36,49,2,24,88,72,8,64,36,38,10,23,36,93,28,51,90,4,99,57,31,10,14,94,21,27,61,34,70,41,32,14,91,20,83,30,54,26,44,30,85,96,87,35,16,61,99,16,32,53,68,87,1,89,43,9,17,4,39,50,61,8,49,27,48,13,51,34,47,30,89,68,50,18,63,99,50,32,41,33,71,1,43,57,64,24,95,9,89,8,64,18,75,23,97,74,67,24,55,1,87,97,44,0,0,21,21,1,10,1,0,0,0,0,0,0
    ];

    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut machine = Machine::init(&positions);
    let mut current = (0, 0);
    let mut direction = Direction::North;
    let end: (i32, i32);

    'outer: loop {
        let next = move_forward(current, direction);
        let response = step(&mut machine, direction);

        match response {
            // Hit wall
            0 => {
                direction = direction.turn_left()
            }

            // Stepped forward
            1 => {
                map.insert(next);
                current = next;
                direction = direction.turn_right();
            }

            // Found target
            2 => {
                map.insert(next);
                end = next;
                break 'outer;
            }

            _ => panic!(),
        }

        // print_screen(&map);
        // let delay = std::time::Duration::from_millis(5);
        // std::thread::sleep(delay);
        // clear_screen();
    }

    let mut queue: PriorityQueue<(i32, i32), Priority> = PriorityQueue::new();
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new();
    const MAX_DISTANCE: i32 = 10_000;

    for (x, y) in map.iter() {
        queue.push((*x, *y), Priority::new(MAX_DISTANCE));
    }

    distances.insert((0, 0), 0);
    queue.push((0, 0), Priority::new(0));

    'outer2: while let Some(((x, y), d)) = queue.pop() {
        let neighbors = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];

        for n in neighbors.iter() {
            if map.contains(&n) {
                let distance = distances.entry(*n).or_insert(MAX_DISTANCE);
                let alternate = d.value + 1;

                if alternate < *distance {
                    *distance = alternate;
                    queue.change_priority(n, Priority::new(alternate));
                }

                if *n == end {
                    println!("Day 15:A = {}", min(*distance, alternate));
                    break 'outer2;
                }
            }
        }
    }
}
