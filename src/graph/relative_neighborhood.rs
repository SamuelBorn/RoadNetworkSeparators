use geo::Point;
use hashbrown::HashSet;
use rayon::prelude::*;
use rstar::PointDistance;
use spade::{DelaunayTriangulation, Point2, Triangulation};

use crate::library;

use super::delaunay::delaunay_points;
use super::geometric_graph::GeometricGraph;
use super::Graph;

pub fn relative_neighborhood(n: usize) -> GeometricGraph {
    let points = library::random_points_in_circle(Point::new(100., 100.), 1., n);
    relative_neighborhood_points(points)
}

pub fn relative_neighborhood_points(points: Vec<Point>) -> GeometricGraph {
    let edges = {
        let triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::bulk_load_stable(
            points
                .par_iter()
                .map(|p| Point2::new(p.x(), p.y()))
                .collect(),
        )
        .unwrap();
        println!("{}\tDelaunay triangulation done", chrono::Local::now());

        let edges_vec = triangulation
            .undirected_edges()
            .par_bridge()
            .filter_map(|edge| {
                let length = edge.length_2();
                let [u, v] = edge.vertices();
                let p_u = points[u.index()];
                let p_v = points[v.index()];

                let witness_exists = u.out_edges().any(|u_edge| {
                    let n_u = u_edge.to();
                    v.out_edges().any(|v_edge| {
                        let n_v = v_edge.to();
                        if n_u == n_v {
                            let p_n = &points[n_u.index()];
                            p_u.distance_2(p_n) < length && p_v.distance_2(p_n) < length
                        } else {
                            false
                        }
                    })
                });

                if witness_exists {
                    None
                } else {
                    Some((u.index(), v.index()))
                }
            })
            .collect::<Vec<_>>();
        println!("{}\tEdges filtered", chrono::Local::now());

        edges_vec
    }; // <-- 'triangulation' is dropped here, freeing its memory.

    let g = Graph::from_edge_list(edges);
    println!("{}\tGraph created", chrono::Local::now());
    GeometricGraph::new(g, points)
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
