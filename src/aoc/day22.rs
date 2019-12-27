use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

const CARD_COUNT: isize = 10_007;

fn track_position<T>(index: isize, command: &str, number: Result<isize, T>) -> isize {
    match (command, number) {
        ("cut", Ok(count)) => (index - count) % CARD_COUNT,

        ("deal with increment", Ok(count)) => (index * count).rem_euclid(CARD_COUNT),

        ("deal into new stack", _) => CARD_COUNT - index - 1,

        _ => panic!(),
    }
}

pub fn solve() {
    let mut file = File::open("input/day22.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let regex = Regex::new(r"([a-z][ a-z]+[a-z]) ?(-?\d*)\n").unwrap();

    let mut index = 2019;

    for capture in regex.captures_iter(&contents) {
        let command = capture[1].parse::<String>().unwrap();
        let number = capture[2].parse::<String>().unwrap();

        index = track_position(index, command.as_str(), number.parse::<isize>());
    }

    assert_eq(Day::new(22, Part::A), 5755, index);
}
