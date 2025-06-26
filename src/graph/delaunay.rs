use geo::{ConvexHull, MultiPoint, Point, Rect};
use itertools::Position;
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use rstar::PointDistance;
use spade::{DelaunayTriangulation, InsertionError, Point2, Triangulation};

use crate::library;

use super::{
    geometric_graph::{self, approx_dedup_edges, GeometricGraph},
    Graph,
};

pub fn triangulation(positions: &[Point]) -> DelaunayTriangulation<Point2<f64>> {
    DelaunayTriangulation::bulk_load_stable(
        positions
            .iter()
            .map(|p| Point2::new(p.x(), p.y()))
            .collect(),
    )
    .unwrap()
}

pub fn delaunay(n: usize) -> GeometricGraph {
    let p = library::random_points_in_circle(Point::new(1.0, 1.0), 1.0, n);
    delaunay_points(&p)
}

pub fn delaunay_points(positions: &[Point]) -> GeometricGraph {
    let triangulation = triangulation(positions);

    let g = Graph::from_edge_list(
        triangulation
            .undirected_edges()
            .map(|edge| {
                let [a, b] = edge.vertices();
                (a.index(), b.index())
            })
            .collect(),
    );

    GeometricGraph::new(g, positions.to_vec())
}

pub fn delaunay_edges(positions: &[Point]) -> Vec<(Point, Point)> {
    let triangulation = triangulation(positions);

    triangulation
        .undirected_edges()
        .map(|edge| {
            let [a, b] = edge.vertices();
            (positions[a.index()], positions[b.index()])
        })
        .collect()
}

pub fn random_delaunay_aabb(n: usize, aabb: Rect) -> GeometricGraph {
    let positions = library::random_points_in_rect(aabb, n);
    delaunay_points(&positions)
}

pub fn random_delaunay(n: usize) -> GeometricGraph {
    let aabb = geometric_graph::karlsruhe_bounding_rect();
    random_delaunay_aabb(n, aabb)
}

pub fn dynamic_length_restriced_delaunay(points: Vec<Point>, keep_factor: f64) -> GeometricGraph {
    assert!((0.0..=1.0).contains(&keep_factor));
    let mut edges = triangulation(&points)
        .undirected_edges()
        .par_bridge()
        .map(|edge| {
            let [a, b] = edge.vertices();
            let (a, b) = (a.index(), b.index());
            (a, b, points[a].distance_2(&points[b]))
        })
        .collect::<Vec<_>>();
    edges.sort_by(|(_, _, l1), (_, _, l2)| l1.partial_cmp(l2).unwrap());
    edges.truncate((edges.len() as f64 * keep_factor) as usize);

    let g = Graph::from_edge_list(edges.into_par_iter().map(|(a, b, _)| (a, b)).collect());
    GeometricGraph::new(g, points.to_vec())
}

// karlsruhe: 0.01
pub fn length_restricted_delaunay(points: Vec<Point>, max_length: f64) -> GeometricGraph {
    let mut edges = triangulation(&points)
        .undirected_edges()
        .par_bridge()
        .filter_map(|edge| {
            let [a, b] = edge.vertices();
            let (a, b) = (a.index(), b.index());
            if points[a].distance_2(&points[b]) <= max_length * max_length {
                Some((a, b, points[a].distance_2(&points[b])))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let g = Graph::from_edge_list(edges.into_par_iter().map(|(a, b, _)| (a, b)).collect());
    GeometricGraph::new(g, points.to_vec())
}

// pub fn degree_restricted_delaunay(
//     n: usize,
//     aabb: Rect,
//     max_dist: f32,
//     avg_deg: f64,
// ) -> GeometricGraph {
//     let mut g = length_restricted_delaunay(n, aabb, max_dist);
//     let num_edges = g.graph.get_num_edges();
//     let wanted_edges = (g.graph.get_num_nodes() as f64 * avg_deg / 2.0) as usize;
//     let edges_to_delete = num_edges - wanted_edges;
//     g.graph.remove_random_edges(edges_to_delete);
//     g
// }

pub fn delauny_avg_degree(points: &[Point], avg_deg: f64) -> GeometricGraph {
    let mut g = dynamic_length_restriced_delaunay(points.to_vec(), 0.99);
    let m = g.graph.get_num_edges();
    let wanted_edges = (g.graph.get_num_nodes() as f64 * avg_deg / 2.0) as usize;
    let edges_to_delete = m - wanted_edges;
    g.graph.remove_random_edges(edges_to_delete);
    g.largest_connected_component()
}
