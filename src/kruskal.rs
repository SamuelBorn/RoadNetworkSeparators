use geo::Point;
use petgraph::unionfind::UnionFind;
use rayon::prelude::*;
use rstar::PointDistance;

use crate::graph::{geometric_graph::GeometricGraph, Graph};

impl GeometricGraph {
    pub fn get_mst(&self) -> GeometricGraph {
        let mut edges = self
            .graph
            .get_edges()
            .par_iter()
            .map(|&(u, v)| {
                let (p1, p2) = (self.positions[u], self.positions[v]);
                (u, v, p1.distance_2(&p2))
            })
            .collect::<Vec<_>>();
        edges.par_sort_unstable_by(|a, b| a.2.total_cmp(&b.2));

        let mut uf: UnionFind<usize> = UnionFind::new(self.graph.get_num_nodes());
        let mut mst = Graph::with_node_count(self.graph.get_num_nodes());

        for (u, v, _) in edges {
            if uf.union(u, v) {
                mst.add_edge(u, v);
            }
        }

        GeometricGraph::new(mst, self.positions.clone())
    }
}

pub fn get_mst_points(points: &[Point]) -> GeometricGraph {
    let g = crate::graph::delaunay::delaunay(points);
    dbg!("Delaunay built");
    g.get_mst()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_get_mst() {
        let g = GeometricGraph::from_edges_usize(&vec![
            ((0, 0), (0, 1)),
            ((0, 0), (2, 0)),
            ((0, 1), (2, 3)),
            ((2, 0), (2, 3)),
        ]);
        let mst = g.get_mst();
        assert_eq!(mst.graph.get_num_nodes(), 4);
        assert_eq!(mst.graph.get_num_edges(), 3);
    }

    #[test]
    fn test_get_mst_points() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(2.0, 0.0),
            Point::new(2.0, 3.0),
        ];
        let mst = get_mst_points(&points);
        mst.save(Path::new("./output/tmp/kruskal_test")).unwrap();
        assert_eq!(mst.graph.get_num_nodes(), 4);
        assert_eq!(mst.graph.get_num_edges(), 3);
    }

    #[test]
    fn random_kruskal() {
        let bounds = 1000.0;
        let points = (0..1000)
            .map(|_| {
                Point::new(
                    rand::random::<f64>() * bounds,
                    rand::random::<f64>() * bounds,
                )
            })
            .collect::<Vec<_>>();
        let mst = get_mst_points(&points);
        mst.save(Path::new("./output/tmp/kruskal_random")).unwrap();
    }

    #[test]
    fn mst_diameter() {
        let bounds = 1000.0;
        let tries = 2;

        let ns: Vec<_> = (100_000..=1_000_000).step_by(100_000).collect();
        ns.into_par_iter().for_each(|n| {
            let mean_diameter = (0..tries)
                .map(|_| {
                    let points = (0..n)
                        .map(|_| {
                            Point::new(
                                rand::random::<f64>() * bounds,
                                rand::random::<f64>() * bounds,
                            )
                        })
                        .collect::<Vec<_>>();
                    let mst = get_mst_points(&points);
                    mst.graph.get_diameter()
                })
                .sum::<usize>() as f64
                / tries as f64;
            println!("n = {}, diameter = {}", n, mean_diameter);
        })
    }

    #[test]
    fn mst_100k() {
        let bounds = 1000.0;
        let n = 100_000;
        let points = (0..n)
            .map(|_| {
                Point::new(
                    rand::random::<f64>() * bounds,
                    rand::random::<f64>() * bounds,
                )
            })
            .collect::<Vec<_>>();
        let mst = get_mst_points(&points);
        mst.save(Path::new("./output/tmp/kruskal_100k")).unwrap();
    }
}
