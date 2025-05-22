use std::path::Path;

use geo::Point;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::library;

use super::{
    delaunay::delaunay,
    example::example_c4,
    geometric_graph::GeometricGraph,
    voronoi::{prune_graph, prune_graph_parallel},
};

fn should_place_point(p: &Point, perlin: &Perlin, scale: Option<f64>) -> bool {
    let scale = scale.unwrap_or(10.0);
    let noise = perlin.get([p.x() * scale, p.y() * scale]);
    let chance = rand::thread_rng().gen_range(-1.0..1.0);
    let chance = chance * chance * chance;
    noise > chance
}

pub fn noise(n: usize, scale: Option<f64>) -> GeometricGraph {
    let mut p = Vec::with_capacity(n);
    let rng = &mut rand::thread_rng();
    let mut perlin = Perlin::new(0);

    while p.len() < n {
        let x = library::random_point_in_circle(Point::new(0., 0.), 1.);
        if should_place_point(&x, &perlin, scale) {
            p.push(x);
        }
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
        // let g = noise(50_000, Some(5.0));
        // g.inertial_flowcutter("noise");
        // g.visualize("noise");
        for i in 1..=10 {
            let g = noise(50_000, Some(i as f64));
            g.inertial_flowcutter(&format!("noise_{}", i));
        }
    }

    #[test]
    fn noise_test_scale() {}
}
