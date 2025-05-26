use std::mem;

use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use rayon::prelude::*;

use super::geometric_graph::GeometricGraph;

use self::Direction::*;

type Point = (usize, usize);
type Edge = (Point, Point);

// Algorithm overview:
// 1. Create base grid, save bot top left right
// 2. Translate copy by length in lower direction
// 3. Add cbrt many new edges between base and copy, sort to make planar

#[derive(Debug, Clone, Copy)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Right => Direction::Left,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
        }
    }
}

struct Grid {
    perimeter: Vec<Vec<Point>>,
    edges: Vec<Edge>,
    node_count: usize,
}

fn translate_point(point: Point, direction: Direction, length: usize) -> Point {
    match direction {
        Direction::Top => (point.0, point.1 + length),
        Direction::Right => (point.0 + length, point.1),
        Direction::Bottom => (point.0, point.1 - length),
        Direction::Left => (point.0 - length, point.1),
    }
}

fn translate_edge(edge: Edge, direction: Direction, length: usize) -> Edge {
    (
        translate_point(edge.0, direction, length),
        translate_point(edge.1, direction, length),
    )
}

impl Grid {
    fn translate(&self, direction: Direction, length: usize) -> Grid {
        let perimeter = self
            .perimeter
            .iter()
            .map(|p| {
                p.par_iter()
                    .map(|&x| translate_point(x, direction, length))
                    .collect()
            })
            .collect();

        let edges = self
            .edges
            .par_iter()
            .map(|&e| translate_edge(e, direction, length))
            .collect();

        Grid {
            perimeter,
            edges,
            node_count: self.node_count,
        }
    }

    fn cbrt_join_v2(&mut self, other: &mut Grid, side: Direction) -> Grid {
        let mut join_side_1 = self.perimeter[side as usize].clone();
        let mut join_side_2 = other.perimeter[side.opposite() as usize].clone();

        let cbrt = ((self.node_count + other.node_count) as f64).cbrt().ceil() as usize;
        join_side_1.shuffle(&mut thread_rng());
        join_side_2.shuffle(&mut thread_rng());
        join_side_1.truncate(cbrt);
        join_side_2.truncate(cbrt);
        join_side_1.sort_unstable();
        join_side_2.sort_unstable();

        let mut bridges = join_side_1
            .iter()
            .map(|x| {
                // let other = join_side_2
                //     .iter()
                //     .min_by_key(|&y| {
                //         let dx = (x.0 as isize - y.0 as isize).abs();
                //         let dy = (x.1 as isize - y.1 as isize).abs();
                //         dx * dx + dy * dy
                //     })
                //     .unwrap();

                // just move by one step in the direction
                match side {
                    Direction::Top => (*x, (x.0, x.1 + 1)),
                    Direction::Right => (*x, (x.0 + 1, x.1)),
                    Direction::Bottom => (*x, (x.0, x.1 - 1)),
                    Direction::Left => (*x, (x.0 - 1, x.1)),
                }
                // (*x, *other)
            })
            .collect::<Vec<_>>();

        let mut new_edges = Vec::new();
        new_edges.append(self.edges.as_mut());
        new_edges.append(other.edges.as_mut());
        new_edges.append(&mut bridges);

        Grid {
            perimeter: join_perimeters(
                mem::take(&mut self.perimeter),
                mem::take(&mut other.perimeter),
                side,
            ),
            edges: new_edges,
            node_count: self.node_count + other.node_count,
        }
    }

    fn cbrt_join(&mut self, other: &mut Grid, side: Direction) -> Grid {
        let mut join_side_1 = self.perimeter[side as usize].clone();
        let mut join_side_2 = other.perimeter[side.opposite() as usize].clone();

        let cbrt = ((self.node_count + other.node_count) as f64).cbrt().ceil() as usize;
        join_side_1.shuffle(&mut thread_rng());
        join_side_2.shuffle(&mut thread_rng());
        join_side_1.truncate(cbrt);
        join_side_2.truncate(cbrt);
        join_side_1.sort_unstable();
        join_side_2.sort_unstable();

        let mut bridges = join_side_1
            .into_iter()
            .zip(join_side_2.into_iter())
            .collect::<Vec<_>>();

        let mut new_edges = Vec::new();
        new_edges.append(self.edges.as_mut());
        new_edges.append(other.edges.as_mut());
        new_edges.append(&mut bridges);

        Grid {
            perimeter: join_perimeters(
                mem::take(&mut self.perimeter),
                mem::take(&mut other.perimeter),
                side,
            ),
            edges: new_edges,
            node_count: self.node_count + other.node_count,
        }
    }
}

fn join_perimeters(
    p1: Vec<Vec<Point>>,
    p2: Vec<Vec<Point>>,
    direction: Direction,
) -> Vec<Vec<Point>> {
    match direction {
        Direction::Right => {
            let mut new_top = p1[Direction::Top as usize].clone();
            new_top.extend(p2[Direction::Top as usize].iter().cloned());
            let new_right = p2[Direction::Right as usize].clone();
            let mut new_bottom = p1[Direction::Bottom as usize].clone();
            new_bottom.extend(p2[Direction::Bottom as usize].iter().cloned());
            let new_left = p1[Direction::Left as usize].clone();
            vec![new_top, new_right, new_bottom, new_left]
        }
        Direction::Top => {
            let new_top = p2[Direction::Top as usize].clone();
            let mut new_right = p1[Direction::Right as usize].clone();
            new_right.extend(p2[Direction::Right as usize].iter().cloned());
            let new_bottom = p1[Direction::Bottom as usize].clone();
            let mut new_left = p1[Direction::Left as usize].clone();
            new_left.extend(p2[Direction::Left as usize].iter().cloned());
            vec![new_top, new_right, new_bottom, new_left]
        }
        _ => panic!("Not implemented"),
    }
}
pub fn build_cbrt_grid(num_doubles: usize) -> GeometricGraph {
    let p = (0, 0);
    let mut g = Grid {
        perimeter: vec![vec![p]; 4],
        edges: Vec::new(),
        node_count: 1,
    };

    for i in 0..num_doubles {
        let mut g_new = g.translate(Direction::Top, 1 << i);
        g = g.cbrt_join_v2(&mut g_new, Direction::Top);
        let mut g_new = g.translate(Direction::Right, 1 << i);
        g = g.cbrt_join_v2(&mut g_new, Direction::Right);
    }

    GeometricGraph::from_edges_usize(&g.edges)
}

// pub fn build_cbrt_grid_recursive(i: usize) -> Grid {
//     if i == 0 {
//         return Grid {
//             perimeter: vec![vec![(0, 0)]; 4],
//             edges: Vec::new(),
//             node_count: 1,
//         };
//     }
//
//     let g1 = build_cbrt_grid_recursive(i - 1);
//     let g2 = build_cbrt_grid_recursive(i - 1).translate(Top, 1 << i + 1);
//     let g3 = build_cbrt_grid_recursive(i - 1).translate(Right, 1 << i + 1);
//     let g4 = build_cbrt_grid_recursive(i - 1)
//         .translate(Top, 1 << i + 1)
//         .translate(Right, 1 << i + 1);
//
//     let g_left = g1.
//
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::separator::Mode::*;

    #[test]
    fn cbrt_grid() {
        let g = build_cbrt_grid(5);
        g.graph.info();
        // g.graph.recurse_separator(Fast, None);
        // g.save(Path::new("output/cbrt_grid"));
        g.visualize("cbrt_grid");
    }

    #[test]
    fn cbrt_grid_large() {
        let g = build_cbrt_grid(10);
        g.inertial_flowcutter("cbrt_grid_large");
    }
}
