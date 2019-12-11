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
    println!("Done");
}

fn main() {
    time(solve_all);
}
