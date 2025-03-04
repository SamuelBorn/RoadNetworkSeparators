use std::path::Path;

use rayon::prelude::*;

use geo::Point;
use geo::{
    polygon, Area, BooleanOps, BoundingRect, Centroid, Contains, Distance, Euclidean, LineString,
};

use crate::graph::Graph;

use super::geometric_graph::GeometricGraph;

fn get_subgrid_nodes(
    g: &mut GeometricGraph,
    width: usize,
    bl: usize,
    tl: usize,
    tr: usize,
    br: usize,
) -> Vec<Vec<usize>> {
    let mut nodes = vec![vec![0; width]; width];
    for i in 0..width {
        for j in 0..width {
            nodes[i][j] = if i == 0 && j == 0 {
                bl
            } else if i == 0 && j == width - 1 {
                tl
            } else if i == width - 1 && j == 0 {
                br
            } else if i == width - 1 && j == width - 1 {
                tr
            } else {
                let v = i as f64 / (width - 1) as f64;
                let u = j as f64 / (width - 1) as f64;

                let x = (1.0 - u) * (1.0 - v) * g.positions[bl].x()
                    + u * (1.0 - v) * g.positions[tl].x()
                    + (1.0 - u) * v * g.positions[br].x()
                    + u * v * g.positions[tr].x();

                let y = (1.0 - u) * (1.0 - v) * g.positions[bl].y()
                    + u * (1.0 - v) * g.positions[tl].y()
                    + (1.0 - u) * v * g.positions[br].y()
                    + u * v * g.positions[tr].y();

                g.add_position_with_new_node(Point::new(x, y))
            };
        }
    }
    nodes
}

fn recurse_grid(
    g: &mut GeometricGraph,
    width: usize,
    level: usize,
    bl: usize,
    tl: usize,
    tr: usize,
    br: usize,
) {
    let subgrid = get_subgrid_nodes(g, width, bl, tl, tr, br);
    for i in 0..width {
        for j in 0..width {
            if i > 0 {
                g.graph.add_edge(subgrid[i][j], subgrid[i - 1][j]);
            }
            if j > 0 {
                g.graph.add_edge(subgrid[i][j], subgrid[i][j - 1]);
            }
            if i > 0 && j > 0 && level > 1 {
                recurse_grid(
                    g,
                    width,
                    level - 1,
                    subgrid[i - 1][j - 1],
                    subgrid[i - 1][j],
                    subgrid[i][j],
                    subgrid[i][j - 1],
                );
            }
        }
    }
}

pub fn build_nested_grid(width: usize, levels: usize) -> GeometricGraph {
    let g = Graph::from_edge_list(vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let positions = vec![
        Point::new(0.0, 0.0),
        Point::new(0.0, 10000.0),
        Point::new(10000.0, 10000.0),
        Point::new(10000.0, 0.0),
    ];
    let mut g = GeometricGraph::new(g, positions);
    recurse_grid(&mut g, width, levels, 0, 1, 2, 3);
    g
}

pub fn analyze_separators() {
    //const TARGET_NUM_NODES: usize = 500_000;
    //for level in (3..8) {
    //    for width in (0..100) {
    //        let g = build_nested_grid(width as usize, level);
    //        println!(
    //            "Level: {}, Width: {}, Nodes: {}",
    //            level,
    //            width,
    //            g.graph.get_num_nodes()
    //        );
    //        if g.graph.get_num_nodes() > TARGET_NUM_NODES {
    //            break;
    //        }
    //    }
    //}

    vec![(3, 10), (4, 6), (6, 4), (7, 4)]
        .into_par_iter()
        .for_each(|(level, width)| {
            let g = build_nested_grid(width, level);
            println!(
                "Level: {}, Width: {}, Nodes: {}",
                level,
                width,
                g.graph.get_num_nodes()
            );
            g.graph.recurse_separator(
                crate::separator::Mode::Strong,
                Some(Path::new(&format!("output/nested_grid_level_{}", level))),
            );
        });
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::graph::{example::example_c4, Graph};
    use geo::Point;

    #[test]
    fn test_get_subgrid_nodes() {
        let mut g = example_c4();
        let nodes = get_subgrid_nodes(&mut g, 3, 0, 1, 2, 3);
        assert_eq!(nodes[0][0], 0);
        assert_eq!(nodes[0][2], 1);
        assert_eq!(nodes[2][2], 2);
        assert_eq!(nodes[2][0], 3);
        assert!(Euclidean::distance(Point::new(0.5, 0.0), dbg!(g.positions[nodes[1][0]])) < 1e-6);
        assert!(Euclidean::distance(Point::new(0.0, 0.5), dbg!(g.positions[nodes[0][1]])) < 1e-6);
        assert!(Euclidean::distance(Point::new(0.5, 0.5), dbg!(g.positions[nodes[1][1]])) < 1e-6);
        assert!(Euclidean::distance(Point::new(0.5, 1.0), dbg!(g.positions[nodes[1][2]])) < 1e-6);
        assert!(Euclidean::distance(Point::new(1.0, 0.5), dbg!(g.positions[nodes[2][1]])) < 1e-6);
    }

    #[test]
    fn test_nested_grid() {
        let g = build_nested_grid(3, 2);
        assert_eq!(g.graph.get_num_nodes(), 5 * 4 + 9);
        g.save(Path::new("output/nested_grid"));
    }
}
