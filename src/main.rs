use std::time::Instant;

mod aoc;

fn time<F>(f: F)
where
    F: Fn(),
{
    let start = Instant::now();
    f();
    let duration = Instant::now().duration_since(start);
    println!("Ran in {:?}\n", duration);
}

fn solve_all() {
    time(aoc::day1::solve);
    time(aoc::day2::solve);
    time(aoc::day3::solve);
    time(aoc::day4::solve);
    time(aoc::day5::solve);
    time(aoc::day6::solve);
    time(aoc::day7::solve);
    time(aoc::day8::solve);
    time(aoc::day9::solve);
    time(aoc::day11::solve);
    time(aoc::day12::solve);
    time(aoc::day13::solve);
    time(aoc::day15::solve);
    time(aoc::day16::solve);
    time(aoc::day17::solve);
    time(aoc::day19::solve);
    time(aoc::day23::solve);
    time(aoc::day24::solve);
    // time(aoc::day25::solve);
    println!("Done");
}

fn main() {
    time(solve_all);
}
