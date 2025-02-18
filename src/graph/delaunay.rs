use geo::{Point, Rect};
use spade::{DelaunayTriangulation, InsertionError, Point2, Triangulation};

use super::{
    geometric_graph::{self, GeometricGraph},
    Graph,
};

pub fn triangulation(positions: &[Point]) -> DelaunayTriangulation<Point2<f32>> {
    DelaunayTriangulation::<Point2<f32>>::bulk_load_stable(
        positions
            .iter()
            .map(|p| Point2::new(p.x() as f32, p.y() as f32))
            .collect(),
    )
    .unwrap()
}

pub fn random_delaunay(n: usize, aabb: Rect) -> GeometricGraph {
    let positions = geometric_graph::random_points(n, aabb);
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

// karlsruhe: 0.01
pub fn length_restricted_delaunay(n: usize, aabb: Rect, max_dist: f32) -> GeometricGraph {
    let positions = geometric_graph::random_points(n, aabb);
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
    g.increase_size_to(n);

    GeometricGraph::new(g, positions)
}

pub fn degree_restricted_delaunay(
    n: usize,
    aabb: Rect,
    max_dist: f32,
    avg_deg: f64,
) -> GeometricGraph {
    let mut g = length_restricted_delaunay(n, aabb, max_dist);
    let num_edges = g.graph.get_num_edges();
    let wanted_edges = (g.graph.get_num_nodes() as f64 * avg_deg / 2.0) as usize;

    for i in wanted_edges..num_edges {
        g.graph.remove_random_edge();
    }

    g
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::graph::example;

    use super::*;

    #[test]
    #[ignore]
    fn test_delaunay() {
        let g = degree_restricted_delaunay(
            120000,
            geometric_graph::karlsruhe_bounding_rect(),
            0.01,
            2.5,
        );
        g.graph.queue_separator(
            crate::separator::Mode::Fast,
            Some(Path::new("output/sep_delaunay_degree")),
        );
    }
}
