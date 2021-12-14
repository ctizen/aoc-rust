use crate::util;

pub(crate) fn calc() -> u32 {
    let content = util::read_file("input/day3.txt");
    let bit_size = content.find("\n").unwrap();
    let stats = content.split("\n")
        .filter(|str| !str.is_empty())
        .map(|str| {
            let mut val: u32 = 0;
            let mut chars = str.chars().collect::<Vec<char>>();
            chars.reverse();
            for i in 0..bit_size {
                if chars[i] == '1' {
                    val |= 1 << i;
                }
            }
            val
        })
        .collect::<Vec<u32>>();
    let mut acc = vec![0; bit_size];
    let common_bit_stats = stats.iter().fold(&mut acc, |acc, val| {
        for i in 0..bit_size {
            acc[i] += if val & (1 << i) > 0 { 1 } else { 0 };
        }
        acc
    });

    let total_count = stats.len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..bit_size {
        if common_bit_stats[i] > total_count as u32 / 2 {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    gamma * epsilon
}