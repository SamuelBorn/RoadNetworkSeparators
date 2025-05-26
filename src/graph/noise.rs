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

fn should_place_point(p: &Point, perlin: &Perlin, scales: &[f64]) -> bool {
    let noise: f64 = scales
        .iter()
        .map(|s| perlin.get([p.x() * s + 3. * s, p.y() * s + 3. * s]) * 0.5 + 0.5)
        .product();

    noise > 0.5f64.powi(scales.len() as i32)
}

pub fn noise(n: usize) -> GeometricGraph {
    let mut p = Vec::with_capacity(n);
    let rng = &mut rand::thread_rng();
    let mut perlin = Perlin::new(rng.gen());

    let scales = vec![4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0, 1024.0];

    while p.len() < n {
        p.append(
            &mut library::random_points_in_circle(Point::new(0., 0.), 1., 1000)
                .into_par_iter()
                .filter(|p| should_place_point(p, &perlin, &scales))
                .collect::<Vec<Point>>(),
        );
    }
    // library::write_point_vec(Path::new("./output/noise_points"), &p);

    println!("Points");
    let mut g = delaunay(&p);
    println!("Delaunay");
    prune_graph_parallel(&mut g, 2.5);
    println!("Pruned");
    g.largest_connected_component()
}

mod tests {
    use super::*;

    #[test]
    fn noise_test() {
        let g = noise(1_000_000);
        g.inertial_flowcutter("tmp");
    }
}
