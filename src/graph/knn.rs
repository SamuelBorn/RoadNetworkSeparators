use geo::Point;

use crate::graph::geometric_graph::GeometricGraph;
use crate::graph::Graph;
use crate::library;
use rayon::prelude::*;
use rstar::primitives::GeomWithData;
use rstar::RTree;

pub fn knn(n: usize, k: usize) -> GeometricGraph {
    let p = library::random_points_in_circle(Point::new(2.0, 2.0), 1.0, n);
    knn_points(&p, k)
}

pub fn knn_points(points: &[Point], k: usize) -> GeometricGraph {
    let indexed_points = points
        .iter()
        .enumerate()
        .map(|(i, &p)| GeomWithData::new(p, i))
        .collect::<Vec<_>>();
    let rtree = RTree::bulk_load(indexed_points);

    let edges = rtree
        .iter()
        .par_bridge()
        .flat_map(|p| {
            rtree
                .nearest_neighbor_iter(p.geom())
                .skip(1)
                .take(k)
                .map(|neighbor| (p.data, neighbor.data))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let g = Graph::from_edge_list(edges);
    GeometricGraph::new(g, points.to_vec()).largest_connected_component()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{geometric_graph::GeometricGraph, voronoi::prune_graph_parallel};

    #[test]
    fn test_knn() {
        let n = 2usize.pow(22);
        let k = 3;
        let mut g = knn(n, k);
        g.graph.info();
        // g.inertial_flowcutter("knn");

        prune_graph_parallel(&mut g, 2.5);
        let g = g.largest_connected_component();
        g.inertial_flowcutter("knn_pruned");
    }
}
