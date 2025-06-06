use geo::Distance;
use geo::Euclidean;
use geo::Point;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use ordered_float::Pow;
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

pub fn no_locality(n: usize, m: usize) -> Graph {
    let mut g = tree::random_tree(n);
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

pub fn tree_locality<F>(n: usize, m: usize, f: F) -> Graph
where
    F: Fn(usize) -> f64 + Send + Sync + Copy,
{
    assert!(f(2) > f(3));
    assert!(m > n);
    let mut g = tree::random_tree(n);
    let edges_to_add = m - (n - 1);

    let mut edges = (0..edges_to_add)
        .into_par_iter()
        .map(|_| {
            let u = rand::thread_rng().gen_range(0..n);
            let distances = g
                .bfs(u)
                .into_iter()
                .map(|d| if d > 1 { d } else { usize::MAX });
            let weights = WeightedIndex::new(distances.into_iter().map(f)).unwrap();
            let v = weights.sample(&mut rand::thread_rng());
            (u, v)
        })
        .collect::<Vec<_>>();

    g.add_edges(&edges);
    g
}

pub fn tree_locality_lca(n: usize, m: usize) -> Graph {
    let rng = &mut rand::thread_rng();
    let mut g = tree::random_tree(n);
    let lca = lca::LcaUtil::new(&g);
    let additional_edges = m - (n - 1);
    let vs = (0..additional_edges)
        .map(|_| rng.gen_range(0..n))
        .collect::<Vec<_>>();
    let edges = (0..additional_edges)
        .map(|i| {
            let v = vs[i];
            let mut w = (0..n)
                .into_par_iter()
                .map(|i| 1.0 / (lca.distance(v, i) as f64).powf(3.0))
                .collect::<Vec<_>>();
            w[v] = 0.0;
            let u = WeightedIndex::new(w).unwrap().sample(rng);
            (u, v)
        })
        .collect::<Vec<_>>();

    g.add_edges(&edges);
    g
}

pub fn tree_locality_bounded<F>(n: usize, m: usize, f: F) -> Graph
where
    F: Fn(usize) -> f64 + Send + Sync + Copy,
{
    assert!(f(2) > f(3));
    assert!(m > n);
    let mut g = tree::random_tree(n);
    let edges_to_add = m - (n - 1);

    let mut edges = (0..edges_to_add)
        .into_par_iter()
        .map(|_| {
            let u = rand::thread_rng().gen_range(0..n);
            let (idx, distances): (Vec<_>, Vec<_>) = g
                .bfs_bounded(u, 50_000)
                .into_iter()
                .filter(|(_, d)| *d > 1)
                .unzip();
            let weights = WeightedIndex::new(distances.into_iter().map(f)).unwrap();
            let v = idx[weights.sample(&mut rand::thread_rng())];
            (u, v)
        })
        .collect::<Vec<_>>();

    g.add_edges(&edges);
    g
}

pub fn geometric_locality(n: usize, m: usize) -> GeometricGraph {
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

    use crate::graph::tree::random_tree;

    use super::*;

    #[test]
    fn test_generate_random_connected() {
        let n = 120000;
        let m = 150000;
        let g = no_locality(n, m);
        g.info();
        let sep = g.get_separator_wrapper(crate::separator::Mode::Fast);
        let subgraphs = g.get_subgraphs(&sep);
        subgraphs.iter().for_each(|g| g.info());
    }

    #[test]
    fn random_spanning_tree_overview() {
        for i in 2..21 {
            let n = 2_usize.pow(i);
            let g1 = no_locality(n, (1.25 * n as f32) as usize);
            let g2 = no_locality(n, (1.25 * n as f32) as usize);
            let g3 = no_locality(n, (1.25 * n as f32) as usize);
            let d1 = g1.get_hop_diameter_approx();
            let d2 = g2.get_hop_diameter_approx();
            let d3 = g3.get_hop_diameter_approx();
            let avg = (d1 + d2 + d3) / 3;
            println!("{} {}", n, avg);
        }
    }

    #[test]
    fn random_local_embedding() {
        let n = 10000;
        let m = 12500;
        let g = geometric_locality(n, m);
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
                let g = tree_locality_lca(n, m);
                g.recurse_separator(crate::separator::Mode::Fast, None);
            });
    }

    #[test]
    fn time_random_tree_local() {
        let n = 1_000_000;
        let m = n * 5 / 4;
        let f = |x: usize| (x as f64).powf(-3.3);

        let g = tree_locality(n, m, f);
        println!("tree built");
        g.flowcutter(&format!("tree_locality_33_{}", n));
    }

    #[test]
    fn check_bounded_percentage() {
        for i in [100_000, 500_000, 1_000_000] {
            let g = tree::random_tree(i);
            let mut dist = g.bfs(0);
            dist[0] = usize::MAX;
            let (_, dist_bounded): (Vec<usize>, Vec<usize>) =
                g.bfs_bounded(0, 50_000).iter().unzip();
            let f = |x: usize| (x as f64).pow(-3.0);
            let sum: f64 = dist.iter().map(|x| f(*x)).sum();
            let sum_bounded: f64 = dist_bounded.iter().map(|x| f(*x)).sum();
            let percentage = sum_bounded / sum;
            println!("n: {}, percentage: {}", i, percentage);
        }
    }

    #[test]
    fn find_best_tree_locality_pow() {
        let n = 2usize.pow(18);
        let m = n * 5 / 4;
        let pows = [2.9, 3.0, 3.1, 3.2, 3.3, 3.4, 3.5, 3.6];

        let graphs = pows
            .iter()
            .map(|pow| {
                let start = Instant::now();
                let g = tree_locality(n, m, |x: usize| (x as f64).powf(-pow));
                println!("Built {} in {} s", pow, start.elapsed().as_secs());
                (g, *pow)
            })
            .collect::<Vec<_>>();

        graphs.into_par_iter().for_each(|(g, pow)| {
            g.flowcutter(&format!("tree_locality_pow_{}", pow));
        });
    }
}
