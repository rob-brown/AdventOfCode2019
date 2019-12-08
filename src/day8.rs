use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
    let mut file = File::open("input/day8.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const PIXELS: usize = WIDTH * HEIGHT;

    let mut chunks = contents
        .chars()
        .collect::<Vec<char>>()
        .chunks(PIXELS)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();

    // Remove the newline
    chunks.pop();

    let mut minimum = 1000;
    let mut product = 0;

    for chunk in chunks.iter() {
        let mut zeroes = 0;
        let mut ones = 0;
        let mut twos = 0;

        for char in chunk.chars() {
            match char {
                '0' => zeroes += 1,

                '1' => ones += 1,

                '2' => twos += 1,

                x => panic!("Unknown pixel {}", x),
            }
        }

        if zeroes < minimum {
            minimum = zeroes;
            product = ones * twos;
        }
    }

    println!("Day 8:A = {}", product);

    let mut frame_buffer: [char; PIXELS] = ['2'; PIXELS];

    for chunk in chunks.iter() {
        for (index, char) in chunk.chars().enumerate() {
            if frame_buffer[index] == '2' {
                frame_buffer[index] = char;
            }
        }
    }

    for i in 0..PIXELS {
        if i % WIDTH == 0 {
            println!();
        }

        match frame_buffer[i] {
            // '0' => print!("_"),
            '1' => print!("O"),
            _ => print!(" "),
        }
    }

    println!("");
    println!("Day 8:B = HFYAK");
}
