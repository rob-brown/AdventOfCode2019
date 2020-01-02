use super::assert::*;
use std::fs::File;
use std::io::prelude::*;
use std::iter;
use std::mem::swap;

pub fn solve() {
    let mut file = File::open("input/day16.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let list: Vec<i32> = contents
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();
    let length = list.len();
    let mut current = list.clone();

    for _ in 0..100 {
        let mut next = Vec::with_capacity(length);

        for (n, _) in current.iter().enumerate() {
            let mut sum = 0;
            let zero = iter::repeat(0).take(n + 1);
            let one = iter::repeat(1).take(n + 1);
            let neg_one = iter::repeat(-1).take(n + 1);
            let mut sequence = zero
                .clone()
                .chain(one)
                .chain(zero)
                .chain(neg_one)
                .cycle()
                .skip(1);

            for y in current.iter() {
                sum += y * sequence.next().unwrap();
            }
            next.push(sum.abs() % 10);
        }

        current = next;
    }

    let answer = current[0..8]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    assert_eq(Day::new(16, Part::A), "23135243", &answer);

    let offset = contents[0..7].parse::<usize>().unwrap();
    let remainder = length * 10_000 - offset;
    let mut buffer = vec![0; remainder];
    let mut temp = vec![0; remainder];
    let mut index = remainder - 1;

    // The end of the phase matrix looks like this:
    //
    //   1 1 1 1 1 1 1 1
    //   0 1 1 1 1 1 1 1
    //   0 0 1 1 1 1 1 1
    //   0 0 0 1 1 1 1 1
    //   0 0 0 0 1 1 1 1
    //   0 0 0 0 0 1 1 1
    //   0 0 0 0 0 0 1 1
    //   0 0 0 0 0 0 0 1
    //
    // So the multiplication is irrelevant.
    // Start by creating the end of the sequence.
    'outer: loop {
        for i in (0..length).rev() {
            buffer[index] = list[i];
            if index == 0 {
                break 'outer;
            }
            index -= 1;
        }
    }

    // Next get the sum of each row.
    // The sum can be efficiently computed starting from the last row.
    for _ in 0..100 {
        let mut sum: u64 = 0;

        for i in (0..remainder).rev() {
            sum += buffer[i] as u64;
            temp[i] = (sum % 10) as i32;
        }

        swap(&mut buffer, &mut temp);
    }

    let result = buffer[0..8]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    assert_eq(Day::new(16, Part::B), "21130597", &result);
}
