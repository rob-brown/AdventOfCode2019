use super::assert::*;
use super::intcode::Machine;
use core::cmp::{max, min};
use std::collections::HashMap;

enum Color {
    Black,
    White,
}

impl Color {
    fn to_int(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    fn from_int(int: i64) -> Self {
        match int {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!(),
        }
    }
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn run(positions: &Vec<i64>, start_color: Color) -> HashMap<(i32, i32), Color> {
    let mut panels: HashMap<(i32, i32), Color> = HashMap::new();
    let mut machine = Machine::init(&positions);
    let mut current = (0, 0);
    let mut direction = Direction::Up;

    panels.insert((0, 0), start_color);

    while machine.halted == false {
        let color = panels.get(&current).unwrap_or(&Color::Black);

        machine.run(vec![color.to_int()]);

        let new_color = machine.values.pop_front().unwrap();
        let turn = machine.values.pop_front().unwrap();

        let (dir, point) = match (direction, turn) {
            (Direction::Up, 0) => (Direction::Left, (current.0 - 1, current.1)),
            (Direction::Up, 1) => (Direction::Right, (current.0 + 1, current.1)),
            (Direction::Left, 0) => (Direction::Down, (current.0, current.1 + 1)),
            (Direction::Left, 1) => (Direction::Up, (current.0, current.1 - 1)),
            (Direction::Down, 0) => (Direction::Right, (current.0 + 1, current.1)),
            (Direction::Down, 1) => (Direction::Left, (current.0 - 1, current.1)),
            (Direction::Right, 0) => (Direction::Up, (current.0, current.1 - 1)),
            (Direction::Right, 1) => (Direction::Down, (current.0, current.1 + 1)),
            _ => panic!(),
        };

        panels.insert(current, Color::from_int(new_color));

        direction = dir;
        current = point;
    }

    panels
}

pub fn solve() {
    let initial = Machine::from_file("input/day11.txt");
    let panels = run(&initial.positions, Color::Black);

    assert_eq(Day::new(11, Part::A), 2129, panels.len());

    let panels = run(&initial.positions, Color::White);

    let mut min_x = 10_000;
    let mut max_x = -10_000;
    let mut min_y = 10_000;
    let mut max_y = -10_000;

    for ((x, y), _) in panels.iter() {
        min_x = min(*x, min_x);
        max_x = max(*x, max_x);
        min_y = min(*y, min_y);
        max_y = max(*y, max_y);
    }

    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            if let Some(Color::White) = panels.get(&(x, y)) {
                print!("O");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    assert_eq(Day::new(11, Part::B), "PECKRGZL", "PECKRGZL");
}
