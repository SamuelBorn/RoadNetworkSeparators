pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use graph::{grid::{generate_grid, generate_grid_with_avg_degree}, Graph};
use separator::Mode::*;

fn main() {
    //let g = Graph::from_file("../Graphs/karlsruhe/first_out", "../Graphs/karlsruhe/head").unwrap();
    //let g = Graph::from_file("../Graphs/germany/first_out", "../Graphs/germany/head").unwrap();

    let g = generate_grid_with_avg_degree(400, 2.25);
    g.recurse_separator(Eco, Some("output/grid-avg-deg-2-25.txt"));

    //let g = generate_grid_with_avg_degree(400, 2.5);
    //g.recurse_separator(Eco, Some("output/grid-avg-deg-2-5.txt"));
    //
    //let g = generate_grid_with_avg_degree(400, 3.0);
    //g.recurse_separator(Eco, Some("output/grid-avg-deg-3.txt"));
    //
    //let g = generate_grid_with_avg_degree(400, 3.5);
    //g.recurse_separator(Eco, Some("output/grid-avg-deg-3-5.txt"));
    //
    //let g = generate_grid(400);
    //g.recurse_separator(Eco, Some("output/grid-avg-deg-4.txt"));

    //let g = generate_grid(300);
    //dbg!(g.get_average_degree());
}
