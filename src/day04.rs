use color_eyre::eyre::Result;
use itertools::Itertools;

use crate::fetch_input;

fn part_1(input: &str) -> Result<u32> {
    let input = input.trim();
    dbg!(input);

    let grid = input.lines().collect_vec();
    let mut display = grid
        .clone()
        .into_iter()
        .map(|s| s.to_string())
        .collect_vec()
        .clone();
    let mut forklifts = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            // Count how many of the 8 neightbors are @
            let mut count = 0;
            if char != '@' {
                continue;
            };

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let ny = (y as i32 + dy) as usize;
                    let nx = (x as i32 + dx) as usize;
                    if ny < grid.len() && nx < grid[ny].len() {
                        if grid[ny].chars().nth(nx) == Some('@') {
                            count += 1;
                        }
                    }
                }
            }

            if count < 4 {
                forklifts += 1;
                display[y].replace_range(x..x + 1, "X");
            }
        }
    }

    dbg!(display);

    Ok(forklifts)
}

fn part_2(input: &str) -> Result<u64> {
    let input = input.trim();
    dbg!(input);

    let mut grid = input.lines().map(|l| l.to_string()).collect_vec();
    let mut forklifts = 0;

    loop {
        let mut next_grid = grid.clone();
        let mut forklifts_this_round = 0;

        for (y, line) in grid.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                // Count how many of the 8 neightbors are @
                let mut count = 0;
                if char != '@' {
                    continue;
                };

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let ny = (y as i32 + dy) as usize;
                        let nx = (x as i32 + dx) as usize;
                        if ny < grid.len() && nx < grid[ny].len() {
                            if grid[ny].chars().nth(nx) == Some('@') {
                                count += 1;
                            }
                        }
                    }
                }

                if count < 4 {
                    forklifts += 1;
                    forklifts_this_round += 1;

                    next_grid[y].replace_range(x..x + 1, ".");
                }
            }
        }

        grid = next_grid;

        if forklifts_this_round == 0 {
            break;
        }
    }

    Ok(forklifts)
}

pub fn main() -> Result<()> {
    let input = fetch_input(4)?;

    //         let input = r#"..@@.@@@@.
    // @@@.@.@.@@
    // @@@@@.@.@@
    // @.@@@@..@.
    // @@.@@@@.@@
    // .@@@@@@@.@
    // .@.@.@.@@@
    // @.@@@.@@@@
    // .@@@@@@@@.
    // @.@.@@@.@.
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
