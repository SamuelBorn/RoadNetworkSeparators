use std::path::Path;

use geo::Point;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::library;

use super::{
    delaunay::delaunay,
    example::example_c4,
    geometric_graph::GeometricGraph,
    voronoi::{prune_graph, prune_graph_parallel},
};

fn should_place_point(p: &Point, perlin: &Perlin, scale: Option<f64>) -> bool {
    let chance: f64 = rand::thread_rng().gen();

    let noise1 = (perlin.get([p.x() * 10., p.y() * 10.]) + 1.) * 0.5;
    let noise2 = (perlin.get([p.x() * 50. + 10., p.y() * 50. + 10.]) + 1.) * 0.5;
    // chance.powi(5) > noise1 * noise2
    noise1 * noise2 > 0.3
}

pub fn noise(n: usize, scale: Option<f64>) -> GeometricGraph {
    let mut p = Vec::with_capacity(n);
    let rng = &mut rand::thread_rng();
    let mut perlin = Perlin::new(rng.gen());

    while p.len() < n {
        p.append(
            &mut library::random_points_in_circle(Point::new(0., 0.), 1., 1000)
                .into_par_iter()
                .filter(|p| should_place_point(p, &perlin, None))
                .collect::<Vec<Point>>(),
        );
    }

    library::write_point_vec(Path::new("./output/noise_points"), &p);

    let mut g = delaunay(&p);
    prune_graph_parallel(&mut g, 2.5);
    // prune_graph(&mut g, 2.5);
    g.largest_connected_component()
}

mod tests {
    use super::*;
    use crate::{graph::geometric_graph::GeometricGraph, library};

    #[test]
    fn noise_test() {
        let g = noise(40_000, Some(10.));
    }
}
