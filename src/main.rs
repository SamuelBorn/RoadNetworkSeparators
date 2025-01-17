pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use graph::{grid, Graph};
use separator::Mode::*;

fn main() {
    //let g = Graph::from_file("../Graphs/karlsruhe/first_out", "../Graphs/karlsruhe/head").unwrap();
    //let g = Graph::from_file("../Graphs/germany/first_out", "../Graphs/germany/head").unwrap();

    grid::save_separator_distribution(10000, 1000000, 2, "output/separator_distribution.txt");
}
