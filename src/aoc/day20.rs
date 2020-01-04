use super::assert::*;
use priority_queue::PriorityQueue;
use std::cmp::{max, Reverse};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (i32, i32);
type RecursivePoint = (Point, i32);

pub fn solve() {
    let mut seen_letters: HashMap<Point, char> = HashMap::new();
    let mut portals_by_name: HashMap<String, Point> = HashMap::new();
    let mut portals: HashMap<Point, Point> = HashMap::new();
    let mut path: HashSet<Point> = HashSet::new();
    let mut start = Point::default();
    let mut end = Point::default();
    let mut line_count = 0;
    let mut line_width = 0;

    let file = File::open("input/day20.txt").unwrap();
    for (y, line) in BufReader::new(file).lines().enumerate() {
        let mut temp_width = 0;
        line_count += 1;

        for (x, char) in String::from(line.unwrap()).chars().enumerate() {
            temp_width += 1;
            let x = x as i32;
            let y = y as i32;
            let point = (x, y);

            if char == '.' {
                path.insert(point);
            } else if char.is_ascii_uppercase() {
                let left = (x - 1, y);
                let up = (x, y - 1);

                if let Some((_, letter)) = seen_letters.remove_entry(&left) {
                    let bytes = vec![letter as u8, char as u8];
                    let name = String::from_utf8(bytes).unwrap();

                    let portal_point = if path.contains(&(x - 2, y)) {
                        (x - 2, y)
                    } else {
                        (point.0 + 1, point.1)
                    };

                    if &name == "AA" {
                        start = portal_point;
                    } else if &name == "ZZ" {
                        end = portal_point;
                    } else if let Some((_, portal)) = portals_by_name.remove_entry(&name) {
                        portals.insert(portal, portal_point);
                        portals.insert(portal_point, portal);
                    } else {
                        portals_by_name.insert(name, portal_point);
                    }
                } else if let Some((_, letter)) = seen_letters.remove_entry(&up) {
                    let bytes = vec![letter as u8, char as u8];
                    let name = String::from_utf8(bytes).unwrap();

                    let portal_point = if path.contains(&(x, y - 2)) {
                        (x, y - 2)
                    } else {
                        (point.0, point.1 + 1)
                    };

                    if &name == "AA" {
                        start = portal_point;
                    } else if &name == "ZZ" {
                        end = portal_point;
                    } else if let Some((_, portal)) = portals_by_name.remove_entry(&name) {
                        portals.insert(portal, portal_point);
                        portals.insert(portal_point, portal);
                    } else {
                        portals_by_name.insert(name, portal_point);
                    }
                } else {
                    seen_letters.insert(point, char);
                }
            }
        }

        line_width = max(line_width, temp_width);
    }

    let mut queue: PriorityQueue<Point, Reverse<i32>> = PriorityQueue::new();
    let mut explored: HashSet<Point> = HashSet::new();

    queue.push(start, Reverse(0));

    'outer: while let Some((point, Reverse(d))) = queue.pop() {
        let (x, y) = point;
        if point == end {
            assert_eq(Day::new(20, Part::A), 606, d);
            break;
        }

        explored.insert((x, y));

        let mut neighbors = vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];

        if let Some(portal) = portals.get(&point) {
            neighbors.push(*portal);
        }

        for n in neighbors.iter() {
            if path.contains(&n) && explored.contains(n) == false {
                queue.push(*n, Reverse(d + 1));
            }
        }
    }

    let mut queue: PriorityQueue<RecursivePoint, Reverse<i32>> = PriorityQueue::new();
    let mut explored: HashSet<RecursivePoint> = HashSet::new();

    queue.push((start, 0), Reverse(0));

    while let Some((node, Reverse(d))) = queue.pop() {
        let (point, depth) = node;
        let (x, y) = point;
        if point == end && depth == 0 {
            assert_eq(Day::new(20, Part::B), 7186, d);
            break;
        }

        explored.insert(node);

        let mut neighbors = vec![
            ((x, y - 1), depth),
            ((x, y + 1), depth),
            ((x - 1, y), depth),
            ((x + 1, y), depth),
        ];

        if let Some(portal) = portals.get(&point) {
            // Outer portals go up, inner portals go down.
            let next = if x == 2 || y == 2 || x == line_width - 3 || y == line_count - 3 {
                depth - 1
            } else {
                depth + 1
            };

            // Don't allow portals above the top level.
            if next >= 0 {
                neighbors.push((*portal, next));
            }
        }

        for n in neighbors.iter() {
            if path.contains(&n.0) && explored.contains(n) == false {
                queue.push(*n, Reverse(d + 1));
            }
        }
    }
}
