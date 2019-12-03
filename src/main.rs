use std::time::Instant;

mod day1;
mod day2;
mod day3;

fn main() {
    let start = Instant::now();

    day1::solve();
    day2::solve();
    day3::solve();

    let duration = Instant::now().duration_since(start);
    println!();
    println!("Ran in {:?}", duration);
}
