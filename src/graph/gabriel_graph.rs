use geo::{Distance, Euclidean, Point};
use hashbrown::HashSet;
use rayon::prelude::*;
use rstar::PointDistance;

use crate::library;

use super::delaunay::delaunay;
use super::geometric_graph::GeometricGraph;
use super::Graph;

pub fn gabriel_graph(n: usize) -> GeometricGraph {
    let points = library::random_points_in_circle(Point::new(100., 100.), 1., n);
    gabriel_graph_points(&points)
}

pub fn gabriel_graph_points(points: &[Point]) -> GeometricGraph {
    let g = delaunay(points);

    let edges = g
        .graph
        .get_directed_edges()
        .into_par_iter()
        .filter(|&(u, v)| {
            let p1 = points[u];
            let p2 = points[v];
            let center = Point::new((p1.x() + p2.x()) / 2.0, (p1.y() + p2.y()) / 2.0);
            let dist = Euclidean::distance(p1, p2);
            let radius = dist / 2.0;

            g.graph.data[u]
                .intersection(&g.graph.data[v])
                .filter(|&&x| Euclidean::distance(center, points[x]) < radius)
                .count()
                == 0
        })
        .collect::<Vec<_>>();

    let g = Graph::from_edge_list(edges);

    GeometricGraph::new(g, points.to_vec())
}

#[cfg(test)]
mod test {
    #[test]
    fn gg_test() {
        let g = super::gabriel_graph(100_000);
        g.visualize("rng");
        g.graph.info();
    }
}
