use super::assert::*;
use super::intcode::Machine;
use std::collections::HashMap;

const SCREEN_WIDTH: i32 = 44;
const SCREEN_HEIGHT: i32 = 23;

struct GameInfo {
    block_count: i32,
    ball_position: (i32, i32),
    paddle_position: (i32, i32),
}

impl GameInfo {
    fn zero() -> Self {
        GameInfo::new(0, (0, 0), (0, 0))
    }

    fn new(block_count: i32, ball_position: (i32, i32), paddle_position: (i32, i32)) -> Self {
        Self {
            block_count,
            ball_position,
            paddle_position,
        }
    }
}

fn game_info(screen: &HashMap<(i32, i32), i32>) -> GameInfo {
    let mut info = GameInfo::zero();

    for ((x, y), z) in screen.iter() {
        match z {
            2 => info.block_count += 1,
            3 => info.paddle_position = (*x, *y),
            4 => info.ball_position = (*x, *y),
            _ => continue,
        }
    }

    info
}

fn block_count(screen: &HashMap<(i32, i32), i32>) -> i32 {
    screen.iter().filter(|(_, x)| **x == 2).count() as i32
}

fn is_gameover(screen: &HashMap<(i32, i32), i32>) -> bool {
    block_count(&screen) == 0
}

#[allow(dead_code)]
fn reset_screen() {
    print!("\x1B[{}D\x1B[{}A", SCREEN_WIDTH + 1, SCREEN_HEIGHT + 1);
}

#[allow(dead_code)]
fn clear_screen() {
    for _ in 0..(SCREEN_HEIGHT + 1) {
        print!("\x1B[1A\x1B[2K");
    }
}

#[allow(dead_code)]
fn print_screen(screen: &HashMap<(i32, i32), i32>) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            match screen.get(&(x, y)).unwrap_or(&0) {
                // Wall
                1 => print!("#"),

                // Block
                2 => print!("O"),

                // Paddle
                3 => print!("="),

                // Ball
                4 => print!("*"),

                _ => print!(" "),
            }
        }
        println!();
    }

    println!();
}

pub fn solve() {
    let initial = Machine::from_file("input/day13.txt");
    let mut screen: HashMap<(i32, i32), i32> = HashMap::new();
    let mut machine = Machine::init(&initial.positions);

    while machine.halted == false {
        machine.run(vec![]);

        while machine.values.is_empty() == false {
            let x = machine.values.pop().unwrap() as i32;
            let y = machine.values.pop().unwrap() as i32;
            let z = machine.values.pop().unwrap() as i32;

            screen.insert((x, y), z);
        }
    }

    let count = screen.iter().filter(|(_, x)| **x == 2).count();

    assert_eq(Day::new(12, Part::A), 414, count);

    let mut screen: HashMap<(i32, i32), i32> = HashMap::new();
    let mut machine = Machine::init(&initial.positions);
    machine.positions[0] = 2;
    let mut score = 0;
    let mut joystick_position = 0;

    while machine.halted == false {
        machine.run(vec![joystick_position]);

        while machine.values.is_empty() == false {
            let x = machine.values.pop().unwrap() as i32;
            let y = machine.values.pop().unwrap() as i32;
            let z = machine.values.pop().unwrap() as i32;

            if x == -1 && y == 0 {
                score = z;
            } else {
                screen.insert((x, y), z);
            }
        }

        let info = game_info(&screen);

        if info.ball_position.0 > info.paddle_position.0 {
            joystick_position = 1;
        } else if info.ball_position.0 < info.paddle_position.0 {
            joystick_position = -1;
        } else {
            joystick_position = 0;
        }

        // print_screen(&screen);
        // let delay = std::time::Duration::from_millis(5);
        // std::thread::sleep(delay);
        // clear_screen();

        if is_gameover(&screen) {
            break;
        }
    }

    assert_eq(Day::new(13, Part::B), 20_183, score);
}
