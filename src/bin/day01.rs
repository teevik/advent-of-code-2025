use color_eyre::eyre::{Result, bail};

enum Direction {
    Left,
    Right,
}

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

fn part_2(input: &str) -> Result<i32> {
    let mut dial = 50;
    let mut count = 0;

    for line in input.lines() {
        let offset = parse_line(line)?;

        let new_dial_unwrapped = dial + offset;

        if offset > 0 {
            count += (dial + offset) / 100;
        } else if offset < 0 {
            count += (dial - 1).div_euclid(100) - (new_dial_unwrapped - 1).div_euclid(100);
        }

        dial = new_dial_unwrapped.rem_euclid(100);
    }

    Ok(count)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = include_str!("day01.txt");

    dbg!(part_1(input)?);
    dbg!(part_2(input)?);

    Ok(())
}
