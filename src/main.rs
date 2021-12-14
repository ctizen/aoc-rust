mod day1;
mod util;
mod day2;
mod day3;

fn main() {
    let (day1m, day1m3win) = day1::calc();
    println!("Day 1 answer: {}, 3-win: {}", day1m, day1m3win);

    let day2m = day2::calc();
    println!("Day 2 answer: {}", day2m);

    let (day3m, day3mlife) = day3::calc();
    println!("Day 3 answer: {}, life rating: {}", day3m, day3mlife);
}
