mod day1;
mod day2;
mod day3;
mod day4;
mod util;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

fn main() {
    let (day1m, day1m3win) = day1::calc();
    println!("Day 1 answer: {}, 3-win: {}", day1m, day1m3win);

    let day2m = day2::calc();
    println!("Day 2 answer: {}", day2m);

    let (day3m, day3mlife) = day3::calc();
    println!("Day 3 answer: {}, life rating: {}", day3m, day3mlife);

    day4::calc();

    let day5m = day5::calc();
    println!("Day 5: {}", day5m);

    let day6m = day6::calc();
    println!("Day 6: {}", day6m);

    let (day7m, day7m2) = day7::calc();
    println!("Day 7: {} / {}", day7m, day7m2);

    let day8m = day8::calc();
    println!("Day 8: {}", day8m);

    let day9m = day9::calc();
    println!("Day 9: {}", day9m);

    let day10m = day10::calc();
    println!("Day 10: {}", day10m);

    let (day11m, day11m2) = day11::calc();
    println!("Day 11: {} / {}", day11m, day11m2);
}
