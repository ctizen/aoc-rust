use crate::util;

type Hits = Vec<u32>;
#[derive(Debug, Clone)]
struct BoardRow(Vec<(u32, bool)>);
#[derive(Debug, Clone)]
struct Board(Vec<BoardRow>);

pub(crate) fn calc() -> u32 {
    let content = util::read_file("input/day4.txt");
    let (hits, mut boards) = parse(content);

    let mut current_num = 0u32;
    let mut last_board = 0;
    for i in &hits {
        mark_numbers(&mut boards, &i);
        match get_unwinning_board(&boards) {
            Some(val) => {
                current_num = *i;
                last_board = val;
            },
            None => {
                break
            }
        }
    }

    println!("{:#?}", last_board);

    let lastnum = hits[hits.iter().position(|val| *val == current_num).unwrap() + 1];
    println!("{} {}", lastnum, count_score(&boards[last_board], lastnum));
    0
}

fn parse(input: String) -> (Hits, Vec<Board>) {
    let rows = input
        .lines()
        .filter(|str| !str.is_empty())
        .collect::<Vec<&str>>();

    let hits_str = &rows[0];
    let hits = hits_str
        .split(",")
        .map(|str| str.parse::<u32>().expect("Failed to parse number"))
        .collect::<Vec<u32>>();

    let boards = rows[1..]
        .chunks(5)
        .map(|board| {
            let outer_it = board.iter().map(|row| {
                let it = row
                    .split_whitespace()
                    .filter(|str| !str.is_empty())
                    .map(|val| (val.trim().parse::<u32>().expect("Failed to parse number"), false));
                BoardRow(it.collect::<Vec<(u32, bool)>>())
            });
            Board(outer_it.collect::<Vec<BoardRow>>())
        })
        .collect::<Vec<Board>>();
    return (hits, boards);
}

fn mark_numbers(boards: &mut Vec<Board>, target: &u32) -> () {
    boards.iter_mut().for_each(|board| {
        board.0.iter_mut().for_each(|row| {
            row.0.iter_mut().for_each(|val| {
                if val.0 == *target {
                    val.1 = true;
                }
            })
        })
    })
}

fn get_unwinning_board(boards: &Vec<Board>) -> Option<usize> {
    boards.iter().rposition(|board| {
        board.0.iter().all(|row| {
            row.0.iter().any(|val| val.1 == false)
        }) && {
            let mut init = vec![0; board.0.len()];
            let cols_marked = board.0.iter().fold(&mut init, |acc, row| {
                for i in 0..row.0.len() {
                    acc[i] += if row.0[i].1 == true { 1 } else { 0 };
                }
                acc
            });
            cols_marked.iter().all(|col| *col != board.0.len())
        }
    })
}

fn count_score(board: &Board, last_number: u32) -> u32 {
    let sum = board.0.iter().fold(0, |acc, row| {
        acc + row.0.iter().fold(0, |acc, val| if !val.1 { acc + val.0 } else { acc })
    });
    sum * last_number
}