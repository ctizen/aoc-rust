use crate::util;

pub(crate) fn calc() -> u32 {
    let content = util::read_file("input/day7.txt")
        .lines()
        .next()
        .expect("Failed to read content")
        .split(",")
        .map(|n| n.parse::<u32>().expect("Failed to parse number"))
        .collect::<Vec<u32>>();

    let mut sorted = content.clone();
    sorted.sort();

    let median = sorted[sorted.len() / 2];
    content.iter().fold(0, |acc, val| acc + (*val as i32 - median as i32).abs() as u32)
}