fn run_intcode(mut positions: Vec<i32>) -> i32 {
    let mut current = 0;

    loop {
        match positions[current] {
            1 => {
                let a = positions[current + 1] as usize;
                let b = positions[current + 2] as usize;
                let c = positions[current + 3] as usize;
                positions[c] = positions[a] + positions[b];
                current += 4;
            }

            2 => {
                let a = positions[current + 1] as usize;
                let b = positions[current + 2] as usize;
                let c = positions[current + 3] as usize;
                positions[c] = positions[a] * positions[b];
                current += 4;
            }

            99 => break,

            x => panic!("Invalid command {}", x),
        }
    }

    positions[0]
}

pub fn solve() {
    let mut positions = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 6, 19, 23, 1, 23, 13, 27,
        2, 6, 27, 31, 1, 5, 31, 35, 2, 10, 35, 39, 1, 6, 39, 43, 1, 13, 43, 47, 2, 47, 6, 51, 1,
        51, 5, 55, 1, 55, 6, 59, 2, 59, 10, 63, 1, 63, 6, 67, 2, 67, 10, 71, 1, 71, 9, 75, 2, 75,
        10, 79, 1, 79, 5, 83, 2, 10, 83, 87, 1, 87, 6, 91, 2, 9, 91, 95, 1, 95, 5, 99, 1, 5, 99,
        103, 1, 103, 10, 107, 1, 9, 107, 111, 1, 6, 111, 115, 1, 115, 5, 119, 1, 10, 119, 123, 2,
        6, 123, 127, 2, 127, 6, 131, 1, 131, 2, 135, 1, 10, 135, 0, 99, 2, 0, 14, 0,
    ];

    positions[1] = 12;
    positions[2] = 2;

    // 2782414
    println!("Day 2:A = {}", run_intcode(positions.clone()));

    for noun in 0..100 {
        for verb in 0..100 {
            positions[1] = noun;
            positions[2] = verb;

            if run_intcode(positions.clone()) == 19690720 {
                // 9820
                println!("Day 2:B = {}", 100 * noun + verb);
            }
        }
    }
}
