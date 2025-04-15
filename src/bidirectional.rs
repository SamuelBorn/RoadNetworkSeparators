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
    graph: &GeometricGraph,
    start_node: usize,
    _direction: Direction,
    max_length: f64,
    path_found: Arc<AtomicBool>,
    distances: Arc<DashMap<usize, f64>>,
    other_distances: Arc<DashMap<usize, f64>>,
    weights: &HashMap<(usize, usize), f64>,
) {
    let mut local_distances: HashMap<usize, f64> = HashMap::new();
    let mut pq = BinaryHeap::<FloatState>::new();

    local_distances.insert(start_node, 0.0);
    distances.insert(start_node, 0.0);
    pq.push(FloatState {
        cost: 0.0,
        position: start_node,
    });

    if let Some(dist_other_ref) = other_distances.get(&start_node) {
        if *dist_other_ref <= max_length {
            path_found.store(true, AtomicOrdering::Release);
            return;
        }
    }

    while let Some(FloatState { cost, position: u }) = pq.pop() {
        if path_found.load(AtomicOrdering::Acquire) {
            return;
        }
        if cost > max_length {
            continue;
        }
        if cost > *local_distances.get(&u).unwrap_or(&f64::INFINITY) {
            continue;
        }

        if let Some(dist_other_ref) = other_distances.get(&u) {
            let current_total_path = cost + *dist_other_ref;
            if current_total_path <= max_length {
                path_found.store(true, AtomicOrdering::Release);
                return;
            }
        }

        for &v in graph.graph.get_neighbors(u) {
            if path_found.load(AtomicOrdering::Acquire) {
                return;
            }
            let weight = match weights.get(&(u, v)) {
                Some(w) => *w,
                None => continue,
            };
            let new_cost = cost + weight;

            if new_cost >= max_length {
                continue;
            }

            if new_cost < *local_distances.get(&v).unwrap_or(&f64::INFINITY) {
                local_distances.insert(v, new_cost);
                distances.insert(v, new_cost);
                pq.push(FloatState {
                    cost: new_cost,
                    position: v,
                });

                if let Some(dist_other_ref) = other_distances.get(&v) {
                    let current_total_path = new_cost + *dist_other_ref;
                    if current_total_path <= max_length {
                        path_found.store(true, AtomicOrdering::Release);
                        return;
                    }
                }
            }
        }
    }
}

impl GeometricGraph {
    // Warning: This function performs worse than the single-threaded version for short queries.
    pub fn bidirectional_multithreaded_dijsktra(
        &self,
        start: usize,
        end: usize,
        max_length: f64,
        weights: &HashMap<(usize, usize), f64>,
    ) -> bool {
        if max_length < 0.0 {
            return false;
        }
        if start == end {
            return true;
        }

        let forward_distances = Arc::new(DashMap::new());
        let backward_distances = Arc::new(DashMap::new());
        let path_found = Arc::new(AtomicBool::new(false));

        thread::scope(|s| {
            let path_found_f = Arc::clone(&path_found);
            let dist_f = Arc::clone(&forward_distances);
            let dist_b_f = Arc::clone(&backward_distances);

            s.spawn(move || {
                dijkstra_worker(
                    self,
                    start,
                    Direction::Forward,
                    max_length,
                    path_found_f,
                    dist_f,
                    dist_b_f,
                    &weights,
                )
            });

            let path_found_b = Arc::clone(&path_found);
            let dist_b = Arc::clone(&backward_distances);
            let dist_f_b = Arc::clone(&forward_distances);

            s.spawn(move || {
                dijkstra_worker(
                    self,
                    end,
                    Direction::Backward,
                    max_length,
                    path_found_b,
                    dist_b,
                    dist_f_b,
                    &weights,
                )
            });
        });

        path_found.load(AtomicOrdering::Relaxed)
    }
}
