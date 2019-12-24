use std::collections::HashSet;

type World = Vec<char>;

const CLEAR: char = '.';
const BUG: char = '#';

fn biodiversity(world: &World) -> usize {
    let mut score  = 0;

    for (n, x) in world.iter().enumerate() {
        if *x == BUG {
            score += 2_usize.pow(n as u32);
        }
    }

    score
}

#[inline]
fn index(x: i32, y: i32) -> usize {
    (y * 5 + x) as usize
}

fn has_bug(world: &World, x: i32, y: i32) -> bool {
    world[index(x, y)] == BUG
}

fn neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .into_iter()
        .filter(|(a, b)| (0..5).contains(a) && (0..5).contains(b))
        .collect()
}

fn neighbor_count(world: &World, x: i32, y: i32) -> usize {
    neighbors(x, y).into_iter().filter(|(a, b)| has_bug(world, *a, *b)).count()
}

#[allow(dead_code)]
fn print_world(world: &World) {
    for (n, c) in world.iter().enumerate() {
        if n % 5 == 0 {
            println!();
        }
        print!("{}", c);
    }
    println!();
}

pub fn solve() {
    let initial: World = "..#.#.#.####....#.#.###..".chars().collect();
    let mut past: HashSet<World> = HashSet::new();
    past.insert(initial.clone());

    let mut current: World = initial.clone();

    print_world(&current);

    loop {
        let mut next = [CLEAR; 25];

        for y in 0..5 {
            for x in 0..5 {
                let count = neighbor_count(&current, x, y);
                let bug = has_bug(&current, x, y);

                next[index(x, y)] = match (bug, count) {
                    (true, 1) => BUG,
                    (false, 1) => BUG,
                    (false, 2) => BUG,
                    _ => CLEAR,
                }
            }
        }

        current = next.iter().cloned().collect();

        print_world(&current);

        if past.contains(&current) {
            // 18401265
            println!("Day 24:A = {}", biodiversity(&current));
            break;
        } else {
            past.insert(current.clone());
        }
    }
}
