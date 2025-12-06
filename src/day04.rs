use color_eyre::eyre::Result;

use crate::fetch_input;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells = input.trim().lines().map(|l| l.chars().collect()).collect();
        Self { cells }
    }

    fn get(&self, y: usize, x: usize) -> Option<char> {
        self.cells.get(y).and_then(|row| row.get(x).copied())
    }

    fn count_neighbors(&self, y: usize, x: usize, target: char) -> usize {
        DIRECTIONS
            .iter()
            .filter(|&&(dy, dx)| {
                let ny = y.checked_add_signed(dy);
                let nx = x.checked_add_signed(dx);
                ny.zip(nx)
                    .and_then(|(ny, nx)| self.get(ny, nx))
                    .is_some_and(|c| c == target)
            })
            .count()
    }

    fn iter_positions(&self) -> impl Iterator<Item = (usize, usize, char)> + '_ {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| (y, x, c)))
    }

    fn set(&mut self, y: usize, x: usize, c: char) {
        self.cells[y][x] = c;
    }
}

fn part_1(input: &str) -> Result<usize> {
    let grid = Grid::new(input);

    let forklifts = grid
        .iter_positions()
        .filter(|&(y, x, c)| c == '@' && grid.count_neighbors(y, x, '@') < 4)
        .count();

    Ok(forklifts)
}

fn part_2(input: &str) -> Result<usize> {
    let mut grid = Grid::new(input);
    let mut forklifts = 0;

    loop {
        let to_remove: Vec<_> = grid
            .iter_positions()
            .filter(|&(y, x, c)| c == '@' && grid.count_neighbors(y, x, '@') < 4)
            .map(|(y, x, _)| (y, x))
            .collect();

        if to_remove.is_empty() {
            break;
        }

        forklifts += to_remove.len();
        for (y, x) in to_remove {
            grid.set(y, x, '.');
        }
    }

    Ok(forklifts)
}

pub fn main() -> Result<()> {
    let input = fetch_input(4)?;

    dbg!(part_1(&input)?);
    dbg!(part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

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
