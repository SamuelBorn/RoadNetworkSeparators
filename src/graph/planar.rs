use geo::algorithm::line_intersection::{line_intersection, LineIntersection};
use geo::{Coord, Intersects, Line, Point};
use hashbrown::HashMap;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use rstar::{PointDistance, RTree, RTreeObject, AABB};
use std::collections::HashSet;

use super::geometric_graph::GeometricGraph;

const EPS: f64 = 1e-8;

#[derive(Debug, Clone, Copy)]
struct Edge {
    nodes: (usize, usize),
    line: Line<f64>,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes
    }
}

impl Eq for Edge {}

impl std::hash::Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.nodes.hash(state);
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.nodes.partial_cmp(&other.nodes)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.nodes.cmp(&other.nodes)
    }
}

impl RTreeObject for Edge {
    type Envelope = <Line<f64> as RTreeObject>::Envelope;

    fn envelope(&self) -> Self::Envelope {
        self.line.envelope()
    }
}

pub fn find_intersections(g: &GeometricGraph) {
    let edges = g
        .graph
        .get_directed_edges()
        .par_iter()
        .enumerate()
        .map(|(idx, &(i, j))| Edge {
            nodes: (i, j),
            line: Line::new(g.positions[i], g.positions[j]),
        })
        .collect::<Vec<_>>();

    let tree = RTree::bulk_load(edges.clone());

    let mut edge_to_intersections: HashMap<Edge, Vec<Edge>> = HashMap::new();
    let mut intersection_to_coord: HashMap<(Edge, Edge), Coord> = HashMap::new();

    edges.iter().for_each(|&current| {
        tree.locate_in_envelope_intersecting(&current.line.envelope())
            .filter(|&&candidate| current < candidate)
            .for_each(|&candidate| {
                if let Some(LineIntersection::SinglePoint { intersection, .. }) =
                    line_intersection(current.line, candidate.line)
                {
                    if intersection.distance_2(&current.line.start) > 1e-8
                        && intersection.distance_2(&current.line.end) > 1e-8
                        && intersection.distance_2(&candidate.line.start) > 1e-8
                        && intersection.distance_2(&candidate.line.end) > 1e-8
                    {
                        let entry = edge_to_intersections
                            .entry(current)
                            .or_insert(Vec::new())
                            .push(candidate);
                        let entry = edge_to_intersections
                            .entry(candidate)
                            .or_insert(Vec::new())
                            .push(current);
                        intersection_to_coord.insert((current, candidate), intersection);
                    }
                }
            })
    });

    println!("Found {} intersections", intersection_to_coord.len());
}

#[cfg(test)]
mod test {
    use crate::graph::{example, Graph};

    use super::*;

    #[test]
    fn intersection_works() {
        let g = Graph::from_edge_list(vec![(0, 1), (2, 3), (4, 5), (0, 2), (0, 2)]);
        let pos = vec![
            Point::new(0.0, 0.0),
            Point::new(3.0, 1.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 0.0),
            Point::new(2.0, 1.0),
        ];
        let g = GeometricGraph::new(g, pos);
        g.graph.info();
        find_intersections(&g);
    }

    #[test]
    fn intersection_karlsruhe() {
        let g = example::geometric_karlsruhe();
        find_intersections(&g);
    }
}
