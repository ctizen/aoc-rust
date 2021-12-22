use crate::util;

pub(crate) fn calc() -> u32 {
    util::read_file("input/day8.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut it = line.split(" | ");

            let codes: Vec<String> = it
                .next()
                .expect("Failed to parse line")
                .split_whitespace()
                .map(|word| sort_string(String::from(word)))
                .collect();
            let map = detect_codes(codes);

            let decoded = it.last()
                .expect("Failed to parse line")
                .split_whitespace()
                .map(|str| sort_string(String::from_iter(str
                    .chars()
                    .map(|char| map[code_by_wire(char)])
                    .collect::<Vec<char>>())))
                .collect::<Vec<String>>();

            let nums: Vec<u8> = decoded.iter().map(|str| match str.as_str() {
                "abcefg" => 0,
                "cf" => 1,
                "acdeg" => 2,
                "acdfg" => 3,
                "bcdf" => 4,
                "abdfg" => 5,
                "abdefg" => 6,
                "acf" => 7,
                "abcdefg" => 8,
                "abcdfg" => 9,
                _ => unreachable!()
            }).collect();

            nums[0] as u32 * 1000 +
            nums[1] as u32 * 100 +
            nums[2] as u32 * 10 +
            nums[3] as u32 * 1
        })
        .fold(0, |acc, num| {
            acc + num
        })
}

fn sort_string(input: String) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    return String::from_iter(chars);
}

fn code_by_wire(wire: char) -> usize {
    match wire {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => 7
    }
}

fn wire_by_code(code: usize) -> char {
    match code {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        _ => '?'
    }
}

fn detect_codes(input: Vec<String>) -> Vec<char> {
    let counts = input.iter().fold(vec![0; 8], |mut acc, str| {
        str.chars().for_each(|ch| acc[code_by_wire(ch)] += 1);
        acc
    });

    let bwire = counts.iter().position(|cnt| *cnt == 6).expect("Not found wire");
    let ewire = counts.iter().position(|cnt| *cnt == 4).expect("Not found wire");
    let fwire = counts.iter().position(|cnt| *cnt == 9).expect("Not found wire");

    let acwire = (
        counts.iter().position(|cnt| *cnt == 8).expect("Not found wire"),
        counts.iter().rposition(|cnt| *cnt == 8).expect("Not found wire"),
    );

    let dgwire = (
        counts.iter().position(|cnt| *cnt == 7).expect("Not found wire"),
        counts.iter().rposition(|cnt| *cnt == 7).expect("Not found wire"),
    );

    let mut cwire: usize = 7;
    let mut awire: usize = 7;
    let mut dwire: usize = 7;
    let mut gwire: usize = 7;

    for x in input {
        match x.len() {
            2 => { // discriminate a-c wire with "1" digit containing 2 wires
                if x.contains(wire_by_code(acwire.0)) {
                    cwire = acwire.0;
                    awire = acwire.1;
                } else {
                    cwire = acwire.1;
                    awire = acwire.0;
                }
            },
            4 => { // discriminate a-c wire with "4" digit containing 4 wires
                if x.contains(wire_by_code(dgwire.0)) {
                    dwire = dgwire.0;
                    gwire = dgwire.1;
                } else {
                    dwire = dgwire.1;
                    gwire = dgwire.0;
                }
            },
            _ => ()
        }
    }

    let mut ret_vec = vec![' '; 7];
    ret_vec[awire] = 'a';
    ret_vec[bwire] = 'b';
    ret_vec[cwire] = 'c';
    ret_vec[dwire] = 'd';
    ret_vec[ewire] = 'e';
    ret_vec[fwire] = 'f';
    ret_vec[gwire] = 'g';
    ret_vec
}
