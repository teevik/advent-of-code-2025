use color_eyre::eyre::Result;
use geo::{Coord, Intersects, LineString, Polygon};
use itertools::Itertools;
use rstar::{RTree, RTreeObject, AABB};
use vek::{Aabr, Extent2, Vec2};

use crate::fetch_input;

fn part_1(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Vec2::new(x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .array_combinations::<2>()
        .map(|[a, b]| (Aabr { min: a, max: b }.made_valid().size() + Extent2::new(1, 1)).product())
        .max()
        .unwrap()
}

#[derive(Clone, Copy)]
struct Edge {
    p1: [i64; 2],
    p2: [i64; 2],
}

impl RTreeObject for Edge {
    type Envelope = AABB<[i64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(
            [self.p1[0].min(self.p2[0]), self.p1[1].min(self.p2[1])],
            [self.p1[0].max(self.p2[0]), self.p1[1].max(self.p2[1])],
        )
    }
}

impl Edge {
    fn is_horizontal(&self) -> bool {
        self.p1[1] == self.p2[1]
    }

    /// Check if this edge crosses a rectangle side strictly inside (not at endpoints)
    fn crosses_rect_side(&self, left: i64, right: i64, top: i64, bottom: i64) -> bool {
        if self.is_horizontal() {
            // Horizontal edge can only cross vertical sides (left/right) of the rect
            let h_y = self.p1[1];
            let (h_min_x, h_max_x) = (self.p1[0].min(self.p2[0]), self.p1[0].max(self.p2[0]));
            // Crosses if y is strictly inside rect and edge spans across the side
            h_y > top
                && h_y < bottom
                && ((h_min_x < left && h_max_x > left) || (h_min_x < right && h_max_x > right))
        } else {
            // Vertical edge can only cross horizontal sides (top/bottom) of the rect
            let v_x = self.p1[0];
            let (v_min_y, v_max_y) = (self.p1[1].min(self.p2[1]), self.p1[1].max(self.p2[1]));
            // Crosses if x is strictly inside rect and edge spans across the side
            v_x > left
                && v_x < right
                && ((v_min_y < top && v_max_y > top) || (v_min_y < bottom && v_max_y > bottom))
        }
    }
}

fn part_2(input: &str) -> i64 {
    let points: Vec<[i64; 2]> = input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            [x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect_vec();

    let point_tree: RTree<[i64; 2]> = RTree::bulk_load(points.clone());

    let polygon = Polygon::new(
        points
            .iter()
            .map(|&[x, y]| Coord { x: x as f64, y: y as f64 })
            .collect::<LineString<f64>>(),
        vec![],
    );

    let edge_tree: RTree<Edge> = RTree::bulk_load(
        points
            .iter()
            .circular_tuple_windows()
            .map(|(&p1, &p2)| Edge { p1, p2 })
            .collect(),
    );

    let n = points.len();
    let mut max_area: i64 = 0;

    for a in 0..n {
        for b in (a + 1)..n {
            let (left, right) = (points[a][0].min(points[b][0]), points[a][0].max(points[b][0]));
            let (top, bottom) = (points[a][1].min(points[b][1]), points[a][1].max(points[b][1]));

            // Check if any point is strictly inside the rectangle
            if left + 1 < right && top + 1 < bottom {
                let interior = AABB::from_corners([left + 1, top + 1], [right - 1, bottom - 1]);
                if point_tree
                    .locate_in_envelope_intersecting(&interior)
                    .next()
                    .is_some()
                {
                    continue;
                }
            }

            // Check all 4 corners are inside or on the polygon boundary
            let corners = [
                [left, top],
                [right, top],
                [left, bottom],
                [right, bottom],
            ];
            if !corners
                .iter()
                .map(|&[x, y]| geo::Point::new(x as f64, y as f64))
                .all(|p| polygon.intersects(&p))
            {
                continue;
            }

            // Check no polygon edge crosses the rectangle sides
            let rect_envelope = AABB::from_corners([left, top], [right, bottom]);
            if edge_tree
                .locate_in_envelope_intersecting(&rect_envelope)
                .any(|edge| edge.crosses_rect_side(left, right, top, bottom))
            {
                continue;
            }

            max_area = max_area.max((right - left + 1) * (bottom - top + 1));
        }
    }

    max_area
}

const SAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

pub fn main() -> Result<()> {
    let input = fetch_input(9)?;

    dbg!(part_1(&SAMPLE_INPUT));
    dbg!(part_1(&input));
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

        assert_eq!(part_1(SAMPLE_INPUT), 13);
        assert_eq!(part_1(&real_input), 1533);
    }

    #[test]
    fn test_part_2() {
        let real_input = fetch_input(4).unwrap();

        assert_eq!(part_2(SAMPLE_INPUT), 43);
        assert_eq!(part_2(&real_input), 9206);
    }
}
