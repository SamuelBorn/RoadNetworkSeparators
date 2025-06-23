use std::{iter, path::Path};

use chrono::Local;
use geo::Point;
use noise::{NoiseFn, Perlin};
use rand::{thread_rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::prelude::*;

use crate::{graph::relative_neighborhood::relative_neighborhood_points, library};

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

fn should_place_point_root_normalized(p: &Point, perlin: &Perlin, scales: &[f64]) -> bool {
    let noise: f64 = scales
        .iter()
        .map(|s| perlin.get([p.x() * s + 3. * s, p.y() * s + 3. * s]) * 0.5 + 0.5)
        .product();

    let normalized = noise.powf(1.0 / scales.len() as f64);
    let random = thread_rng().gen_range(0.0..1.0);

    normalized > random
}

fn should_place_point_pink_noise(p: &Point, perlin: &Perlin, scales: &[f64]) -> bool {
    let noise = scales
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            let noise = perlin.get([p.x() * s + 3. * s, p.y() * s + 3. * s]) * 0.5 + 0.5;
            noise * 1.0 / 2f64.powi(i as i32 + 1)
        })
        .sum::<f64>();

    thread_rng().gen_bool(noise)
}

pub fn get_noise_points(n: usize) -> Vec<Point> {
    let scales = [8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0, 1024.0];
    get_noise_points_scales(n, &scales)
}

pub fn get_noise_points_scales(n: usize, scales: &[f64]) -> Vec<Point> {
    let perlin = Perlin::new(rand::thread_rng().gen());
    iter::repeat(())
        .par_bridge()
        .filter_map(|()| {
            let candidate = library::random_point_in_circle(Point::new(0., 0.), 1.);

            if should_place_point(&candidate, &perlin, scales) {
                Some(candidate)
            } else {
                None
            }
        })
        .take_any(n)
        .collect()
}

pub fn noise(n: usize) -> GeometricGraph {
    let scales = [
        4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0, 1024.0, 2048.0,
    ];
    noise_scales(n, &scales)
}

pub fn noise_scales(n: usize, scales: &[f64]) -> GeometricGraph {
    let p = get_noise_points_scales(n, scales);
    println!("{}\tPoints sampled", Local::now());
    let g = relative_neighborhood_points(p);
    println!("{}\tRelative Neighborhood generated", Local::now());
    g
}

mod tests {
    use super::*;

    #[test]
    fn noise_test() {
        let g = noise(1_000_000);
        // g.inertial_flowcutter("tmp");
    }
}
