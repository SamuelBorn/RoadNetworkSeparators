use geo::Point;
use hashbrown::HashSet;
use rayon::prelude::*;
use rstar::PointDistance;

use crate::library;

use super::delaunay::delaunay;
use super::geometric_graph::GeometricGraph;
use super::Graph;

pub fn relative_neighborhood(n: usize) -> GeometricGraph {
    let points = library::random_points_in_circle(Point::new(100., 100.), 1., n);
    relative_neighborhood_points(&points)
}

pub fn relative_neighborhood_points(points: &[Point]) -> GeometricGraph {
    let g = delaunay(points);

    let edges = g
        .graph
        .get_directed_edges()
        .into_par_iter()
        .filter(|&(u, v)| {
            let p1 = points[u];
            let p2 = points[v];
            let dist2 = p1.distance_2(&p2);

            g.graph.data[u]
                .intersection(&g.graph.data[v])
                .filter(|&&x| {
                    p1.distance_2(&points[x]) < dist2 && p2.distance_2(&points[x]) < dist2
                })
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
    fn rng_test() {
        let g = super::relative_neighborhood(100_000);
        g.visualize("rng");
        g.graph.info();
    }
}
