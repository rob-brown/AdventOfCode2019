use super::assert::*;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

const CARD_COUNT: i64 = 10_007;

enum Shuffle {
    Cut(i64),
    Increment(i64),
    Reverse,
}

fn track_position(mut index: i64, commands: &Vec<Shuffle>) -> i64 {
    for c in commands.iter() {
        match c {
            Shuffle::Cut(count) => index = (index - count) % CARD_COUNT,

            Shuffle::Increment(count) => index = (index * count).rem_euclid(CARD_COUNT),

            Shuffle::Reverse => index = CARD_COUNT - index - 1,
        }
    }

    index
}

pub fn solve() {
    let mut file = File::open("input/day22.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let regex = Regex::new(r"([a-z][ a-z]+[a-z]) ?(-?\d*)\n").unwrap();

    let mut commands: Vec<Shuffle> = Vec::new();

    for capture in regex.captures_iter(&contents) {
        let command = capture[1].parse::<String>().unwrap();
        let number = capture[2].parse::<String>().unwrap();

        match command.as_str() {
            "cut" => commands.push(Shuffle::Cut(number.parse::<i64>().unwrap())),

            "deal with increment" => commands.push(Shuffle::Increment(number.parse::<i64>().unwrap())),

            "deal into new stack" => commands.push(Shuffle::Reverse),

            _ => panic!(),
        }
    }

    assert_eq(Day::new(22, Part::A), 5755, track_position(2019, &commands));
}
