use super::assert::*;
use std::fs::File;
use std::io::prelude::*;
use std::iter;

pub fn solve() {
    let mut file = File::open("input/day16.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // let mut contents = "80871224585914546619083218645595";
    let list: Vec<i32> = contents
        .trim()
        .chars()
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect();
    let length = list.len();
    let mut current = list;

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

    let answer = current
        .iter()
        .cloned()
        .take(8)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    assert_eq(Day::new(16, Part::A), "23135243", &answer);
}
