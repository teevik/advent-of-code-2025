use std::cmp::minmax;

use color_eyre::eyre::Result;
use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::fetch_input;

fn part_1(input: &str) -> Result<u32> {
    let input = input.trim();

    let mut sum = 0;

    for line in input.lines() {
        let numbers = line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();

        let mut biggest = 0;

        for [a, b] in numbers.iter().array_combinations::<2>() {
            let n = a * 10 + b;
            if n > biggest {
                biggest = n;
            }
        }

        sum += biggest;
    }

    Ok(sum)
}

fn part_2(input: &str) -> Result<u64> {
    let input = input.trim();

    let mut sum = 0;

    for line in input.lines() {
        let digits = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect_vec();

        let n = digits.len();
        let to_pick = 12;

        // Greedy selection: for each of the 12 positions, pick the largest
        // digit we can while leaving enough digits for remaining positions
        let mut result = 0;
        let mut start = 0; // earliest index we can pick from

        for remaining in (1..=to_pick).rev() {
            let end = n - remaining;

            let (best_idx, best_val) = digits[start..=end]
                .iter()
                .enumerate()
                .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i2.cmp(i1)))
                .unwrap();

            result = result * 10 + best_val;
            start = start + best_idx + 1;
        }

        dbg!(result);

        sum += result;
    }

    Ok(sum)
}

pub fn main() -> Result<()> {
    let input = fetch_input(3)?;

    //     let input = r#"987654321111111
    // 811111111111119
    // 234234234234278
    // 818181911112111
    // "#;

    dbg!(part_1(&input)?);
    dbg!(part_2(&input)?);

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const SAMPLE_INPUT: &str = "L68
// L30
// R48
// L5
// R60
// L55
// L1
// L99
// R14
// L82
// ";

//     #[test]
//     fn test_part_1() {
//         let real_input = fetch_input(1).unwrap();

//         assert_eq!(part_1(SAMPLE_INPUT).unwrap(), 3);
//         assert_eq!(part_1(&real_input).unwrap(), 1147);
//     }

//     #[test]
//     fn test_part_2() {
//         let real_input = fetch_input(1).unwrap();

//         assert_eq!(part_2(SAMPLE_INPUT).unwrap(), 6);
//         assert_eq!(part_2(&real_input).unwrap(), 6789);
//     }
// }
