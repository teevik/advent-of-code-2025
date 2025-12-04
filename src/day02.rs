use color_eyre::eyre::Result;
use itertools::Itertools;

use crate::fetch_input;

/// Check if a number is "invalid" - made of some sequence of digits repeated twice
/// e.g., 55 (5 twice), 6464 (64 twice), 123123 (123 twice)
fn is_invalid(num: u64) -> bool {
    let s = num.to_string();
    let len = s.len();

    // Must have even number of digits to be repeated twice
    if len % 2 != 0 {
        return false;
    }

    let (first, second) = s.split_at(len / 2);
    first == second
}

/// Check if a number is "invalid" - made of some sequence of digits repeated twice or more
fn is_invalid_2(num: u64) -> bool {
    let s = num.to_string();
    let len = s.len();

    for i in 1..len {
        let mut chunks = s.as_bytes().chunks_exact(i);
        if chunks.remainder().is_empty() {
            if chunks.all_equal() {
                return true;
            }
        }
    }

    false
}

fn part_1(input: &str) -> Result<u64> {
    let mut invalid = 0;

    // Parse ranges separated by commas
    for range_str in input.trim().split(',') {
        let parts: Vec<&str> = range_str.split('-').collect();
        let start: u64 = parts[0].parse()?;
        let end: u64 = parts[1].parse()?;

        // Check all IDs in this range (inclusive)
        for id in start..=end {
            if is_invalid(id) {
                invalid += id
            }
        }
    }

    Ok(invalid)
}

fn part_2(input: &str) -> Result<u64> {
    let mut invalid = 0;

    // Parse ranges separated by commas
    for range_str in input.trim().split(',') {
        let parts: Vec<&str> = range_str.split('-').collect();
        let start: u64 = parts[0].parse()?;
        let end: u64 = parts[1].parse()?;

        // Check all IDs in this range (inclusive)
        for id in start..=end {
            if is_invalid_2(id) {
                invalid += id
            }
        }
    }

    Ok(invalid)
}

pub fn main() -> Result<()> {
    let input = fetch_input(2)?;

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
