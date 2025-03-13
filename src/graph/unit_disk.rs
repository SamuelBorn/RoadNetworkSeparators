use hashbrown::{HashMap, HashSet};

use rand::Rng;

use crate::graph::Graph;

pub fn generate_unit_disk_graph(n: usize, area_size: Option<f64>) -> Graph {
    let mut rng = rand::thread_rng();
    let mut g = Graph::with_node_count(n);

    let area_size = area_size.unwrap_or((n as f64).sqrt());
    let mut grid = HashMap::new();

    let positions: Vec<(f64, f64)> = (0..n)
        .map(|i| {
            let x = rng.gen_range(0.0..area_size);
            let y = rng.gen_range(0.0..area_size);
            let cell = (x.floor() as i32, y.floor() as i32);
            grid.entry(cell).or_insert_with(Vec::new).push((i, (x, y)));
            (x, y)
        })
        .collect();

    let neighbor_offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (i, (x, y)) in positions.iter().enumerate() {
        let cell = (x.floor() as i32, y.floor() as i32);
        for (dx, dy) in neighbor_offsets.iter() {
            let neighbor_cell = (cell.0 + dx, cell.1 + dy);
            if let Some(neighbors) = grid.get(&neighbor_cell) {
                for (i2, (x2, y2)) in neighbors.iter() {
                    let distance = ((x - x2).powi(2) + (y - y2).powi(2)).sqrt();
                    if distance < 1.0 {
                        g.add_edge(i, *i2);
                    }
                }
            }
        }
    }

    g.largest_connected_component()
}

pub fn generate_unit_disk_graph_with_avg_degree(
    n: usize,
    area_size: Option<f64>,
    avg_degree: f64,
) -> Graph {
    let mut g = generate_unit_disk_graph(n, area_size);
    let num_edges = g.get_num_edges();
    let mut rng = rand::thread_rng();

    let goal_num_edges = (avg_degree * g.get_num_nodes() as f64 / 2.0) as usize;

    g.remove_random_edges(num_edges - goal_num_edges);

    g.largest_connected_component()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_unit_disk_graph() {
        return;
        let g = generate_unit_disk_graph(3000, Some(43.0));
        println!("{}", g.get_average_degree());
    }

    #[test]
    fn test_generate_grid_with_avg_degree() {
        return;
        let g = generate_unit_disk_graph_with_avg_degree(3000, Some(40.0), 3.4);
        println!("{}", g.get_average_degree());
        g.recurse_separator(crate::separator::Mode::Eco, None)
    }
}
