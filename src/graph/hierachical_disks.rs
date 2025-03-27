use geo::Point;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    graph::{delaunay, geometric_graph::GeometricGraph},
    library,
};

use super::planar::planarize;

type Edge = (Point, Point);

pub fn generate_circle_center_graph(
    points_per_level: &[usize],
    city_percentage: &[f64],
    radii: &[f64],
) -> GeometricGraph {
    let center = Point::new(100000.0, 100000.0);
    let edges = generate_circle_center_graph_rec(points_per_level, city_percentage, radii, center);
    let mut g = GeometricGraph::from_edges_point(&edges);
    planarize(&mut g);
    g
}

fn generate_circle_center_graph_rec(
    points_per_level: &[usize],
    city_percentage: &[f64],
    radii: &[f64],
    center: Point,
) -> Vec<Edge> {
    assert_eq!(points_per_level.len(), radii.len());
    assert_eq!(points_per_level.len(), city_percentage.len());

    let points = library::random_points_in_circle(center, radii[0], points_per_level[0]);
    let mut edges = delaunay::delaunay_edges(&points);

    if points_per_level.len() == 1 {
        return edges;
    }

    let new_centers = points.choose_multiple(
        &mut thread_rng(),
        (city_percentage[0] * points.len() as f64) as usize,
    );

    for center in new_centers {
        edges.append(&mut generate_circle_center_graph_rec(
            &points_per_level[1..],
            &city_percentage[1..],
            &radii[1..],
            *center,
        ));
    }

    edges
}

pub fn example1() -> GeometricGraph {
    let points_per_level = vec![200, 50, 10];
    let city_percentage = vec![0.4, 0.4, 0.2];
    let radii = vec![4000.0, 600.0, 50.0];

    generate_circle_center_graph(&points_per_level, &city_percentage, &radii)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    #[test]
    fn circle_center() {
        let points_per_level = vec![200, 50, 10];
        let city_percentage = vec![0.4, 0.4, 0.2];
        let radii = vec![4000.0, 600.0, 50.0];

        let g = super::generate_circle_center_graph(
            &points_per_level,
            &city_percentage,
            &radii,
        );
        g.graph.info();
        g.save(Path::new("output/circle_center"));
        g.graph.recurse_separator(crate::separator::Mode::Eco, None);
    }
}
