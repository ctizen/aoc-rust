use crate::util;

pub(crate) fn calc() -> usize {
    let mut nums = util::read_file("input/day6.txt")
        .lines()
        .next()
        .expect("Failed to read string")
        .split(",")
        .map(|num| num.parse::<i32>().expect("Failed to parse number"))
        .collect::<Vec<i32>>();

    for _i in 0..80 {
        next_day(&mut nums);
    }

    nums.len()
}

fn next_day(state: &mut Vec<i32>) {
    let len = state.len();
    for i in 0..len {
        state[i] -= 1;
        if state[i] < 0 {
            state[i] = 6;
            state.push(8);
        }
    }
}