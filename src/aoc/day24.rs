use super::assert::*;
use std::collections::HashSet;

const CLEAR: char = '.';
const BUG: char = '#';
const UNKNOWN: char = '?';

#[derive(Clone, PartialEq, Hash)]
struct World {
    cells: Vec<char>,
}

impl Eq for World {}

impl World {
    fn empty() -> Self {
        let cells = [CLEAR; 25].to_vec();
        World {cells}
    }

    fn from_str(str: &str) -> Self {
        let cells = str.chars().collect();
        World {cells}
    }

    fn has_bug(&self, x: i32, y: i32) -> bool {
        self.cells[World::index(x, y)] == BUG
    }

    fn neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
        vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter(|(a, b)| (0..5).contains(a) && (0..5).contains(b))
            .collect()
    }

    fn neighbor_count(&self, x: i32, y: i32) -> usize {
        World::neighbors(x, y)
            .into_iter()
            .filter(|(a, b)| self.has_bug(*a, *b))
            .count()
    }

    #[inline(always)]
    fn index(x: i32, y: i32) -> usize {
        (y * 5 + x) as usize
    }

    fn biodiversity(&self) -> usize {
        let mut score = 0;

        for (n, x) in self.cells.iter().enumerate() {
            if *x == BUG {
                score += 2_usize.pow(n as u32);
            }
        }

        score
    }

    fn step(&self) -> Self {
        let mut next = World::empty();

        for y in 0..5 {
            for x in 0..5 {
                let count = self.neighbor_count(x, y);
                let bug = self.has_bug(x, y);

                next.cells[World::index(x, y)] = match (bug, count) {
                    (true, 1) => BUG,
                    (false, 1) => BUG,
                    (false, 2) => BUG,
                    _ => CLEAR,
                }
            }
        }

        next
    }

    fn bug_count(&self) -> usize {
        self.cells.iter().filter(|x| **x == BUG).count()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for (n, c) in self.cells.iter().enumerate() {
            if n % 5 == 0 {
                println!();
            }
            print!("{}", c);
        }
        println!();
    }
}

enum HyperWorld {
    Root(World, Box<HyperWorld>, Box<HyperWorld>),
    UpperBranch(World, Box<HyperWorld>),
    LowerBranch(World, Box<HyperWorld>),
    UpperEmpty,
    LowerEmpty,
}

impl HyperWorld {
    fn init(world: World) -> Self {
        HyperWorld::Root(world, Box::new(HyperWorld::UpperEmpty), Box::new(HyperWorld::LowerEmpty))
    }

    fn bug_count(&self) -> usize {
        match self {
            HyperWorld::Root(world, above, below) => {
                HyperWorld::bug_count(above) + HyperWorld::bug_count(below) + world.bug_count()
            }

            HyperWorld::UpperBranch(world, next) => {
                HyperWorld::bug_count(next) + world.bug_count()
            }

            HyperWorld::LowerBranch(world, next) => {
                HyperWorld::bug_count(next) + world.bug_count()
            }

            HyperWorld::UpperEmpty => 0,

            HyperWorld::LowerEmpty => 0,
        }
    }

    fn world(&self) -> World {
        match self {
            HyperWorld::Root(world, _, _) => world.clone(),
            HyperWorld::UpperBranch(world, _) => world.clone(),
            HyperWorld::LowerBranch(world, _) => world.clone(),
            HyperWorld::UpperEmpty => World::empty(),
            HyperWorld::LowerEmpty => World::empty(),
        }
    }

    fn step(&self, previous: Option<&World>) -> HyperWorld {
        match (self, previous) {
            (HyperWorld::Root(world, above, below), _) => {
                let new_world = HyperWorld::step_world(world, &above.world(), &below.world());
                HyperWorld::Root(new_world, Box::new(above.step(Some(world))), Box::new(below.step(Some(world))))
            }

            (HyperWorld::UpperBranch(world, next), Some(prev)) => {
                let new_world = HyperWorld::step_world(world, &next.world(), &prev);
                HyperWorld::UpperBranch(new_world, Box::new(next.step(Some(world))))
            }

            (HyperWorld::LowerBranch(world, next), Some(prev)) => {
                let new_world = HyperWorld::step_world(world, &prev, &next.world());
                HyperWorld::LowerBranch(new_world, Box::new(next.step(Some(world))))
            }

            (HyperWorld::UpperEmpty, Some(prev)) => {
                let new_world = HyperWorld::step_world(&World::empty(), &World::empty(), &prev);

                if new_world.bug_count() == 0 {
                    HyperWorld::UpperEmpty
                } else {
                    HyperWorld::UpperBranch(new_world, Box::new(HyperWorld::UpperEmpty))
                }
            }

            (HyperWorld::LowerEmpty, Some(prev)) => {
                let new_world = HyperWorld::step_world(&World::empty(), &prev, &World::empty());

                if new_world.bug_count() == 0 {
                    HyperWorld::LowerEmpty
                } else {
                    HyperWorld::LowerBranch(new_world, Box::new(HyperWorld::LowerEmpty))
                }
            }

            _ => panic!(),
        }
    }

    fn step_world(world: &World, above: &World, below: &World) -> World {
        let mut next = World::empty();

        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    // The center is always unknown since it's another 5x5 grid.
                    next.cells[World::index(2, 2)] = UNKNOWN;
                    continue;
                }

                let count = HyperWorld::neighbor_bug_count(world, above, below, x, y);
                let bug = world.has_bug(x, y);

                next.cells[World::index(x, y)] = match (bug, count) {
                    (true, 1) => BUG,
                    (false, 1) => BUG,
                    (false, 2) => BUG,
                    _ => CLEAR,
                }
            }
        }

        next
    }

    fn neighbor_bug_count(world: &World, above: &World, below: &World, x: i32, y: i32) -> usize {
        // Immediate neighbors
        let mut list = vec![(world, x + 1, y), (world, x - 1, y), (world, x, y + 1), (world, x, y - 1)]
            .into_iter()
            .filter(|(_, a, b)| (0..5).contains(a) && (0..5).contains(b))
            .map(|(w, a, b)| w.has_bug(a, b))
            .collect::<Vec<bool>>();

        // Edge neighbors to upper world.
        if x == 0 {
            list.push(above.has_bug(1, 2));
        } else if x == 4 {
            list.push(above.has_bug(3, 2));
        }

        if y == 0 {
            list.push(above.has_bug(2, 1));
        } else if y == 4 {
            list.push(above.has_bug(2, 3));
        }

        // Center neighbors to lower world.
        match (x, y) {
            (1, 2) => {
                list.append(&mut vec![
                    below.has_bug(0, 0),
                    below.has_bug(0, 1),
                    below.has_bug(0, 2),
                    below.has_bug(0, 3),
                    below.has_bug(0, 4),
                    ])
            }

            (2, 1) => {
                list.append(&mut vec![
                    below.has_bug(0, 0),
                    below.has_bug(1, 0),
                    below.has_bug(2, 0),
                    below.has_bug(3, 0),
                    below.has_bug(4, 0),
                    ])
            }

            (3, 2) => {
                list.append(&mut vec![
                    below.has_bug(4, 0),
                    below.has_bug(4, 1),
                    below.has_bug(4, 2),
                    below.has_bug(4, 3),
                    below.has_bug(4, 4),
                    ])
            }

            (2, 3) => {
                list.append(&mut vec![
                    below.has_bug(0, 4),
                    below.has_bug(1, 4),
                    below.has_bug(2, 4),
                    below.has_bug(3, 4),
                    below.has_bug(4, 4),
                    ])
            }

            _ => (),
        }

        list.iter().filter(|x| **x).count()
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.print_helper(0)
    }

    fn print_helper(&self, depth: u32) {
        match self {
            HyperWorld::Root(world, above, below) => {
                above.print_helper(1);
                print!("Depth 0:");
                world.print();
                println!();
                below.print_helper(1);
            }

            HyperWorld::UpperBranch(world, next) => {
                next.print_helper(depth + 1);
                print!("Depth -{}:", depth);
                world.print();
                println!();
            }

            HyperWorld::LowerBranch(world, next) => {
                print!("Depth {}:", depth);
                world.print();
                println!();
                next.print_helper(depth + 1);
            }

            HyperWorld::UpperEmpty => (),

            HyperWorld::LowerEmpty => (),
        }
    }
}

pub fn solve() {
    let initial = World::from_str("..#.#.#.####....#.#.###..");
    let mut past: HashSet<World> = HashSet::new();
    past.insert(initial.clone());

    let mut current = initial.clone();

    loop {
        current = current.step();
        // current.print();

        if past.contains(&current) {
            assert_eq(Day::new(24, Part::A), 18_401_265, current.biodiversity());
            break;
        } else {
            past.insert(current.clone());
        }
    }

    let mut current = HyperWorld::init(initial.clone());

    for _ in 0..200 {
        // current.print();
        current = current.step(None);
    }

    // current.print();

    assert_eq(Day::new(24, Part::B), 2078, current.bug_count());
}
