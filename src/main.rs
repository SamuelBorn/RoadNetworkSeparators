pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use std::fs;
use std::path::Path;

use graph::geometric_graph::{GeometricGraph, Position};
use graph::planar::naive_find_intersections;
use graph::{grid, Graph};
use separator::Mode::*;

fn main() {
    //let g = Graph::from_file(Path::new("../Graphs/karlsruhe")).unwrap();
    let mut g = GeometricGraph::from_file(Path::new("../Graphs/karlsruhe")).unwrap();
    //let g = Graph::from_file(Path::new("../Graphs/germany).unwrap();
    g.graph.recurse_separator(Eco, Some(Path::new("output/separator_karlsruhe_non_planar.txt")));
    
    let intersections = library::read_position_list(Path::new("output/intersections_karlsruhe.txt")).unwrap();
    println!("{} {}", g.graph.get_num_nodes(), g.graph.get_num_edges());
    g.planarize(intersections);
    println!("{} {}", g.graph.get_num_nodes(), g.graph.get_num_edges());
    
    g.graph.recurse_separator(Strong, Some(Path::new("output/separator_karlsruhe_planar.txt")));
}
