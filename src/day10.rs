use color_eyre::eyre::Result;
use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variable};
use itertools::Itertools;
use pathfinding::prelude::bfs;
use rayon::prelude::*;

use crate::fetch_input;

fn part_1(input: &str) -> usize {
    let input = input.trim();

    let mut sum = 0;

    for line in input.lines() {
        let (target_lights, rest) = line.split_once(' ').unwrap();
        let (buttons, _joltage_requirements) = rest.rsplit_once(' ').unwrap();

        let target_lights = &target_lights[1..target_lights.len() - 1];
        let length = target_lights.len();
        // Map target_lights . to 0 and # to 1 bitmap
        let mut target_lights_map = 0u16;
        for (i, target_light) in target_lights.chars().rev().enumerate() {
            target_lights_map |= ((target_light == '#') as u16) << (i as u16);
        }

        // println!("{target_lights} {target_lights_map:b}");

        let buttons = buttons
            .split(' ')
            .map(|buttons| {
                let buttons = &buttons[1..buttons.len() - 1];
                let mut button_map = 0u16;

                for n in buttons.split(',').map(|n| n.parse::<u16>().unwrap()) {
                    let pos = length as u16 - 1 - n;
                    button_map |= 1 << pos;
                }

                // println!("{buttons} {button_map:b}");

                button_map
            })
            .collect_vec();

        let result = bfs(
            &0u16,
            |current| buttons.iter().map(|button| current ^ button).collect_vec(),
            // |current| 1,
            |&current| current == target_lights_map,
        );

        let length = result.unwrap().len() - 1;
        sum += length;
    }

    sum
}

/// Solve using Integer Linear Programming with CBC solver
fn solve_ilp(buttons: &[Vec<usize>], targets: &[u32]) -> usize {
    let num_counters = targets.len();
    let num_buttons = buttons.len();

    // Create integer variables for each button press count
    let mut problem = variable::ProblemVariables::new();
    let button_vars: Vec<_> = (0..num_buttons)
        .map(|_| problem.add(variable::variable().integer().min(0)))
        .collect();

    // Objective: minimize total button presses
    let objective: Expression = button_vars.iter().map(|&v| v).sum();

    // Create the optimization problem
    let mut model = problem.minimise(objective).using(default_solver);

    // Add constraints: for each counter, sum of button presses affecting it = target
    for counter_idx in 0..num_counters {
        let mut counter_sum: Expression = 0.into();
        for (button_idx, button) in buttons.iter().enumerate() {
            if button.contains(&counter_idx) {
                counter_sum += button_vars[button_idx];
            }
        }
        model = model.with(constraint!(counter_sum == targets[counter_idx] as f64));
    }

    // Solve ILP
    let solution = model.solve().unwrap();

    // Sum up the integer button presses
    button_vars
        .iter()
        .map(|&v| solution.value(v).round() as usize)
        .sum()
}

fn part_2(input: &str) -> usize {
    let input = input.trim();

    input
        .lines()
        .map(|line| {
            let (_target_lights, rest) = line.split_once(' ').unwrap();
            let (buttons, joltage_requirements) = rest.rsplit_once(' ').unwrap();

            let joltage_requirements = &joltage_requirements[1..joltage_requirements.len() - 1];
            let targets: Vec<u32> = joltage_requirements
                .split(',')
                .map(|joltage| joltage.parse().unwrap())
                .collect_vec();

            let buttons: Vec<Vec<usize>> = buttons
                .split(' ')
                .map(|buttons| {
                    let buttons = &buttons[1..buttons.len() - 1];
                    buttons
                        .split(",")
                        .map(|button| button.parse().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            solve_ilp(&buttons, &targets)
        })
        .sum()
}

const SAMPLE_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

pub fn main() -> Result<()> {
    let input = fetch_input(10)?;

    // dbg!(part_1(&SAMPLE_INPUT));
    // dbg!(part_1(&input));
    dbg!(part_2(&SAMPLE_INPUT));
    dbg!(part_2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let real_input = fetch_input(10).unwrap();

        assert_eq!(part_1(SAMPLE_INPUT), 13);
        assert_eq!(part_1(&real_input), 1533);
    }

    #[test]
    fn test_part_2() {
        let real_input = fetch_input(10).unwrap();

        assert_eq!(part_2(SAMPLE_INPUT), 33);
    }
}
