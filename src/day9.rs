use crate::util;

pub(crate) fn calc() -> u32 {
    let content = util::read_file("input/day9.txt")
        .lines()
        .map(|str| String::from(str))
        .collect::<Vec<String>>();
    let row_size = content[0].len();
    let mut nums = content.iter().flat_map(|str| str
        .chars()
        .map(|char| match char {
            '0' => 0, '1' => 1, '2' => 2, '3' => 3, '4' => 4, '5' => 5,
            '6' => 6, '7' => 7, '8' => 8, '9' => 9, _ => unreachable!()
        }).collect::<Vec<u8>>()
    ).collect::<Vec<u8>>();

    let mut mins = Vec::new();
    for i in 0..nums.len() {
        if is_min(&nums, row_size, i) {
            mins.push(i);
        }
    }

    let mut sizes = mins
        .iter()
        .map(|idx| find_basin(&mut nums, row_size, *idx))
        .collect::<Vec<u32>>();
    sizes.sort_by(|a, b| b.cmp(a));

    sizes[0] * sizes[1] * sizes[2]
}

fn is_min(set: &Vec<u8>, row_size: usize, index: usize) -> bool {
    let i1 = index as i32 - 1; // left
    let i2 = index as i32 + 1; // right
    let i3 = index as i32 + row_size as i32; // down
    let i4 = index as i32 - row_size as i32; // up

    let mut is_min = true;
    if i1 > 0 && index as i32 % row_size as i32 != 0 {
        is_min = is_min && set[index] < set[i1 as usize];
    }
    if i2 < set.len() as i32 && ((index as i32 + 1) % row_size as i32) != 0 {
        is_min = is_min && set[index] < set[i2 as usize];
    }
    if i3 < set.len() as i32 {
        is_min = is_min && set[index] < set[i3 as usize];
    }
    if i4 > 0 {
        is_min = is_min && set[index] < set[i4 as usize];
    }
    is_min
}

fn find_basin(set: &mut Vec<u8>, row_size: usize, index: usize) -> u32 {
    let i1 = index as i32 - 1; // left
    let i2 = index as i32 + 1; // right
    let i3 = index as i32 + row_size as i32; // down
    let i4 = index as i32 - row_size as i32; // up

    let mut size: u32 = 1; // count current field
    set[index] = 0;
    if i1 > 0 && index as i32 % row_size as i32 != 0 && set[i1 as usize] > 0 && set[i1 as usize] < 9 {
        size += find_basin(set, row_size, i1 as usize);
    }
    if i2 < set.len() as i32 && ((index as i32 + 1) % row_size as i32) != 0 && set[i2 as usize] > 0 && set[i2 as usize] < 9 {
        size += find_basin(set, row_size, i2 as usize);
    }
    if i3 < set.len() as i32 && set[i3 as usize] > 0 && set[i3 as usize] < 9 {
        size += find_basin(set, row_size, i3 as usize);
    }
    if i4 > 0 && set[i4 as usize] > 0 && set[i4 as usize] < 9 {
        size += find_basin(set, row_size, i4 as usize);
    }

    size
}