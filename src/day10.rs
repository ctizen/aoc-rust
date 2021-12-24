use crate::util;

pub(crate) fn calc() -> u32 {
    // ) , ] , } , >
    let scores = (3, 57, 1197, 25137);
    let mut found_total = (0, 0, 0, 0);
    util::read_file("input/day10.txt")
        .lines()
        .for_each(|line| {
            let mut stack: Vec<char> = Vec::new();
            let mut bracket_depth = 0;
            let mut line_failed = false;
            let mut found = (0, 0, 0, 0);
            for char in line.chars() {
                match char {
                    '{' | '[' | '<' | '(' => {
                        stack.push(char);
                        bracket_depth += 1;
                    },
                    ')' => {
                        bracket_depth -= 1;
                        if *stack.last().expect("Char not found") == '(' {
                            stack.pop();
                        } else {
                            if !line_failed {
                                found.0 += scores.0;
                            }
                            line_failed = true;
                        }
                    },
                    ']' => {
                        bracket_depth -= 1;
                        if *stack.last().expect("Char not found") == '[' {
                            stack.pop();
                        } else {
                            if !line_failed {
                                found.1 += scores.1;
                            }
                            line_failed = true;
                        }
                    },
                    '}' => {
                        bracket_depth -= 1;
                        if *stack.last().expect("Char not found") == '{' {
                            stack.pop();
                        } else {
                            if !line_failed {
                                found.2 += scores.2;
                            }
                            line_failed = true;
                        }
                    },
                    '>' => {
                        bracket_depth -= 1;
                        if *stack.last().expect("Char not found") == '<' {
                            stack.pop();
                        } else {
                            if !line_failed {
                                found.3 += scores.3;
                            }
                            line_failed = true;
                        }
                    },
                    _ => unreachable!()
                }
            }
            // if bracket_depth == 0 {
                found_total.0 += found.0;
                found_total.1 += found.1;
                found_total.2 += found.2;
                found_total.3 += found.3;
            // }
        });
    found_total.0 + found_total.1 + found_total.2 + found_total.3
}
