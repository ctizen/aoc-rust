use crate::util;

type Hits = Vec<u32>;
#[derive(Debug)]
struct BoardRow(Vec<u32>);
#[derive(Debug)]
struct Board(Vec<BoardRow>);

pub(crate) fn calc() -> u32 {
    let content = util::read_file("input/day4.txt");
    let (hits, boards) = parse(content);
    println!("{:?}", hits);
    println!("{:?}", boards);
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
            let mut outer_it = board.iter().map(|row| {
                let mut it = row
                    .split_whitespace()
                    .filter(|str| !str.is_empty())
                    .map(|val| val.trim().parse::<u32>().expect("Failed to parse number"));
                BoardRow(it.collect::<Vec<u32>>())
            });
            Board(outer_it.collect::<Vec<BoardRow>>())
        })
        .collect::<Vec<Board>>();
    return (hits, boards);
}
