pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use graph::Graph;
use separator::Mode;

fn main() {
    let g = Graph::from_file("../Graphs/karlsruhe/first_out", "../Graphs/karlsruhe/head").unwrap();
    //let g = Graph::from_file("../Graphs/germany/first_out", "../Graphs/germany/head").unwrap();

    g.recurse_separator(0, Mode::Fast);
}
