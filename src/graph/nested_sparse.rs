use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use rayon::{
    iter::{
        IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
    },
    slice::ParallelSliceMut,
};

use crate::graph::Graph;

use super::geometric_graph::GeometricGraph;

type Point = (usize, usize);
type Edge = (Point, Point);

fn build_graph_from_edges(edges: &[Edge]) -> GeometricGraph {
    let mut points = edges
        .par_iter()
        .flat_map(|&(p1, p2)| vec![p1, p2])
        .collect::<Vec<_>>();
    points.par_sort_unstable();
    points.dedup();
    let mut geo_points = points
        .par_iter()
        .map(|&(x, y)| geo::Point::new(x as f64, y as f64))
        .collect();
    let mut point_to_idx = points
        .par_iter()
        .enumerate()
        .map(|(i, &p)| (p, i))
        .collect::<HashMap<_, _>>();

    let mut edges_idx = edges
        .par_iter()
        .map(|(p1, p2)| {
            (
                *point_to_idx.get(p1).unwrap(),
                *point_to_idx.get(p2).unwrap(),
            )
        })
        .collect();

    let g = Graph::from_edge_list(edges_idx);
    GeometricGraph::new(g, geo_points)
}

fn get_super_grid_width(subgraph_size: usize, subgraph_count: usize) -> usize {
    // given a subgraph we want to create a grid where k cells of that grid are the subgraph
    // The width of this grid has to be such that cbrt(k*subgraph_size + width * width) = width
    // This function calculates the width of the grid given the subgraph node count and the number of times the subgraph appears in the supergraph
    // The solution is relatively complicated but can be found by wolfram alpha by solving the
    // cbrt(...) = width equation for width

    // -4 * subgraph_count to not count corners twice
    let x = (subgraph_size * subgraph_count - 4 * subgraph_count) as f64;
    // Compute the inner square root: sqrt(4x + 27x^2)
    let sqrt_inner = (4.0 * x + 27.0 * x * x).sqrt();
    // Compute the term inside the cube root: 2 + 27x + 3√3 * sqrt(4x + 27x^2)
    let term = 2.0 + 27.0 * x + 3.0 * 3_f64.sqrt() * sqrt_inner;
    // Compute the cube roots
    let term_cbrt = term.powf(1.0 / 3.0);
    let two_cbrt = 2.0_f64.powf(1.0 / 3.0);
    // Return the overall expression: 1/3 * ( 1 + (2^(1/3)/term_cbrt) + (term_cbrt/2^(1/3)) )
    ((1.0 / 3.0) * (1.0 + two_cbrt / term_cbrt + term_cbrt / two_cbrt)).ceil() as usize
}

fn get_grid(bottom_left: Point, top_right: Point, stride: usize) -> Vec<Edge> {
    assert_eq!(0, (top_right.0 - bottom_left.0) % stride);
    assert_eq!(0, (top_right.1 - bottom_left.1) % stride);

    let mut edges = Vec::new();
    for i in (bottom_left.0..=top_right.0).step_by(stride) {
        for j in (bottom_left.1..=top_right.1).step_by(stride) {
            if i + stride <= top_right.0 {
                edges.push(((i, j), (i + stride, j)));
            }
            if j + stride <= top_right.1 {
                edges.push(((i, j), (i, j + stride)));
            }
        }
    }
    edges
}

fn get_random_cells(
    bottom_left: Point,
    top_right: Point,
    subgraph_count: usize,
    stride: usize,
) -> Vec<(Point, Point)> {
    let mut points = HashSet::new();
    while points.len() < subgraph_count {
        let counts = (top_right.0 - bottom_left.0) / stride;
        let bottom_left = (
            thread_rng().gen_range(0..counts) * stride + bottom_left.0,
            thread_rng().gen_range(0..counts) * stride + bottom_left.1,
        );
        let top_right = (bottom_left.0 + stride, bottom_left.1 + stride);
        points.insert((bottom_left, top_right));
    }
    points.into_iter().collect()
}

fn build_sparse_grid_rec(
    bottom_left: Point,
    top_right: Point,
    level: usize,
    grid_widths: &[usize],
    subgraph_counts: &[usize],
) -> Vec<Edge> {
    let subgraph_count = subgraph_counts[level];
    let grid_width = grid_widths[level];
    let stride = (top_right.0 - bottom_left.0) / (grid_width - 1);

    let mut edges = get_grid(bottom_left, top_right, stride);

    if level == 0 {
        assert_eq!(stride, 1);
        return edges;
    }

    let cells = get_random_cells(bottom_left, top_right, subgraph_count, stride);
    for (bottom_left, top_right) in cells {
        let new_edges = build_sparse_grid_rec(
            bottom_left,
            top_right,
            level - 1,
            grid_widths,
            subgraph_counts,
        );
        edges.extend(new_edges);
    }

    edges
}

pub fn build_sparse_grid(subgraph_counts: &[usize], bottom_grid_width: usize) -> Vec<Edge> {
    assert_eq!(subgraph_counts[0], 0, "Lowest level cannot have subgraphs");
    let mut grid_widths = vec![bottom_grid_width];
    let mut subgraph_size = bottom_grid_width * bottom_grid_width;

    for &subgraph_count in subgraph_counts.iter().skip(1) {
        let width = get_super_grid_width(subgraph_size, subgraph_count);
        grid_widths.push(width);
        subgraph_size = subgraph_count * subgraph_size + width * width - 4 * subgraph_count;
    }

    let bottom_left = (0, 0);
    let scale = grid_widths.iter().map(|&x| x - 1).product();
    let top_right = (scale, scale);

    let mut edges = build_sparse_grid_rec(
        bottom_left,
        top_right,
        subgraph_counts.len() - 1,
        &grid_widths,
        &subgraph_counts,
    );

    edges.par_sort_unstable();
    edges.dedup();
    edges
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn working_base_grid() {
        let grid = get_grid((2, 2), (6, 6), 2);
        assert_eq!(grid.len(), 12);
        grid.iter().for_each(|(a, b)| {
            assert_eq!(a.0 % 2, 0);
            assert_eq!(a.1 % 2, 0);
            assert_eq!(b.0 % 2, 0);
            assert_eq!(b.1 % 2, 0);
            assert!(a.0 >= 2);
            assert!(a.1 >= 2);
            assert!(a.0 <= 6);
            assert!(a.1 <= 6);
            assert!(b.0 >= 2);
            assert!(b.1 >= 2);
            assert!(b.0 <= 6);
            assert!(b.1 <= 6);
        });
    }

    #[test]
    fn top_down_build_test() {
        let grid_widths = vec![3, 3, 3];
        let subgraph_counts = vec![0, 2, 2];
        let edges = build_sparse_grid_rec((0, 0), (8, 8), 2, &grid_widths, &subgraph_counts);
        edges
            .iter()
            .for_each(|(a, b)| println!("{} {} {} {}", a.0, a.1, b.0, b.1));
    }

    #[test]
    fn build_sparse_grid_test() {
        let edges = build_sparse_grid(&[0, 5, 50, 1000], 4);
        let g = build_graph_from_edges(&edges);
        g.graph.info();
        g.save(Path::new("output/sparse_grid"));
    }
}
