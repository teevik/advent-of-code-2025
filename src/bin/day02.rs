use color_eyre::eyre::Result;
use itertools::Itertools;

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

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = include_str!("day02.txt");

    dbg!(part_1(input)?);
    dbg!(part_2(input)?);

    Ok(())
}
