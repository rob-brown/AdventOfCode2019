use super::assert::*;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

const CARD_COUNT: i64 = 10_007;
const BIG_CARD_COUNT: i64 = 119_315_717_514_047;
const SHUFFLE_COUNT: i64 = 101_741_582_076_661;

enum Shuffle {
    Cut(i64),
    Increment(i64),
    Reverse,
}

fn track_position(mut index: i64, commands: &Vec<Shuffle>, modulus: i64) -> i64 {
    for c in commands.iter() {
        match c {
            Shuffle::Cut(count) => index = (index - count) % modulus,

            Shuffle::Increment(count) => index = (index * count).rem_euclid(modulus),

            Shuffle::Reverse => index = modulus - index - 1,
        }
    }

    index
}

// Modular multiplicative inverse
fn modinv(a: i64, modulus: i64) -> i64 {
    return modpow(a, modulus - 2, modulus);
}

// Modular exponentiation
fn modpow(base: i64, exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let modulus: i128 = modulus as i128;
    let test: i128 = (modulus - 1) * (modulus - 1);
    assert!(test > 0);

    let mut base: i128 = (base as i128) % (modulus as i128);
    let mut result: i128 = 1;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    result as i64
}

fn poly(x: i64, commands: &Vec<Shuffle>) -> i64 {
    let x = x as i128;
    let p0 = track_position(0, commands, BIG_CARD_COUNT);
    let p1 = track_position(1, commands, BIG_CARD_COUNT);
    let a1 = (p1 - p0).rem_euclid(BIG_CARD_COUNT);
    let b1: i128 = p0 as i128;
    let modulus: i128 = BIG_CARD_COUNT as i128;
    let a_t: i128 = modinv(a1, BIG_CARD_COUNT) as i128;
    let b_t: i128 = (-a_t * b1).rem_euclid(modulus);

    let temp: i128 = modpow(a_t as i64, SHUFFLE_COUNT, BIG_CARD_COUNT) as i128;
    let inverse: i128 = modinv(a_t as i64 - 1, BIG_CARD_COUNT) as i128;

    ((temp * x + ((((temp - 1) * inverse % modulus) * b_t) % modulus)) % modulus) as i64
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

            "deal with increment" => {
                commands.push(Shuffle::Increment(number.parse::<i64>().unwrap()))
            }

            "deal into new stack" => commands.push(Shuffle::Reverse),

            _ => panic!(),
        }
    }

    assert_eq(Day::new(22, Part::A), 5755, track_position(2019, &commands, CARD_COUNT));
    assert_eq(Day::new(22, Part::B), 42152620178084, poly(2020, &commands));
}
