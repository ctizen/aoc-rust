use crate::util;

pub(crate) fn calc() -> i64 {
    let list = util::read_file("input/day2.txt");
    let folding = list
        .split("\n")
        .filter(|str| !str.is_empty())
        .map(|cmd| {
            let pieces = cmd.split(" ").collect::<Vec<&str>>();
            let val_int = pieces[1].parse::<i64>().expect("Failed to parse number");
            match pieces[0] {
                "forward" => (0, val_int),
                "up" => (-val_int, 0),
                "down" => (val_int, 0),
                _ => (0, 0),
            }
        })
        .fold((0, 0, 0), |acc, tuple| {
            (
                acc.0 + (tuple.1 * (acc.2 + tuple.0)),
                acc.1 + tuple.1,
                acc.2 + tuple.0,
            )
        });
    folding.0 as i64 * folding.1 as i64
}
