use crate::util;

pub(crate) fn calc() -> u32 {
    let content = util::read_file("input/day8.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line
            .split(" | ")
            .last()
            .expect("Failed to parse line")
            .split_whitespace()
            .map(|word| word.len())
            .collect::<Vec<usize>>()
        )
        .fold(vec![0; 8], |mut acc, num| {
            acc[num] += 1;
            acc
        });
    content[2] + content[3] + content[4] + content[7]
}
