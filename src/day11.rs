use crate::util;

struct Octopus {
    energy: u8,
    flash: bool
}

trait Flashing {
    fn accumulate(&mut self) -> bool;
}

impl Flashing for Octopus {
    fn accumulate(&mut self) -> bool {
        if self.flash {
            return false;
        }
        self.energy += 1;
        return self.energy > 9;
    }
}

pub(crate) fn calc() -> (u32, u32) {
    let mut octopi = util::read_file("input/day11.txt")
        .lines()
        .filter(|str| !str.is_empty())
        .map(|line| line
            .chars()
            .map(|char| Octopus {
                energy: char.to_digit(10).unwrap() as u8,
                flash: false
            })
            .collect::<Vec<Octopus>>()
        )
        .collect::<Vec<Vec<Octopus>>>();

    let mut count = 0;
    let mut sync_step = 0;
    for _i in 0..100 {
        count += run_step(&mut octopi);
    }
    for i in 1..1000 {
        if run_step(&mut octopi) as usize == octopi.len() * octopi[0].len() {
            sync_step = i + 100;
            break;
        }
    }
    (count, sync_step)
}

fn run_step(octopi: &mut Vec<Vec<Octopus>>) -> u32 /* flash count */ {
    // 1) flash
    for i in 0..octopi.len() {
        for j in 0..octopi[i].len() {
            if octopi[i][j].accumulate() {
                flash(octopi, i as i32, j as i32);
            }
        }
    }

    // 2) count flashes and recharge
    octopi
        .iter_mut()
        .map(|row| row
            .iter_mut()
            .map(|oct| {
                if oct.flash {
                    oct.energy = 0;
                    oct.flash = false;
                    1
                } else {
                    0
                }
            }).sum::<u32>()
        ).sum::<u32>()
}

fn flash(octopi: &mut Vec<Vec<Octopus>>, i: i32, j: i32) {
    if octopi[i as usize][j as usize].energy > 9 && !octopi[i as usize][j as usize].flash {
        octopi[i as usize][j as usize].flash = true;
        for xi in i-1..=i+1 {
            for yi in j-1..=j+1 {
                if xi >= 0 && xi < octopi.len() as i32 && yi >= 0 && yi < octopi[xi as usize].len() as i32 {
                    if octopi[xi as usize][yi as usize].accumulate() {
                        flash(octopi, xi, yi);
                    }
                }
            }
        }
    }
}