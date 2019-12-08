use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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
    time(day1::solve);
    time(day2::solve);
    time(day3::solve);
    time(day4::solve);
    time(day5::solve);
    time(day6::solve);
    time(day7::solve);
    println!("Done");
}

fn main() {
    time(solve_all);
}
