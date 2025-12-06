use core::str;
use std::ops::RangeInclusive;

use color_eyre::eyre::Result;
use itertools::Itertools;
use rangemap::RangeInclusiveSet;

use crate::fetch_input;

fn part_1(input: &str) -> Result<usize> {
    let input = input.trim();

    let mut lines = input.lines().map(|l| l.to_owned()).collect_vec();
    let operators = lines.pop().unwrap();
    let operators = operators.split_whitespace().collect_vec();

    let numbers = lines
        .into_iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut total_sum = 0;

    for (i, op) in operators.iter().enumerate() {
        let mut sum = match *op {
            "+" => 0,
            "*" => 1,
            _ => unreachable!(),
        };

        for line in &numbers {
            match *op {
                "+" => sum += line[i],
                "*" => sum *= line[i],
                _ => unreachable!(),
            }
        }

        total_sum += sum;
    }

    Ok(total_sum)
}

fn part_2(input: &str) -> Result<usize> {
    let input = input.trim();

    let mut lines = input.lines().map(|l| l.as_bytes()).collect_vec();

    let operators = lines.pop().unwrap();
    let len = operators.len();

    let mut operators = operators
        .into_iter()
        .copied()
        .enumerate()
        .filter(|(i, c)| *c != b' ')
        .collect_vec();

    operators.push((len + 2, b's'));

    dbg!(&operators);

    let mut total_sum = 0;

    for [op, next_op] in operators.array_windows::<2>() {
        let &(i, op) = op;
        if op == b's' {
            continue;
        }

        let start = i;
        let end = next_op.0 - 1;

        dbg!(str::from_utf8(&lines[0][start..end]));

        let lines = lines
            .iter()
            .map(|l| {
                l[start..end]
                    .iter()
                    .copied()
                    .map(|c| match c {
                        b' ' => None,
                        c => Some((c as char).to_digit(10).unwrap() as usize),
                    })
                    .collect_vec()
            })
            .collect_vec();
        dbg!(&lines);

        let mut sum = match op {
            b'+' => 0,
            b'*' => 1,
            _ => unreachable!(),
        };

        let numbers = lines[0].len();
        for x in 0..numbers {
            let mut number = 0;

            for line in &lines {
                let digit = line[x];
                if digit.is_none() {
                    continue;
                }

                number *= 10;
                number += digit.unwrap();
            }

            match op {
                b'+' => sum += number,
                b'*' => sum *= number,
                _ => unreachable!(),
            }
        }
        total_sum += sum;
    }

    Ok(total_sum)
}

const SAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

pub fn main() -> Result<()> {
    let input = fetch_input(6)?;

    // dbg!(part_1(&SAMPLE_INPUT)?);
    // dbg!(part_1(&input)?);
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
