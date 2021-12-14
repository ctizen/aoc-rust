use crate::util;

fn make_stat_single_bit(input: &Vec<u32>, bit_index: usize) -> u32 {
    input.iter().fold(0, |acc, val| {
        acc + if val & (1 << bit_index) > 0 { 1 } else { 0 }
    })
}

pub(crate) fn calc() -> (u32, u32) {
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

    let mut oxy_search: Vec<u32> = stats.clone();
    let mut co_search: Vec<u32> = stats.clone();
    let mut oxy_count = total_count;
    let mut co_count = total_count;
    for i in (0..bit_size).rev() {
        let bit_oxy = make_stat_single_bit(&oxy_search, i);
        if oxy_search.len() > 1 {
            let oxy_search_tmp = oxy_search.iter()
                .map(|val| *val)
                .filter(|val| (val & (1 << i) > 0) == (2 * bit_oxy >= oxy_count as u32))
                .collect::<Vec<u32>>();
            if oxy_search_tmp.len() > 0 {
                oxy_search = oxy_search_tmp;
                oxy_count = oxy_search.len();
            }
        }

        let bit_co = make_stat_single_bit(&co_search, i);
        if co_search.len() > 1 {
            let co_search_tmp = co_search.iter()
                .map(|val| *val)
                .filter(|val| (val & (1 << i) > 0) == (2 * bit_co < co_count as u32))
                .collect::<Vec<u32>>();
            if co_search_tmp.len() > 0 {
                co_search = co_search_tmp;
                co_count = co_search.len();
            }
        }

        println!("----------------------------------------");
        for i in &co_search {
            println!("{:b}", i);
        }
    }

    (gamma * epsilon, co_search.iter().next().unwrap() * oxy_search.iter().next().unwrap())
}
