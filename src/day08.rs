use core::str;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    io::BufRead,
    ops::RangeInclusive,
};

use color_eyre::eyre::Result;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use vek::Vec3;

use crate::fetch_input;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // each element is its own parent
            size: vec![1; n],         // each set starts with size 1
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px != py {
            // union by size
            if self.size[px] < self.size[py] {
                self.parent[px] = py;
                self.size[py] += self.size[px];
            } else {
                self.parent[py] = px;
                self.size[px] += self.size[py];
            }
        }
    }
}

fn part_1<const N: usize>(input: &str) -> usize {
    let input = input.trim();

    let points = input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();

            Vec3::new(x, y, z)
        })
        .collect_vec();
    let n = points.len();

    let pairs = (0..n)
        .array_combinations::<2>()
        .sorted_unstable_by_key(|&[a, b]| (points[a].distance_squared(points[b])))
        .collect_vec();

    let mut union = UnionFind::new(n);
    for &[a, b] in &pairs[0..N] {
        union.union(a, b);
    }

    union.size.sort_unstable_by_key(|&n| Reverse(n));

    let largest_groups = union.size.into_iter().take(3).product();

    largest_groups
}

fn part_2(input: &str) -> usize {
    let input = input.trim();

    let points = input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();

            Vec3::new(x, y, z)
        })
        .collect_vec();
    let n = points.len();

    let pairs = (0..n)
        .array_combinations::<2>()
        .sorted_unstable_by_key(|&[a, b]| points[a].distance_squared(points[b]))
        .collect_vec();

    let mut union = UnionFind::new(n);
    let mut num_components = n;

    for &[a, b] in &pairs {
        let pa = union.find(a);
        let pb = union.find(b);

        if pa != pb {
            union.union(a, b);
            num_components -= 1;

            if num_components == 1 {
                // All nodes are now in one component
                // Return the product of X coordinates
                return (points[a].x as usize) * (points[b].x as usize);
            }
        }
    }

    0
}

const SAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

pub fn main() -> Result<()> {
    let input = fetch_input(8)?;

    dbg!(part_1::<10>(&SAMPLE_INPUT));
    dbg!(part_1::<1000>(&input));
    dbg!(part_2(&SAMPLE_INPUT));
    dbg!(part_2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let real_input = fetch_input(4).unwrap();

        assert_eq!(part_1::<10>(SAMPLE_INPUT), 13);
        assert_eq!(part_1::<1000>(&real_input), 1533);
    }

    #[test]
    fn test_part_2() {
        let real_input = fetch_input(4).unwrap();

        assert_eq!(part_2(SAMPLE_INPUT), 43);
        assert_eq!(part_2(&real_input), 9206);
    }
}
