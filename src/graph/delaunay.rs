use spade::{DelaunayTriangulation, InsertionError, Point2, Triangulation};

use super::{
    geometric_graph::{GeometricGraph, Position, AABB},
    Graph,
};

pub fn triangulation(positions: &[Position]) -> DelaunayTriangulation<Point2<f32>> {
    DelaunayTriangulation::<Point2<f32>>::bulk_load_stable(
        positions
            .iter()
            .map(|p| Point2::new(p.latitude(), p.longitude()))
            .collect(),
    )
    .unwrap()
}

pub fn random_delaunay(n: usize, aabb: AABB) -> GeometricGraph {
    let positions = Position::random_positions(n, aabb);
    let triangulation = triangulation(&positions);

    let g = Graph::from_edge_list(
        triangulation
            .undirected_edges()
            .map(|edge| {
                let [a, b] = edge.vertices();
                (a.index(), b.index())
            })
            .collect(),
    );

    GeometricGraph::new(g, positions)
}

pub fn length_restricted_delaunay(n: usize, aabb: AABB, max_dist: f32) -> GeometricGraph {
    let positions = Position::random_positions(n, aabb);
    let triangulation = triangulation(&positions);

    let mut g = Graph::from_edge_list(
        triangulation
            .undirected_edges()
            .filter(|edge| edge.length_2() < max_dist * max_dist)
            .map(|edge| {
                let [a, b] = edge.vertices();
                (a.index(), b.index())
            })
            .collect(),
    );
    g.add_max_node(n);

    GeometricGraph::new(g, positions)
}
