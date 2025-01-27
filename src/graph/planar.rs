use std::collections::HashMap;

use ordered_float::OrderedFloat;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rstar::{Point, RTree, AABB};
use serde_json::Map;

use super::geometric_graph::{GeometricGraph, Position};

const EPS: f32 = 0.0000001;

impl Point for Position {
    type Scalar = OrderedFloat<f32>;
    const DIMENSIONS: usize = 2;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        Position::new_ordered(generator(0), generator(1))
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => OrderedFloat(self.latitude()),
            1 => OrderedFloat(self.longitude()),
            _ => unreachable!(),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => self.latitude_mut(),
            1 => self.longitude_mut(),
            _ => unreachable!(),
        }
    }
}

pub fn intersection(a1: Position, a2: Position, b1: Position, b2: Position) -> Option<Position> {
    let denom = (b2.latitude() - b1.latitude()) * (a2.longitude() - a1.longitude())
        - (b2.longitude() - b1.longitude()) * (a2.latitude() - a1.latitude());

    if denom.abs() < EPS {
        return None;
    }

    let ua = ((b2.longitude() - b1.longitude()) * (a1.latitude() - b1.latitude())
        - (b2.latitude() - b1.latitude()) * (a1.longitude() - b1.longitude()))
        / denom;
    let ub = ((a2.longitude() - a1.longitude()) * (a1.latitude() - b1.latitude())
        - (a2.latitude() - a1.latitude()) * (a1.longitude() - b1.longitude()))
        / denom;

    if ua > EPS && ua < 1.0 - EPS && ub > EPS && ub < 1.0 - EPS {
        Some(Position::new(
            a1.longitude() + ua * (a2.longitude() - a1.longitude()),
            a1.latitude() + ua * (a2.latitude() - a1.latitude()),
        ))
    } else {
        None
    }
}

pub fn naive_find_intersections(g: &GeometricGraph) -> Vec<Position> {
    let edges = g.graph.get_edges();

    (0..edges.len())
        .into_par_iter()
        .flat_map(|i| {
            let mut local_intersections = Vec::new();
            for j in i + 1..edges.len() {
                let (a1, a2) = edges[i];
                let (b1, b2) = edges[j];

                if a1 == b1 || a1 == b2 || a2 == b1 || a2 == b2 {
                    continue;
                }

                if let Some(pos) = intersection(
                    g.get_position(a1),
                    g.get_position(a2),
                    g.get_position(b1),
                    g.get_position(b2),
                ) {
                    local_intersections.push(pos);
                }
            }
            local_intersections
        })
        .collect()
}

impl GeometricGraph {
    pub fn planarize(&mut self, intersections: Vec<Position>) {
        let mut rtree = RTree::bulk_load(intersections.to_vec());
        let mut map = HashMap::with_capacity(intersections.len());

        for intersection in intersections {
            let index = self.add_position(intersection);
            map.insert(intersection, index);
        }

        for (u, v) in self.graph.get_edges() {
            let bounding_box = AABB::from_corners(self.get_position(u), self.get_position(v));
            let possible_intersections = rtree.locate_in_envelope_intersecting(&bounding_box);

            for intersection in possible_intersections {
                if intersection.on_line(self.get_position(u), self.get_position(v)) {
                    let index = *map.get(&intersection).unwrap();
                    self.graph.remove_edge(u, v);
                    self.graph.add_edge(u, index);
                    self.graph.add_edge(v, index);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::graph::Graph;

    use super::*;

    #[test]
    fn test_intersection() {
        let pos1 = Position::new(0.0, 0.0);
        let pos2 = Position::new(1.0, 1.0);
        let pos3 = Position::new(0.0, 1.0);
        let pos4 = Position::new(1.0, 0.0);

        assert_eq!(
            intersection(pos1, pos2, pos3, pos4),
            Some(Position::new(0.5, 0.5))
        );

        assert_eq!(intersection(pos1, pos3, pos2, pos4), None);

        assert_eq!(intersection(pos1, pos2, pos1, pos3), None);
    }

    #[test]
    fn test_naive_find_intersections() {
        let g = GeometricGraph::new(
            Graph::from_edge_list(vec![(0, 5), (1, 2), (2, 4), (4, 3), (3, 1), (0, 5)]),
            vec![
                Position::new(0.0, 0.0),
                Position::new(1.0, 0.0),
                Position::new(2.0, 0.0),
                Position::new(1.0, 1.0),
                Position::new(2.0, 1.0),
                Position::new(3.0, 1.0),
            ],
        );

        let intersections = naive_find_intersections(&g);
        assert_eq!(intersections.len(), 2);
        assert!(intersections.contains(&Position::new(0.33333334, 1.0)));
        assert!(intersections.contains(&Position::new(0.6666667, 2.0)));
    }

    #[test]
    fn test_planarize() {
        let mut g = GeometricGraph::new(
            Graph::from_edge_list(vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 2), (1, 3)]),
            vec![
                Position::new(0.0, 1.0),
                Position::new(0.0, 0.0),
                Position::new(1.0, 0.0),
                Position::new(1.0, 1.0),
            ],
        );

        let intersections = naive_find_intersections(&g);
        assert_eq!(intersections.len(), 1);
        assert!(intersections.contains(&Position::new(0.5, 0.5)));

        g.planarize(intersections);
        assert_eq!(g.graph.get_num_nodes(), 5);
        assert_eq!(g.graph.get_num_edges(), 8);
    }
}
