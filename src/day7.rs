use crate::util;

pub(crate) fn calc() -> (u32, u32) {
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
    let var1 = content.iter().fold(0, |acc, val| acc + (*val as i32 - median as i32).abs() as u32);

    let avg = (content.iter().fold(0, |acc, val| acc + val) / content.len() as u32) as u32;
    let var2 = content.iter().fold(0, |acc, val| acc + factorize((*val as i32 - avg as i32).abs() as u32));

    (var1, var2)
}

fn factorize(val: u32) -> u32 {
    ((1 + val) as f64 * ((val as f64) / 2f64)).round() as u32
}