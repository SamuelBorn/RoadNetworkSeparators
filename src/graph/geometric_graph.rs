use geo::Rect;
use geo::{Distance, Euclidean, Point};
use hashbrown::{HashMap, HashSet};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;
use rstar::PointDistance;

use std::borrow::Borrow;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

use crate::library;
use crate::Graph;
use ordered_float::OrderedFloat;
use rand::Rng;

#[derive(Debug)]
pub struct GeometricGraph {
    pub graph: Graph,
    pub positions: Vec<Point>,
}

const QUANTIZE_SCALE: f64 = 1e12;

pub fn quantize(p: &geo::Point) -> (i64, i64) {
    (
        (p.x() * QUANTIZE_SCALE).round() as i64,
        (p.y() * QUANTIZE_SCALE).round() as i64,
    )
}

pub fn inv_quantize((x, y): (i64, i64)) -> Point {
    Point::new(x as f64 / QUANTIZE_SCALE, y as f64 / QUANTIZE_SCALE)
}

pub fn approx_dedup_points(points: &mut Vec<Point>) {
    points.sort_unstable_by(|a, b| quantize(a).cmp(&quantize(b)));
    points.dedup_by(|a, b| quantize(a) == quantize(b));
}

pub fn approx_dedup_edges(edges: &mut Vec<(Point, Point)>) {
    edges.sort_unstable_by(|(a1, a2), (b1, b2)| {
        let a = quantize(a1).cmp(&quantize(a2));
        if a == std::cmp::Ordering::Equal {
            quantize(b1).cmp(&quantize(b2))
        } else {
            a
        }
    });
    edges.dedup_by(|(a1, a2), (b1, b2)| {
        quantize(a1) == quantize(b1) && quantize(a2) == quantize(b2)
    });
}

pub fn karlsruhe_bounding_rect() -> Rect {
    let min_point = Point::new(48.3, 8.0);
    let max_point = Point::new(49.2, 9.0);
    Rect::new(min_point, max_point)
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

    pub fn get_edges_points(&self) -> Vec<(Point, Point)> {
        self.graph
            .get_edges()
            .par_iter()
            .map(|(u, v)| (self.get_position(*u), self.get_position(*v)))
            .collect()
    }

    pub fn from_edges_usize(edges: &[((usize, usize), (usize, usize))]) -> GeometricGraph {
        let mut points = edges
            .par_iter()
            .flat_map(|&(p1, p2)| vec![p1, p2])
            .collect::<Vec<_>>();
        points.par_sort_unstable();
        points.dedup();
        let mut geo_points = points
            .par_iter()
            .map(|&(x, y)| geo::Point::new(x as f64, y as f64))
            .collect();
        let mut point_to_idx = points
            .par_iter()
            .enumerate()
            .map(|(i, &p)| (p, i))
            .collect::<HashMap<_, _>>();

        let mut edges_idx = edges
            .par_iter()
            .map(|(p1, p2)| {
                (
                    *point_to_idx.get(p1).unwrap(),
                    *point_to_idx.get(p2).unwrap(),
                )
            })
            .collect();

        let g = Graph::from_edge_list(edges_idx);
        GeometricGraph::new(g, geo_points)
    }

    pub fn from_edges_point(edges: &[(Point, Point)]) -> GeometricGraph {
        let edges = edges
            .par_iter()
            .map(|(p1, p2)| {
                let (q1, q2) = (quantize(p1), quantize(p2));
                (
                    (q1.0 as usize, q1.1 as usize),
                    (q2.0 as usize, q2.1 as usize),
                )
            })
            .collect::<Vec<_>>();

        let mut g = GeometricGraph::from_edges_usize(&edges);
        g.positions = g
            .positions
            .par_iter()
            .map(|p| Point::new(p.x() / QUANTIZE_SCALE, p.y() / QUANTIZE_SCALE))
            .collect();
        g
    }

    pub fn save(&self, dir: &Path) -> io::Result<()> {
        self.graph.save(dir)?;
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

    pub fn euclidean_distance(&self, u: usize, v: usize) -> f64 {
        Euclidean::distance(self.get_position(u), self.get_position(v))
    }

    pub fn save_edge_length_overview(&self, file: &Path) {
        fs::write(
            file,
            self.graph
                .get_edges()
                .par_iter()
                .map(|(u, v)| self.euclidean_distance(*u, *v).to_string())
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
            .map(|(u, v)| ((u, v), self.euclidean_distance(u, v)))
            .collect()
    }

    pub fn get_edge_lengths_unidirectional(&self) -> HashMap<(usize, usize), f64> {
        self.graph
            .get_edges()
            .into_par_iter()
            .filter(|(u, v)| u < v)
            .map(|(u, v)| ((u, v), self.euclidean_distance(u, v)))
            .collect()
    }

    pub fn connected_with_prune_distance(
        &self,
        u: usize,
        v: usize,
        prune_distance: f64,
        edge_lengths: &HashMap<(usize, usize), f64>,
    ) -> bool {
        let mut distances = HashMap::new();
        let mut heap = BinaryHeap::new();
        distances.insert(u, 0.0);
        heap.push(Reverse((OrderedFloat(0.0), u))); // (distance, vertex)

        while let Some(Reverse((OrderedFloat(dist), current))) = heap.pop() {
            if dist >= prune_distance {
                return false;
            }

            if current == v {
                return dist <= prune_distance;
            }

            let current_best = *distances.get(&current).unwrap();
            if dist > current_best || dist >= prune_distance {
                continue;
            }

            for &neighbor in self.graph.get_neighbors(current) {
                let weight = *edge_lengths.get(&(current, neighbor)).unwrap();
                let new_dist = dist + weight;

                // Only process if the new distance is better and less than prune_distance
                if new_dist < *distances.get(&neighbor).unwrap_or(&f64::INFINITY)
                    && new_dist <= prune_distance
                {
                    distances.insert(neighbor, new_dist);
                    heap.push(Reverse((OrderedFloat(new_dist), neighbor)));
                }
            }
        }

        false
    }

    pub fn dijsktra(&self, start: usize, end: usize) -> f64 {
        let n = self.graph.data.len();
        let mut distances = vec![f64::INFINITY; n];
        distances[start] = 0.0;
        let mut pq = BinaryHeap::new();
        pq.push((OrderedFloat(0.0), start));

        while let Some((OrderedFloat(dist), u)) = pq.pop() {
            if u == end {
                return dist;
            }
            if dist > distances[u] {
                continue;
            }
            for &v in &self.graph.data[u] {
                let weight = self.euclidean_distance(u, v);
                let new_dist = dist + weight;

                if new_dist < distances[v] {
                    distances[v] = new_dist;
                    pq.push((OrderedFloat(new_dist), v));
                }
            }
        }

        f64::INFINITY
    }

    pub fn distance_overview(&self, n: usize) -> Vec<f64> {
        let mut res = (0..n)
            .into_par_iter()
            .map(|_| {
                let i = rand::thread_rng().gen_range(0..self.graph.get_num_nodes());
                let j = rand::thread_rng().gen_range(0..self.graph.get_num_nodes());
                self.dijsktra(i, j)
            })
            .collect::<Vec<_>>();
        let max = *res
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&1.0);
        res.iter_mut().for_each(|x| *x /= max);
        res
    }

    pub fn visualize(&self, name: &str) {
        let g_path = format!("./output/graphs/{}", name);
        self.save(Path::new(&g_path));

        Command::new("python3")
            .arg("scripts/visualize_graph.py")
            .arg(g_path)
            .spawn();
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
        assert!(!g.connected_with_prune_distance(0, 2, 1.4, &edge_lengths));
        assert!(g.connected_with_prune_distance(0, 2, 2.00001, &edge_lengths));
    }
}
