use super::assert::*;
use super::intcode::Machine;
use core::cmp::{min, Ord, Ordering};
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

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
fn print_screen(screen: &HashSet<(i32, i32)>) {
    for y in -SCREEN_HEIGHT..SCREEN_HEIGHT {
        for x in -SCREEN_WIDTH..SCREEN_WIDTH {
            if screen.contains(&(x, y)) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!();
}

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
    machine.values[0] as i32
}

// Used to turn max priority queue to min priority queue.
#[derive(PartialEq, Eq, Debug)]
struct Priority {
    value: i32,
}

impl Priority {
    fn new(value: i32) -> Self {
        Self { value }
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
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut machine = Machine::from_file("input/day15.txt");
    let mut current = (0, 0);
    let mut direction = Direction::North;
    let end: (i32, i32);

    'outer: loop {
        let next = move_forward(current, direction);
        let response = step(&mut machine, direction);

        match response {
            // Hit wall
            0 => direction = direction.turn_left(),

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
                    assert_eq(Day::new(15, Part::A), 374, min(*distance, alternate));
                    break 'outer2;
                }
            }
        }
    }
}
