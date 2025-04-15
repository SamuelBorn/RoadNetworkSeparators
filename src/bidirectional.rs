use dashmap::DashMap;
use hashbrown::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::f64;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::sync::Arc;
use std::thread;

use crate::graph::geometric_graph::GeometricGraph;

#[derive(Copy, Clone, PartialEq)]
struct FloatState {
    cost: f64,
    position: usize,
}

impl Eq for FloatState {}

impl PartialOrd for FloatState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for FloatState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| {
                panic!("Comparison resulted in NaN cost in Dijkstra state");
            })
            .then_with(|| self.position.cmp(&other.position))
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Forward,
    Backward,
}

fn dijkstra_worker(
    graph: Arc<GeometricGraph>,
    start_node: usize,
    direction: Direction,
    max_length: f64,
    path_found: Arc<AtomicBool>,
    distances: Arc<DashMap<usize, f64>>,
    other_distances: Arc<DashMap<usize, f64>>,
    weights: Arc<HashMap<(usize, usize), f64>>,
) {
    let mut local_distances: HashMap<usize, f64> = HashMap::new();
    let mut pq = BinaryHeap::<FloatState>::new();

    local_distances.insert(start_node, 0.0);
    distances.insert(start_node, 0.0);
    pq.push(FloatState {
        cost: 0.0,
        position: start_node,
    });

    while let Some(FloatState { cost, position: u }) = pq.pop() {
        if path_found.load(AtomicOrdering::Acquire) {
            return;
        }

        if cost >= max_length {
            continue;
        }

        if cost > *local_distances.get(&u).unwrap_or(&f64::INFINITY) {
            continue;
        }

        if let Some(dist_other_ref) = other_distances.get(&u) {
            let current_total_path = cost + *dist_other_ref;
            if current_total_path < max_length {
                path_found.store(true, AtomicOrdering::Release);
                return;
            }
        }

        for &v in graph.graph.get_neighbors(u) {
            let weight = *weights.get(&(u, v)).unwrap();
            if path_found.load(AtomicOrdering::Acquire) {
                return;
            }

            let new_cost = cost + weight;

            if new_cost >= max_length {
                continue;
            }

            if new_cost < *local_distances.get(&v).unwrap_or(&f64::INFINITY) {
                local_distances.insert(v, new_cost);
                pq.push(FloatState {
                    cost: new_cost,
                    position: v,
                });
                distances.insert(v, new_cost);
            }
        }
    }
}

pub fn check_path_exists_max_length(
    graph: Arc<GeometricGraph>,
    start: usize,
    end: usize,
    max_length: f64,
) -> bool {
    if max_length <= 0.0 {
        return false;
    }
    if start == end {
        return true;
    }

    let weights = Arc::new(graph.get_edge_lengths());
    let forward_distances: Arc<DashMap<usize, f64>> = Arc::new(DashMap::new());
    let backward_distances: Arc<DashMap<usize, f64>> = Arc::new(DashMap::new());
    let path_found: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    let graph_f = Arc::clone(&graph);
    let path_found_f = Arc::clone(&path_found);
    let dist_f = Arc::clone(&forward_distances);
    let dist_b_f = Arc::clone(&backward_distances);
    let weights_f = Arc::clone(&weights);

    let forward_handle = thread::spawn(move || {
        dijkstra_worker(
            graph_f,
            start,
            Direction::Forward,
            max_length,
            path_found_f,
            dist_f,
            dist_b_f,
            weights_f,
        )
    });

    let graph_b = Arc::clone(&graph);
    let path_found_b = Arc::clone(&path_found);
    let dist_b = Arc::clone(&backward_distances);
    let dist_f_b = Arc::clone(&forward_distances);
    let weights_b = Arc::clone(&weights);

    let backward_handle = thread::spawn(move || {
        dijkstra_worker(
            graph_b,
            end,
            Direction::Backward,
            max_length,
            path_found_b,
            dist_b,
            dist_f_b,
            weights_b,
        )
    });

    forward_handle.join().expect("Forward thread panicked");
    backward_handle.join().expect("Backward thread panicked");

    path_found.load(AtomicOrdering::Relaxed)
}
