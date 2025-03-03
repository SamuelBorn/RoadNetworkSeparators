use geo::Rect;
use geo::{Distance, Euclidean, Point};
use hashbrown::{HashMap, HashSet};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rstar::PointDistance;

use std::borrow::Borrow;
use std::collections::BinaryHeap;
use std::fs;
use std::io;
use std::path::Path;

use crate::library;
use crate::Graph;
use ordered_float::OrderedFloat;
use rand::Rng;

#[derive(Debug)]
pub struct GeometricGraph {
    pub graph: Graph,
    pub positions: Vec<Point>,
}

pub fn karlsruhe_bounding_rect() -> Rect {
    let min_point = Point::new(48.3, 8.0);
    let max_point = Point::new(49.2, 9.0);
    Rect::new(min_point, max_point)
}

pub fn random_point(aabb: Rect) -> Point {
    let mut rng = rand::thread_rng();
    Point::new(
        rng.gen_range(aabb.min().x..aabb.max().x),
        rng.gen_range(aabb.min().y..aabb.max().y),
    )
}

pub fn random_points(n: usize, aabb: Rect) -> Vec<Point> {
    (0..n).map(|_| random_point(aabb)).collect()
}

impl GeometricGraph {
    pub fn new(graph: Graph, positions: Vec<Point>) -> GeometricGraph {
        assert_eq!(graph.get_num_nodes(), positions.len());
        GeometricGraph { graph, positions }
    }

    pub fn from_file(dir: &Path) -> io::Result<Self> {
        let g = Graph::from_file(dir)?;

        let latitudes = library::read_binary_vec::<f32>(&dir.join("latitude"))?
            .par_iter()
            .map(|&f| f as f64)
            .collect::<Vec<_>>();
        let longitudes = library::read_binary_vec::<f32>(&dir.join("longitude"))?
            .par_iter()
            .map(|&f| f as f64)
            .collect::<Vec<_>>();

        assert_eq!(g.get_num_nodes(), latitudes.len());
        assert_eq!(g.get_num_nodes(), longitudes.len());

        let positions = latitudes
            .par_iter()
            .zip(longitudes.par_iter())
            .map(|(&lat, &lon)| Point::new(lat, lon))
            .collect();
        Ok(GeometricGraph::new(g, positions))
    }

    pub fn save(&self, dir: &Path) -> io::Result<()> {
        self.graph.to_file(dir)?;
        library::write_binary_vec(
            &self
                .positions
                .iter()
                .map(|p| p.x() as f32)
                .collect::<Vec<f32>>(),
            &dir.join("latitude"),
        )?;
        library::write_binary_vec(
            &self
                .positions
                .iter()
                .map(|p| p.y() as f32)
                .collect::<Vec<f32>>(),
            &dir.join("longitude"),
        )
    }

    pub fn get_position(&self, node: usize) -> Point {
        self.positions[node]
    }

    pub fn add_position_with_new_node(&mut self, pos: Point) -> usize {
        self.positions.push(pos);
        self.graph.add_node()
    }

    pub fn distance(&self, u: usize, v: usize) -> f64 {
        Euclidean::distance(self.get_position(u), self.get_position(v))
    }

    pub fn save_distance_overview(&self, file: &Path) {
        fs::write(
            file,
            self.graph
                .get_edges()
                .par_iter()
                .map(|(u, v)| self.distance(*u, *v).to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }

    pub fn largest_connected_component(&self) -> GeometricGraph {
        let g_map = self.graph.get_subgraphs_map(&HashSet::new());
        let g_map = g_map.iter().max_by_key(|(g)| g.len()).unwrap();

        let mut mapping = HashMap::new();
        let mut data = vec![Vec::new(); g_map.len()];
        let mut positions = vec![Point::new(0.0, 0.0); g_map.len()];

        for (&from_idx, to_nodes) in g_map.iter() {
            let possbile_next_idx = mapping.len();
            let new_from_idx = *mapping.entry(from_idx).or_insert(possbile_next_idx);
            positions[new_from_idx] = self.get_position(from_idx);
            for &to in to_nodes {
                let next_id = mapping.len();
                let to_idx = *mapping.entry(to).or_insert(next_id);
                data[new_from_idx].push(to_idx);
            }
        }

        GeometricGraph::new(Graph::new(data), positions)
    }

    pub fn get_edge_lengths(&self) -> HashMap<(usize, usize), f64> {
        self.graph
            .get_edges()
            .into_par_iter()
            .map(|(u, v)| ((u, v), self.distance(u, v)))
            .collect()
    }

    pub fn connected_with_prune_distance(&self, u: usize, v: usize, prune_distance: f64, edge_lengths: &HashMap<(usize, usize), f64>) -> bool {
        let mut visited = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push((OrderedFloat(0.0), u));

        while let Some((OrderedFloat(distance), node)) = heap.pop() {
            if distance > prune_distance {
                return false;
            }

            if node == v {
                return true;
            }

            if visited.contains(&node) {
                continue;
            }

            visited.insert(node);

            for &neighbor in self.graph.get_neighbors(node) {
                if visited.contains(&neighbor) {
                    continue;
                }

                let new_distance = distance + edge_lengths[&(node, neighbor)];
                heap.push((OrderedFloat(new_distance), neighbor));
            }

        }

        return false;
    }
}

#[cfg(test)]
mod test {
    use crate::graph::example;

    use super::*;

    #[test]
    fn approx_connected() {
        let g = example::example_c4();
        let edge_lengths = g.get_edge_lengths();
        assert!(!g.connected_with_prune_distance(0, 2, 1.0, &edge_lengths));
        assert!(g.connected_with_prune_distance(0, 2, 2.0, &edge_lengths));
    }

}
