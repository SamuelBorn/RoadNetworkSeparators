use geo::{Coord, Point};
use rayon::prelude::*;
use hashbrown::HashSet;
use rstar::primitives::GeomWithData;
use rstar::{PointDistance, RTree};

use crate::library;

use super::delaunay::delaunay;
use super::geometric_graph::GeometricGraph;
use super::Graph;

type IndexedPoint = GeomWithData<Point, usize>;

pub fn relative_neighborhood(n: usize) -> GeometricGraph {
    let points = library::random_points_in_circle(Point::new(100., 100.), 1., n);
    relative_neighborhood_points(&points)
}

pub fn relative_neighborhood_points(points: &[Point]) -> GeometricGraph {
    let indexed_points: Vec<IndexedPoint> = points
        .iter()
        .enumerate()
        .map(|(i, p)| IndexedPoint::new(*p, i))
        .collect();
    let rtree = RTree::bulk_load(indexed_points);

    let g = delaunay(points);

    let edges = g
        .graph
        .get_directed_edges()
        .into_iter()
        .filter(|edge| {
            let p1 = points[edge.0];
            let p2 = points[edge.1];
            let dist2 = p1.distance_2(&p2) - 1e-7;

            let l1 = rtree
                .locate_within_distance(p1, dist2)
                .map(|e| e.data)
                .collect::<HashSet<usize>>();
            let l2 = rtree
                .locate_within_distance(p2, dist2)
                .map(|e| e.data)
                .collect::<HashSet<usize>>();

            let intersection = l1.intersection(&l2);
            intersection.count() == 0
        })
        .collect::<Vec<_>>();

    let g = Graph::from_edge_list(edges);

    GeometricGraph::new(g, points.to_vec())
}

#[cfg(test)]
mod test {
    #[test]
    fn rng_test() {
        let g = super::relative_neighborhood(100_000);
        g.visualize("rng");
        g.graph.info();
    }
}
