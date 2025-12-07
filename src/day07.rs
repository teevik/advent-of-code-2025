use core::str;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    ops::RangeInclusive,
};

use color_eyre::eyre::Result;
use itertools::Itertools;
use rangemap::RangeInclusiveSet;

use crate::fetch_input;

fn part_1(input: &str) -> Result<usize> {
    let input = input.trim();

    let mut grid = input.lines().map(|line| line.as_bytes()).collect_vec();

    let mut start = None;

    for (x, &cell) in grid[0].iter().enumerate() {
        if cell == b'S' {
            start = Some(x);
        }
    }

    dbg!(start);

    let start_x = start.unwrap();
    let mut tachyon_beams = HashSet::new();
    tachyon_beams.insert(start_x);

    let mut splits = 0;

    for y in 1..grid.len() {
        let row = &grid[y];

        for beam in tachyon_beams.clone() {
            if row[beam] == b'^' {
                splits += 1;
                tachyon_beams.remove(&beam);
                tachyon_beams.insert(beam - 1);
                tachyon_beams.insert(beam + 1);
            }
        }
    }

    dbg!(tachyon_beams.len());

    Ok(splits)
}

fn part_2(input: &str) -> Result<usize> {
    let input = input.trim();

    let mut grid = input.lines().map(|line| line.as_bytes()).collect_vec();

    let mut start = None;

    for (x, &cell) in grid[0].iter().enumerate() {
        if cell == b'S' {
            start = Some(x);
        }
    }

    dbg!(start);

    let start_x = start.unwrap();
    let mut timelines = HashMap::new();
    timelines.insert(start_x, 1);

    for y in 1..grid.len() {
        let row = &grid[y];
        let mut new_timelines: HashMap<usize, usize> = HashMap::new();

        for (&pos, &count) in &timelines {
            if row[pos] == b'^' {
                *new_timelines.entry(pos - 1).or_default() += count;
                *new_timelines.entry(pos + 1).or_default() += count;
            } else {
                *new_timelines.entry(pos).or_default() += count;
            }
        }

        timelines = new_timelines;
    }

    dbg!(timelines.len());
    Ok(timelines.values().sum())
}

const SAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

pub fn main() -> Result<()> {
    let input = fetch_input(7)?;

    dbg!(part_1(&SAMPLE_INPUT)?);
    dbg!(part_1(&input)?);
    dbg!(part_2(&SAMPLE_INPUT)?);
    dbg!(part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let real_input = fetch_input(4).unwrap();

        assert_eq!(part_1(SAMPLE_INPUT).unwrap(), 13);
        assert_eq!(part_1(&real_input).unwrap(), 1533);
    }

    #[test]
    fn test_part_2() {
        let real_input = fetch_input(4).unwrap();

        assert_eq!(part_2(SAMPLE_INPUT).unwrap(), 43);
        assert_eq!(part_2(&real_input).unwrap(), 9206);
    }
}
