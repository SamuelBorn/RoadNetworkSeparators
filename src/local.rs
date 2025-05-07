use geo::Distance;
use geo::Euclidean;
use geo::Point;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::seq::SliceRandom;
use rand::Rng;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use rstar::primitives::GeomWithData;
use rstar::RTree;

use crate::graph::geometric_graph::GeometricGraph;
use crate::graph::tree;
use crate::graph::Graph;
use crate::kruskal::get_mst_points;
use crate::lca;
use crate::library;

type IndexedPoint = GeomWithData<Point, usize>;

pub fn generate_random_connected(n: usize, m: usize) -> Graph {
    let mut g = tree::generate_random_tree(n);
    let mut edge_count = n - 1;

    while edge_count < m {
        let u = rand::thread_rng().gen_range(0..n);
        let v = rand::thread_rng().gen_range(0..n);

        if !g.has_edge(u, v) && u != v {
            g.add_edge(u, v);
            edge_count += 1;
        }
    }

    g
}

pub fn generate_local_graph_all(n: usize, m: usize) -> Graph {
    let rng = &mut rand::thread_rng();
    let mut g = tree::generate_random_tree(n);
    let additional_edges = m - (n - 1);
    let vs = (0..additional_edges)
        .map(|_| rng.gen_range(0..n))
        .collect::<Vec<_>>();
    let edges = (0..additional_edges)
        .map(|i| {
            let v = vs[i];
            let mut distances = g.bfs(v);
            distances[v] = 1;
            let mut w = distances
                .into_iter()
                .map(|d| 1.0 / (d as f64).powf(3.0))
                .collect::<Vec<_>>();
            w[v] = 0.0;
            let u = WeightedIndex::new(w).unwrap().sample(rng);
            (u, v)
        })
        .collect::<Vec<_>>();

    g.add_edges(&edges);
    g
}

pub fn generate_local_graph_all_lca(n: usize, m: usize) -> Graph {
    let rng = &mut rand::thread_rng();
    let mut g = tree::generate_random_tree(n);
    let lca = lca::LcaUtil::new(&g);
    let additional_edges = m - (n - 1);
    let vs = (0..additional_edges)
        .map(|_| rng.gen_range(0..n))
        .collect::<Vec<_>>();
    let edges = (0..additional_edges)
        .map(|i| {
            let v = vs[i];
            let mut distances = (0..n)
                .into_par_iter()
                .map(|i| lca.distance(v, i))
                .collect::<Vec<_>>();
            distances[v] = 1;
            let mut w = distances
                .into_iter()
                .map(|d| 1.0 / (d as f64).powf(3.0))
                .collect::<Vec<_>>();
            w[v] = 0.0;
            let u = WeightedIndex::new(w).unwrap().sample(rng);
            (u, v)
        })
        .collect::<Vec<_>>();

    g.add_edges(&edges);
    g
}

pub fn generate_local_graph(n: usize, m: usize) -> Graph {
    let mut g = tree::generate_random_tree(n);
    let mut edge_count = n - 1;

    while edge_count < m {
        let u = rand::thread_rng().gen_range(0..n);

        let (nodes, distances): (Vec<_>, Vec<_>) =
            g.get_extended_neighborhood(u, 1000).into_iter().unzip();
        let weights = distances
            .into_iter()
            .map(|d| 1.0 / d as f32)
            .collect::<Vec<_>>();
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::thread_rng();
        let v = nodes[dist.sample(&mut rng)];

        if !g.has_edge(u, v) {
            g.add_edge(u, v);
            edge_count += 1;
        }
    }

    g
}

pub fn generate_local_points(n: usize, m: usize) -> GeometricGraph {
    let points = library::random_points_in_circle(Point::new(1000.0, 1000.), 100.0, n);
    let rtree_data = points
        .iter()
        .enumerate()
        .map(|(i, &p)| IndexedPoint::new(p, i))
        .collect();
    let rtree = RTree::bulk_load(rtree_data);
    let mut g = get_mst_points(&points);
    let max_edge_length = *g
        .get_edge_lengths()
        .iter()
        .map(|(a, b)| OrderedFloat(*b))
        .max()
        .unwrap();
    let mut edge_count = n - 1;
    let rng = &mut rand::thread_rng();

    while edge_count < m {
        let u = rng.gen_range(0..n);
        let v_options = rtree
            .locate_within_distance(points[u], max_edge_length * max_edge_length)
            .collect::<Vec<_>>();
        let v = v_options.choose(rng).unwrap().data;

        if u != v {
            g.graph.add_edge(u, v);
            edge_count += 1;
        }
    }

    g
}

#[cfg(test)]
mod tests {
    use std::{time::Instant, vec};

    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    use crate::graph::tree::generate_random_tree;

    use super::*;

    #[test]
    fn test_generate_local_graph() {
        return;
        let n = 12000;
        let m = 15000;
        let g = generate_local_graph(n, m);
        assert_eq!(g.get_num_nodes(), n);
        assert_eq!(g.get_num_edges(), m * 2);

        // expect 22 at highest level
        g.recurse_separator(crate::separator::Mode::Fast, None);
    }

    #[test]
    fn test_generate_random_connected() {
        let n = 12000;
        let m = 15000;
        // let g = generate_random_connected(n, m);
        let g = generate_random_tree(n);

        dbg!(g.get_diameter());
    }

    #[test]
    fn random_spanning_tree_overview() {
        for i in 2..21 {
            let n = 2_usize.pow(i);
            let g1 = generate_random_connected(n, (1.25 * n as f32) as usize);
            let g2 = generate_random_connected(n, (1.25 * n as f32) as usize);
            let g3 = generate_random_connected(n, (1.25 * n as f32) as usize);
            let d1 = g1.get_diameter();
            let d2 = g2.get_diameter();
            let d3 = g3.get_diameter();
            let avg = (d1 + d2 + d3) / 3;
            println!("{} {}", n, avg);
        }
    }

    #[test]
    fn random_local_embedding() {
        let n = 10000;
        let m = 12500;
        let g = generate_local_points(n, m);
        g.graph.info();
        g.visualize("local_embedding");
        // g.inertial_flowcutter("local_embedding");
    }

    #[test]
    fn bulk_random_local() {
        vec![10_000, 30_000, 60_000, 100_000, 200_000]
            .into_iter()
            .for_each(|n| {
                let m = (1.25 * n as f64) as usize;
                let g = generate_local_graph_all(n, m);
                g.recurse_separator(crate::separator::Mode::Fast, None);
            });
    }

    #[test]
    fn time_random_tree_local() {
        let n = 10_000;
        let m = 12_500;

        let now = Instant::now();
        let g = generate_local_graph_all(n, m);
        println!("Time taken: {:?}", now.elapsed());

        let now = Instant::now();
        let g = generate_local_graph_all_lca(n, m);
        println!("Time taken: {:?}", now.elapsed());
    }
}
