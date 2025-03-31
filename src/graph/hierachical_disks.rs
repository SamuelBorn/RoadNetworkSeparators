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
    //planarize(&mut g);
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

pub fn generate_circle_center_graph_v2(
    points_per_level: &[usize],
    city_percentage: &[f64],
    radii: &[f64],
) -> GeometricGraph {
    let rng = &mut thread_rng();
    let mut centers = vec![Point::new(100000.0, 100000.0)];
    let mut edges = vec![];
    assert!(city_percentage[0] == 1.0);

    for i in 0..points_per_level.len() {
        let chosen = centers
            .choose_multiple(rng, (city_percentage[i] * centers.len() as f64) as usize)
            .cloned()
            .collect::<Vec<_>>();

        chosen.into_iter().for_each(|center| {
            let mut points =
                library::random_points_in_circle(center, radii[i], points_per_level[i]);
            points.push(center);
            edges.append(&mut delaunay::dynamic_length_restriced_delaunay(
                &points, 0.95,
            ));
            centers.append(&mut points);
        });
    }

    let mut g = GeometricGraph::from_edges_point(&edges);
    g.graph.info();
    planarize(&mut g);
    g
}

pub fn example1() -> GeometricGraph {
    //let points_per_level = vec![200, 50, 10];
    //let city_percentage = vec![0.4, 0.4, 0.0];
    //let radii = vec![4000.0, 600.0, 50.0];
    //
    //generate_circle_center_graph(&points_per_level, &city_percentage, &radii)
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    use geo::Point;

    use crate::library;

    #[test]
    fn length_overview() {
        let points = library::random_points_in_circle(Point::new(1e4, 1e4), 7.0, 50);
        let edges = delaunay::dynamic_length_restriced_delaunay(&points, 0.95);
        //let edges = delaunay::delaunay_edges(&points);
        let g = GeometricGraph::from_edges_point(&edges);
        g.save_edge_length_overview(Path::new("output/disks_length_overview"));
    }

    #[test]
    fn disks() {
        let city_percentage = vec![1.0, 0.7, 0.4, 0.1];
        let points_per_level = vec![200, 100, 40, 10];
        let radii = vec![350.0, 40.0, 6.0, 1.0];

        let g = super::generate_circle_center_graph_v2(&points_per_level, &city_percentage, &radii);
        g.graph.info();
        g.save(Path::new("output/graphs/disks"));

        //println!(""g.graph.get_separator_size(crate::separator::Mode::Fast));
        //g.graph.recurse_separator(crate::separator::Mode::Fast, None);
        //g.inertial_flowcutter("tmp2");
    }
}
