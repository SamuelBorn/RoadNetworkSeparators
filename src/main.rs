pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use graph::{grid, Graph};
use graph::positioned_graph::PositionedGraph;
use separator::Mode::*;

fn main() {
    //let g = Graph::from_file("../Graphs/karlsruhe/first_out", "../Graphs/karlsruhe/head").unwrap();
    //let g = Graph::from_file("../Graphs/germany/first_out", "../Graphs/germany/head").unwrap();
    let g = PositionedGraph::from_file(
        "../Graphs/germany/first_out",
        "../Graphs/germany/head",
        "../Graphs/germany/latitude",
        "../Graphs/germany/longitude",
    );

    //let g = Graph::from_edge_list_file("dependencies/BoltzmannPlanarGraphs/300k").unwrap();
    //g.recurse_separator(Fast, Some("output/planar.txt"));
}
