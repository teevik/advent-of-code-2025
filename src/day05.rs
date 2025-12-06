use std::ops::RangeInclusive;

use color_eyre::eyre::Result;
use itertools::Itertools;
use rangemap::RangeInclusiveSet;

use crate::fetch_input;

fn part_1(input: &str) -> Result<usize> {
    let input = input.trim();

    let (fresh_ingredient_ranges, available_ingredients) = input.split_once("\n\n").unwrap();

    let fresh_ingredient_ranges = fresh_ingredient_ranges
        .lines()
        .map(|l| {
            let parts = l.split_once('-').unwrap();
            let (start, last) = (
                parts.0.parse::<usize>().unwrap(),
                parts.1.parse::<usize>().unwrap(),
            );
            std::range::RangeInclusive { start, last }
        })
        .collect_vec();

    let fresh_ids = available_ingredients
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .filter(|&id| {
            fresh_ingredient_ranges
                .iter()
                .any(|range| range.contains(&id))
        })
        .count();

    Ok(fresh_ids)
}

fn part_2(input: &str) -> Result<usize> {
    let input = input.trim();

    let (fresh_ingredient_ranges, available_ingredients) = input.split_once("\n\n").unwrap();

    let mut range_set = RangeInclusiveSet::new();

    fresh_ingredient_ranges.lines().for_each(|l| {
        let parts = l.split_once('-').unwrap();
        let (start, last) = (
            parts.0.parse::<usize>().unwrap(),
            parts.1.parse::<usize>().unwrap(),
        );

        range_set.insert(start..=last);
    });

    Ok(range_set.into_iter().flatten().count())
}

const SAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

pub fn main() -> Result<()> {
    let input = fetch_input(5)?;

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
