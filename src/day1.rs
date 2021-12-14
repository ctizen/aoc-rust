use crate::util;

pub(crate) fn calc() -> (u32, u32) {
    let content = util::read_file("input/day1.txt");
    let pieces = content.split("\n")
        .filter(|str| !str.is_empty())
        .map(|str| str.parse::<u32>().expect("Failed to parse number"))
        .collect::<Vec<u32>>();
    let mut measures = 0;
    let mut measures3window = 0;
    for i in 1..pieces.len() {
        if pieces[i] > pieces[i - 1] {
            measures += 1;
        }
    }
    for i in 0..pieces.len() - 3 {
        if pieces[i + 3] > pieces[i] {
            measures3window += 1;
        }
    }
    (measures, measures3window)
}
