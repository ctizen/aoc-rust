use std::collections::VecDeque;
use crate::util;

pub(crate) fn calc() -> u128 {
    let nums = util::read_file("input/day6.txt")
        .lines()
        .next()
        .expect("Failed to read string")
        .split(",")
        .map(|num| num.parse::<usize>().expect("Failed to parse number"))
        .collect::<Vec<usize>>();

    let mut counts: VecDeque<u128> = nums.iter()
        .fold(VecDeque::from(vec![0; 9]), |mut acc, num| {
            acc[*num] += 1;
            acc
        });

    for _i in 0..256 {
        next_day(&mut counts);
    }

    counts.iter().fold(0, |acc, num| acc + num)
}

fn next_day(state: &mut VecDeque<u128>) {
    let overflowed = state.pop_front().expect("Wat");
    state.push_back(overflowed);
    state[6] += overflowed;
}