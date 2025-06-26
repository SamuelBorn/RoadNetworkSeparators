use super::{geometric_graph::GeometricGraph, relative_neighborhood::relative_neighborhood_points};
use crate::{
    graph::{
        delaunay,
        geometric_graph::approx_dedup_points,
        voronoi::{prune_graph, prune_graph_parallel},
    },
    library,
};
use geo::Point;
use rand::seq::SliceRandom;

pub fn pruned_hierachical_delaunay(
    city_percentage: &[f64],
    points_per_level: &[usize],
    radii: &[f64],
) -> GeometricGraph {
    let start = std::time::Instant::now();
    let mut g = generate_hierachical_delaunay(city_percentage, points_per_level, radii);
    // g.visualize("delaunay_pre_prune");
    println!("Delaunay took {} s", start.elapsed().as_secs());
    let start = std::time::Instant::now();
    // prune_graph(&mut g, 2.0);
    prune_graph_parallel(&mut g, 2.5);
    println!("Pruning took {} s", start.elapsed().as_secs());

    if !g.graph.is_connected() {
        g = g.largest_connected_component();
    }
    // g.visualize("delaunay_post_prune");
    g
}

pub fn random_pruned_hierachical_delaunay(
    city_percentage: &[f64],
    points_per_level: &[usize],
    radii: &[f64],
) -> GeometricGraph {
    let mut g = generate_hierachical_delaunay(city_percentage, points_per_level, radii);
    let goal_edges = g.graph.get_num_nodes() * 5 / 4;
    let edges_to_remove = g.graph.get_num_edges() - goal_edges;
    g.graph.remove_random_edges(edges_to_remove);
    g.largest_connected_component()
}

pub fn generate_hierachical_points(
    city_percentage: &[f64],
    points_per_level: &[usize],
    radii: &[f64],
) -> Vec<Point> {
    assert_eq!(city_percentage[0], 1.0);
    let rng = &mut rand::thread_rng();
    let mut points = vec![Point::new(1000.0, 1000.0)];

    for i in 0..city_percentage.len() {
        let chosen = points
            .choose_multiple(rng, (city_percentage[i] * points.len() as f64) as usize)
            .cloned()
            .collect::<Vec<_>>();

        for center in chosen {
            points.append(&mut library::random_points_in_circle(
                center,
                radii[i],
                points_per_level[i],
            ));
        }
    }

    points
}

pub fn generate_hierachical_relative_neighborhood(
    city_percentage: &[f64],
    points_per_level: &[usize],
    radii: &[f64],
) -> GeometricGraph {
    let points = generate_hierachical_points(city_percentage, points_per_level, radii);
    relative_neighborhood_points(points)
}

pub fn generate_hierachical_delaunay(
    city_percentage: &[f64],
    points_per_level: &[usize],
    radii: &[f64],
) -> GeometricGraph {
    let points = generate_hierachical_points(city_percentage, points_per_level, radii);
    delaunay::delaunay_points(&points)
}

pub fn example() -> GeometricGraph {
    let f1 = 1.0;
    let f2 = 0.5;
    let f3 = 0.3;
    let f4 = 0.1;

    let s1 = 300;
    let s2 = 150;
    let s3 = 75;
    let s4 = 35;

    let r1 = 1000.0;
    let r2 = 2.0 * r1 / (s1 as f64).sqrt();
    let r3 = 1.5 * r2 / (s2 as f64).sqrt();
    let r4 = 1.3 * r3 / (s3 as f64).sqrt();

    // generate_hierachical_delaunay(&[f1, f2, f3, f4], &[s1, s2, s3, s4], &[r1, r2, r3, r4])
    generate_hierachical_relative_neighborhood(
        &[f1, f2, f3, f4],
        &[s1, s2, s3, s4],
        &[r1, r2, r3, r4],
    )
}

#[cfg(test)]
mod test {
    use crate::graph::example::example1;

    use super::*;

    #[test]
    fn simple_generate_hierachical_delaunay() {
        let city_percentage = vec![1.0, 0.3, 0.6, 0.5];
        let points_per_level = vec![70, 30, 30, 30];
        let radii = vec![750., 200., 30., 3.];
        let g = generate_hierachical_delaunay(&city_percentage, &points_per_level, &radii);

        g.visualize("hierachical_delaunay");
        g.graph.info();
        g.graph
            .recurse_separator(crate::separator::Mode::Fast, None);
    }

    #[test]
    fn simple_generate_hierachical_relative_neighborhood() {
        let g = example();
        g.visualize("hierachical_relative_neighborhood");
    }
}
