use std::path::Path;

use geo::Point;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::library;

use super::{delaunay::delaunay, example::example_c4, geometric_graph::GeometricGraph, voronoi::prune_graph_parallel};

// assumes x, y are in [0, 1]
fn should_place_point(x: f64, y: f64, perlin: &Perlin) -> bool {
    let scale = 10.;
    let noise = perlin.get([x * scale, y * scale]);
    let chance = rand::thread_rng().gen_range(-1.0..1.0);
    noise > chance
}

pub fn noise(n: usize) -> GeometricGraph {
    let mut p = Vec::with_capacity(n);
    let rng = &mut rand::thread_rng();
    let mut perlin = Perlin::new(0);

    while p.len() < n {
        let x = rng.gen();
        let y = rng.gen();

        if should_place_point(x, y, &perlin) {
            p.push(Point::new(x, y));
        }
    }

    library::write_point_vec(Path::new("./output/noise_points"), &p);

    let mut g = delaunay(&p);
    prune_graph_parallel(&mut g, 2.5);
    g.largest_connected_component()
}

mod tests {
    use super::*;
    use crate::{graph::geometric_graph::GeometricGraph, library};

    #[test]
    fn noise_test() {
        let g = noise(100_000);
        g.inertial_flowcutter("noise");
    }
}
