use super::geometric_graph::GeometricGraph;
use crate::{
    graph::{delaunay, geometric_graph::approx_dedup_points, voronoi::prune_graph},
    library,
};
use geo::Point;
use rand::seq::SliceRandom;

pub fn generate_hierachical_delaunay(
    city_percentage: &[f64],
    points_per_level: &[usize],
    radii: &[f64],
) -> GeometricGraph {
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

    //let mut g = delaunay::delaunay(&points);
    //prune_graph(&mut g, 4.0);
    //g
    delaunay::delaunay(&points)
}

#[cfg(test)]
mod test {
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
}
