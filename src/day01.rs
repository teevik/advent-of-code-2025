use color_eyre::eyre::{Result, bail};

use crate::fetch_input;

/// Parse line into offset
fn parse_line(line: &str) -> Result<i32> {
    let (dir, offset) = line.split_at(1);
    let offset = offset.parse::<i32>()?;

    let offset = match dir {
        "L" => -offset,
        "R" => offset,
        _ => bail!("Invalid direction"),
    };

    Ok(offset)
}

fn part_1(input: &str) -> Result<i32> {
    let mut dial = 50;
    let mut count = 0;

    for line in input.lines() {
        let offset = parse_line(line)?;

        dial = (dial + offset).rem_euclid(100);

        if dial == 0 {
            count += 1;
        }
    }

    Ok(count)
}

fn part_2(input: &str) -> Result<u32> {
    let mut dial = 50;
    let mut count = 0;

    for line in input.lines() {
        let offset = parse_line(line)?;

        let new_dial_unwrapped = dial + offset;

        if offset > 0 {
            count += (dial + offset) as u32 / 100;
        } else if offset < 0 {
            count += i32::abs_diff((dial).div_ceil(100), (new_dial_unwrapped).div_ceil(100));
        }

        dial = new_dial_unwrapped.rem_euclid(100);
    }

    Ok(count)
}

pub fn main() -> Result<()> {
    let input = fetch_input(1)?;

    dbg!(part_1(&input)?);
    dbg!(part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_part_1() {
        let real_input = fetch_input(1).unwrap();

        assert_eq!(part_1(SAMPLE_INPUT).unwrap(), 3);
        assert_eq!(part_1(&real_input).unwrap(), 1147);
    }

    #[test]
    fn test_part_2() {
        let real_input = fetch_input(1).unwrap();

        assert_eq!(part_2(SAMPLE_INPUT).unwrap(), 6);
        assert_eq!(part_2(&real_input).unwrap(), 6789);
    }
}
