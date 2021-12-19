use crate::util;

#[derive(Debug, Clone, Copy)]
struct Point(u16, u16);
#[derive(Debug)]
struct Line(Point, Point);

pub(crate) fn calc() -> usize {
    let mut biggest_point = (0, 0);
    let content = util::read_file("input/day5.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let points = line
                .split("->")
                .map(str::trim)
                .map(|point| {
                    let coords = point
                        .split(",")
                        .map(|coord| coord
                            .parse::<u16>()
                            .expect("Failed to parse number"))
                        .collect::<Vec<u16>>();
                    if coords[0] > biggest_point.0 {
                        biggest_point.0 = coords[0];
                    }
                    if coords[1] > biggest_point.1 {
                        biggest_point.1 = coords[1];
                    }
                    Point(coords[0], coords[1])
                })
                .collect::<Vec<Point>>();
            Line(points[0], points[1])
        })
        .filter(|line| {
            line.0.0 == line.1.0 ||
                line.0.1 == line.1.1 ||
                (line.1.1 as i32 - line.0.1 as i32).abs() == (line.1.0 as i32 - line.0.0 as i32).abs()
        }).collect::<Vec<Line>>();

    let mut field: Vec<u8> = vec![0; (biggest_point.0 + 1) as usize * (biggest_point.1 + 1) as usize];
    content.iter().for_each(|line| {
        if line.0.0 == line.1.0 { // verticals
            for i in std::cmp::min(line.0.1, line.1.1)..=std::cmp::max(line.0.1, line.1.1) {
                field[(i as usize * biggest_point.0 as usize + line.0.0 as usize) as usize] += 1;
            }
        } else if line.0.1 == line.1.1 { // horizontals
            for i in std::cmp::min(line.0.0, line.1.0)..=std::cmp::max(line.0.0, line.1.0) {
                field[(i as usize + line.0.1 as usize * biggest_point.0 as usize) as usize] += 1;
            }
        } else { // diagonals
            if (line.1.0 > line.0.0) == (line.1.1 > line.0.1) { // up left to down right
                // iterating over x
                let start_x = std::cmp::min(line.0.0, line.1.0);
                for i in start_x..=std::cmp::max(line.0.0, line.1.0) {
                    let idx = (
                        i as usize + (
                            (
                                i as usize
                                    - start_x as usize
                                    + std::cmp::min(line.0.1, line.1.1) as usize
                            ) * biggest_point.0 as usize
                        )
                    ) as usize;
                    field[idx] += 1;
                }
            } else { // left down to up right
                // iterating over x
                let start_x = std::cmp::min(line.0.0, line.1.0);
                for i in start_x..=std::cmp::max(line.0.0, line.1.0) {
                    let idx = (
                        i as usize + (
                            (
                                std::cmp::max(line.0.1, line.1.1) as usize
                                - (i as usize - start_x as usize)
                            ) * biggest_point.0 as usize
                        )
                    ) as usize;
                    field[idx] += 1;
                }
            }
        }
    });

    // dump_debug("input/dump5.txt", biggest_point.0 as usize, field.as_slice());
    field.iter().filter(|val| **val > 1u8).count()
}