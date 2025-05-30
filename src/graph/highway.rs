use geo::{Distance, Euclidean, Point};
use hashbrown::{HashMap, HashSet};
use ordered_float::Pow;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Exp, Normal, Uniform};
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use rstar::PointDistance;
use rstar::RTree;

use super::{geometric_graph::GeometricGraph, Graph};

const EPS: f64 = 1e-8;
const SCALE: f64 = 1e8;

fn quantize(p: &geo::Point) -> (i128, i128) {
    ((p.x() * SCALE) as i128, (p.y() * SCALE) as i128)
}

fn inv_quantize((x, y): (i128, i128)) -> Point {
    Point::new(x as f64 / SCALE, y as f64 / SCALE)
}

fn pick_random_point_in_square(width: f64, height: f64) -> Point {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0..width);
    let y = rng.gen_range(0.0..height);
    geo::Point::new(x, y)
}

fn pick_random_points_city_like(width: f64, height: f64, n: usize) -> Vec<Point> {
    let s = width.max(height);
    let mut total_population = 0.0;
    let mut city_centers = Vec::new();

    // pick city centers
    while (total_population < n as f64) {
        let p = pick_random_point_in_square(width, height);
        let radius = Exp::new(1.0 / (0.05 * s))
            .unwrap()
            .sample(&mut thread_rng());
        let population: f64 = radius.pow(1.1);
        total_population += population;
        city_centers.push((p, radius, population));
    }

    // pick nodes around city centers
    let mut points = Vec::with_capacity(n);
    while (!city_centers.is_empty()) {
        let i = rand::thread_rng().gen_range(0..city_centers.len());
        let (center, radius, population) = city_centers.get_mut(i).unwrap();
        *population -= 1.0;

        let angle = rand::thread_rng().gen_range(0.0..2.0 * std::f64::consts::PI);
        let distance = Normal::new(0.0, *radius).unwrap().sample(&mut thread_rng());
        points.push(Point::new(
            center.x() + distance * angle.cos(),
            center.y() + distance * angle.sin(),
        ));

        if *population < 0.0 {
            city_centers.swap_remove(i);
        }
    }

    points
}

pub fn build_highway_network(n: usize) -> GeometricGraph {
    let s = 10000.0;
    let s_height = 0.75 * s;
    let levels = 25 as usize;
    let d = 2_usize.pow(levels as u32);
    let k = 2.0_f64.sqrt();

    let mut c_spatial: Vec<RTree<Point>> = vec![RTree::new(); levels];
    let mut c = vec![vec![]; levels];
    let mut e = Vec::with_capacity(n * 3);

    let random_points = pick_random_points_city_like(s, s_height, n);
    let pows = (1..=levels)
        .map(|i| 2usize.pow(i as u32) as f64)
        .collect::<Vec<_>>();

    for t in (0..n) {
        dbg!(t);
        let vt = random_points[t];
        for i in (0..=levels) {
            if i == levels || {
                let nearest = c_spatial[i].nearest_neighbor(&vt);
                nearest.is_some() && Euclidean::distance(vt, *nearest.unwrap()) <= pows[i]
            } {
                for j in (0..i) {
                    //for &w in c[j].iter() {
                    //    if Euclidean::distance(vt, w) <= k * pows[j] {
                    //        e.push((vt, w));
                    //    }
                    //}
                    e.extend(
                        c[j].par_iter()
                            .filter(|&&w| Euclidean::distance(vt, w) < k * pows[j])
                            .map(|&w| (vt, w))
                            .collect::<Vec<_>>(),
                    );

                    c_spatial[j].insert(vt);
                    c[j].push(vt);
                }

                if i < levels {
                    if let Some(nearest) = c_spatial[i].nearest_neighbor(&vt) {
                        e.push((vt, *nearest));
                    }
                }
                break;
            }
        }
    }

    build_graph_from_position_edges(&e)
}

fn build_graph_from_position_edges(edges: &[(Point, Point)]) -> GeometricGraph {
    let mut unique_points = edges
        .par_iter()
        .flat_map(|(p1, p2)| vec![quantize(p1), quantize(p2)])
        .collect::<Vec<_>>();
    unique_points.par_sort();
    unique_points.dedup();

    let p2i = unique_points
        .par_iter()
        .enumerate()
        .map(|(idx, p)| (p, idx))
        .collect::<HashMap<_, _>>();

    let e = edges
        .par_iter()
        .map(|(p1, p2)| {
            (
                *p2i.get(&quantize(p1)).unwrap(),
                *p2i.get(&quantize(p2)).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let g = Graph::from_edge_list(e);
    let pos = unique_points.par_iter().map(|&p| inv_quantize(p)).collect();
    GeometricGraph::new(g, pos)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_highway_network() {
        let g = build_highway_network(10_000);
        g.visualize("highway");
    }
}
