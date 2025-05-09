use geo::{Distance, Euclidean, Point};
use petgraph::unionfind::UnionFind;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use rstar::{primitives::GeomWithData, PointDistance};

use crate::{
    graph::{geometric_graph::GeometricGraph, Graph},
    library,
};

type IndexedPoint3D = GeomWithData<[f64; 3], usize>;

pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    pub fn random() -> Self {
        let rng = &mut rand::thread_rng();
        Point3D::new(
            rng.gen_range(1.0..1000.0),
            rng.gen_range(1.0..1000.0),
            rng.gen_range(1.0..1000.0),
        )
    }

    pub fn distance_2(&self, other: &Point3D) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)
    }

    pub fn distance(&self, other: &Point3D) -> f64 {
        self.distance_2(other).sqrt()
    }

    pub fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn almost_equal(&self, other: &Point3D) -> bool {
        (self.x - other.x).abs() < 1e-7
            && (self.y - other.y).abs() < 1e-7
            && (self.z - other.z).abs() < 1e-7
    }
}

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
    g.get_mst()
}

pub fn get_mst(n: usize) -> GeometricGraph {
    let points = library::random_points_in_circle(Point::new(1000.0, 1000.), 100.0, n);
    get_mst_points(&points)
}

fn get_expected_max_mst_distance2(n: usize, side_length: f64) -> f64 {
    const GAMMA: f64 = 0.5772156649015329;
    let n_f64 = n as f64;
    let res =
        side_length * ((3.0 * (n_f64.ln() + GAMMA)) / (4.0 * std::f64::consts::PI * n_f64)).cbrt();
    (res * 2.0).powi(2)
}

pub fn kruskal3d_points(points: &[Point3D]) -> Graph {
    let mut rng = &mut rand::thread_rng();
    let n = points.len();

    let rtee_data = points
        .iter()
        .enumerate()
        .map(|(i, p)| IndexedPoint3D::new(p.to_array(), i))
        .collect::<Vec<_>>();
    let rtree = rstar::RTree::bulk_load(rtee_data);

    // this assumes uniform distribution of points
    let max_expected_distance2 = get_expected_max_mst_distance2(n, 999.0);
    let mut distances = (0..n)
        .into_par_iter()
        .flat_map(|v| {
            rtree
                .locate_within_distance(points[v].to_array(), max_expected_distance2)
                .map(|u| (v, u.data, points[v].distance_2(&points[u.data])))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    distances.par_sort_unstable_by(|a, b| a.2.total_cmp(&b.2));

    let mut uf: UnionFind<usize> = UnionFind::new(n);
    let mut g = Graph::with_node_count(n);
    let mut num_edges = 0;
    for (u, v, _) in distances {
        if uf.union(u, v) {
            g.add_edge(u, v);
            num_edges += 1;
            if num_edges == n - 1 {
                break;
            }
        }
    }

    g
}

pub fn kruskal3d(n: usize) -> (Graph, Vec<Point3D>) {
    let points = (0..n).map(|_| Point3D::random()).collect::<Vec<_>>();
    (kruskal3d_points(&points), points)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_get_mst() {
        let g = GeometricGraph::from_edges_usize(&[
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

    #[test]
    fn simple_kruskal_3d() {
        for i in [10_000, 20_000, 50_000, 70_000, 100_000, 500_000, 1_000_000] {
            let g = kruskal3d(i);
            println!("{} {}", i, g.0.get_diameter());
        }
    }

    #[test]
    fn recurse_kruskal_3d() {
        let g = kruskal3d(10_000_000).0;
        g.recurse_diameter(Some(Path::new("./output/diameter/kruskal_3d")));
    }
}
