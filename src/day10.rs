use crate::util;

pub(crate) fn calc() -> u128 {
    let mut scores: Vec<u128> = Vec::new();
    util::read_file("input/day10.txt")
        .lines()
        .for_each(|line| {
            let mut stack: Vec<char> = Vec::new();
            let mut score = 0;
            let mut processed_chars = 0;
            for char in line.chars() {
                processed_chars += 1;
                match char {
                    '{' | '[' | '<' | '(' => {
                        stack.push(char);
                    },
                    ')' => {
                        if *stack.last().expect("Char not found") == '(' {
                            stack.pop();
                        } else {
                            break;
                        }
                    },
                    ']' => {
                        if *stack.last().expect("Char not found") == '[' {
                            stack.pop();
                        } else {
                            break;
                        }
                    },
                    '}' => {
                        if *stack.last().expect("Char not found") == '{' {
                            stack.pop();
                        } else {
                            break;
                        }
                    },
                    '>' => {
                        if *stack.last().expect("Char not found") == '<' {
                            stack.pop();
                        } else {
                            break;
                        }
                    },
                    _ => unreachable!()
                }
            }

            if processed_chars == line.len() {
                stack.reverse();
                stack.iter().for_each(|char| match *char {
                    '(' => score = score * 5 + 1,
                    '[' => score = score * 5 + 2,
                    '{' => score = score * 5 + 3,
                    '<' => score = score * 5 + 4,
                    _ => unreachable!()
                });
                scores.push(score);
            }
        });
    scores.sort();
    scores[scores.len() / 2]
}
