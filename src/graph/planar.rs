use geo::algorithm::line_intersection::{line_intersection, LineIntersection};
use crate::library;
use crate::separator::Mode::*;
use geo::{Coord, Intersects, Line, Point};
use hashbrown::HashMap;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use rstar::{PointDistance, RTree, RTreeObject, AABB};
use std::collections::HashSet;
use std::mem;
use std::path::Path;

use super::{example, Graph};
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

fn custom_cmp(a: &Point, b: &Point) -> std::cmp::Ordering {
    let x_diff = (a.x() - b.x()).abs();

    if x_diff < EPS {
        a.y().total_cmp(&b.y())
    } else {
        a.x().total_cmp(&b.x())
    }
}

fn replace_with_intersections(g: &mut GeometricGraph, edge: &Edge, intersections: &mut Vec<usize>) {
    let (u, v) = edge.nodes;
    intersections.push(u);
    intersections.push(v);
    intersections.sort_unstable_by(|&a, &b| custom_cmp(&g.positions[a], &g.positions[b]));

    g.graph.remove_edge(u, v);
    for window in intersections.windows(2) {
        let (i, j) = (window[0], window[1]);
        assert!(i != j);
        g.graph.add_edge(i, j);
    }
}

pub fn planarize(g: &mut GeometricGraph) {
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

    // maps interscetion to new node id
    let mut intersection_to_coord: HashMap<(Edge, Edge), usize> = HashMap::new();
    // maps edges to all new nodes that are created due to intersections on this edge
    let mut edge_to_intersections: HashMap<Edge, Vec<usize>> = HashMap::new();

    println!("edges: {}", edges.len());
    edges.iter().enumerate().for_each(|(status, &current)| {
        if status % 1000 == 0 {
            println!("status: {}", status);
        }

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
                        let id = *intersection_to_coord
                            .entry((current, candidate))
                            .or_insert_with(|| g.add_position_with_new_node(intersection.into()));
                        let entry = edge_to_intersections
                            .entry(current)
                            .or_insert(Vec::new())
                            .push(id);
                        let entry = edge_to_intersections
                            .entry(candidate)
                            .or_insert(Vec::new())
                            .push(id);
                    }
                }
            })
    });
    println!("intersections: {}", intersection_to_coord.len());

    for (edge, mut intersections) in edge_to_intersections {
        replace_with_intersections(g, &edge, &mut intersections);
    }
}

pub fn extend_karlsruhe_separator() {
    let g = example::karlsruhe();
    let sep = g.get_separator_wrapper(Eco);
    library::write_text_vec(
        &sep.iter().collect::<Vec<_>>(),
        Path::new("output/karlsruhe_top_level_sep.txt"),
    );
    let mut g_planar = Graph::from_file(Path::new("output/karlsruhe_planar")).unwrap();
    let subgraphs = g_planar.get_subgraphs(&sep);
    subgraphs
        .iter()
        .for_each(|s| println!("{}", s.get_num_nodes()));

    for n in sep {
        g_planar.clear_vertex_edges(n);
    }
    let sep = g_planar.get_separator_wrapper(Strong);
    library::write_text_vec(
        &sep.iter().collect::<Vec<_>>(),
        Path::new("output/karlsruhe_top_level_sep_planar_additional.txt"),
    );
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::graph::{example, Graph};

    use super::*;
    use crate::separator::Mode::*;

    #[test]
    #[ignore]
    fn intersection_simple() {
        let g = Graph::from_edge_list(vec![(0, 1), (2, 3), (4, 5), (0, 2), (0, 2)]);
        let pos = vec![
            Point::new(0.0, 0.0),
            Point::new(3.0, 1.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 0.0),
            Point::new(2.0, 1.0),
        ];
        let mut g = GeometricGraph::new(g, pos);
        planarize(&mut g);
        g.graph.print();
    }

    #[test]
    #[ignore]
    fn intersection_karlsruhe() {
        let mut g = example::geometric_germany();
        planarize(&mut g);
        g.save(Path::new("output/germany_planar"));
        // germany #intersections: 101062
    }
}
