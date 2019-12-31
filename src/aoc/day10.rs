use super::assert::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use core::cmp::Ordering;
use core::f32::consts::{PI, FRAC_PI_4};

const TAU: f32 = PI * 2.0;

fn visible_asteroids(
    i: usize,
    j: usize,
    asteroids: &HashSet<(usize, usize)>,
) -> HashMap<String, ((usize, usize), f32, usize)> {
    let mut angles: HashMap<String, ((usize, usize), f32, usize)> = HashMap::new();

    for &(i2, j2) in asteroids.iter() {
        // Don't include angle to self.
        if i == i2 && j == j2 {
            continue;
        }

        let opposite = (i2 as i32) - (i as i32);
        let adjacent = (j2 as i32) - (j as i32);

        // Manhattan distance for fast, easy distance.
        let distance = (opposite.abs() + adjacent.abs()) as usize;
        // Rotate 90˚ and shift range from -pi..pi to 0..tau.
        let angle = (f32::atan2(opposite as f32, adjacent as f32) + TAU + FRAC_PI_4) % TAU;
        // f32 isn't hashable in Rust.
        let key = angle.to_string();

        if let Some((_, _, other_distance)) = angles.get(&key) {
            if distance < *other_distance {
                angles.insert(key, ((i2, j2), angle, distance));
            }
        } else {
            angles.insert(key, ((i2, j2), angle, distance));
        }
    }

    angles
}

pub fn solve() {
    let mut file = File::open("input/day10.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut asteroids: HashSet<(usize, usize)> = HashSet::new();

    let file = File::open("input/day10.txt").unwrap();
    for (y, line) in BufReader::new(file).lines().enumerate() {
        for (x, char) in String::from(line.unwrap()).chars().enumerate() {
            if char == '#' {
                asteroids.insert((x, y));
            }
        }
    }

    let mut maximum = 0;
    let mut visible: HashMap<String, ((usize, usize), f32, usize)> = HashMap::new();
    let mut angles: HashMap<String, ((usize, usize), f32, usize)>;

    for (i1, j1) in asteroids.iter() {
        angles = visible_asteroids(*i1, *j1, &asteroids);

        if angles.len() > maximum {
            maximum = angles.len();
            visible = angles;
        }
    }

    assert_eq(Day::new(10, Part::A), 309, maximum);

    let mut info: Vec<((usize, usize), f32)> = visible
            .values()
            .map(|(p, a, _)| (*p, *a))
            .collect();
    info.sort_by(|(_, a1), (_, a2)| if a1 < a2 { Ordering::Less } else { Ordering::Greater });
    let ((x, y), _) = info[200];

    assert_eq(Day::new(10, Part::B), 416, x * 100 + y)
}
