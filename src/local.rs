use geo::Distance;
use geo::Euclidean;
use geo::Point;
use ordered_float::OrderedFloat;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::seq::SliceRandom;
use rand::Rng;
use rstar::primitives::GeomWithData;
use rstar::RTree;

use crate::graph::geometric_graph::GeometricGraph;
use crate::graph::tree;
use crate::graph::Graph;
use crate::kruskal::get_mst_points;
use crate::library;

type IndexedPoint = GeomWithData<Point, usize>;

pub fn generate_random_connected(n: usize, m: usize) -> Graph {
    let mut g = tree::generate_random_tree(n);
    let mut edge_count = n - 1;

    while edge_count < m {
        let u = rand::thread_rng().gen_range(0..n);
        let v = rand::thread_rng().gen_range(0..n);

        if !g.has_edge(u, v) {
            g.add_edge(u, v);
            edge_count += 1;
        }
    }

    g
}

pub fn generate_local_graph(n: usize, m: usize) -> Graph {
    let mut g = tree::generate_random_tree(n);
    let mut edge_count = n - 1;

    while edge_count < m {
        let u = rand::thread_rng().gen_range(0..n);

        let (nodes, distances): (Vec<_>, Vec<_>) =
            g.get_extended_neighborhood(u, 200).into_iter().unzip();
        let weights = distances
            .into_iter()
            .map(|d| 1.0 / d.pow(7) as f32)
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
        println!("edge count {}", edge_count);
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
        let n = 1000000;
        let m = 1250000;
        let g = generate_local_points(n, m);
        g.graph.info();
        // g.visualize("local_embedding");
        g.inertial_flowcutter("local_embedding");
    }
}
