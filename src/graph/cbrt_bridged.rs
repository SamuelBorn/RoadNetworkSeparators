use std::{cmp::max, f64::consts::PI};

use geo::{ConvexHull, MultiPoint, Point, Polygon};
use hashbrown::HashSet;
use ordered_float::OrderedFloat;
use petgraph::unionfind::UnionFind;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Exp, Normal, Uniform};
use rayon::prelude::*;
use rstar::PointDistance;

use crate::graph::geometric_graph::quantize;

use super::delaunay::delaunay;

struct Subgraph {
    center: Point,
    node_count: usize,
    edges: Vec<(Point, Point)>,
    hull: Polygon,
}

fn generate_random_blobs(n: usize, width: f64, height: f64) -> Vec<Subgraph> {
    let r = Normal::new(0.0, 1.0).unwrap();

    (0..n)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let p = Point::new(
                rng.gen_range(100.0..width + 100.0),
                rng.gen_range(100.0..height + 100.0),
            );
            let mut population = Normal::new(80.0, 20.0).unwrap().sample(&mut rng);
            let mut radius = Normal::new(0.0, (population / PI).sqrt()).unwrap();

            let points = (0..population as usize)
                .into_iter()
                .map(|_| {
                    let angle = rng.gen_range(0.0..2.0 * PI);
                    let distance = radius.sample(&mut rng);
                    Point::new(
                        (p.x() + distance * angle.cos()).max(0.0),
                        (p.y() + distance * angle.sin()).max(0.0),
                    )
                })
                .collect::<Vec<_>>();

            Subgraph {
                center: p,
                node_count: points.len(),
                edges: super::delaunay::convex_delaunay(&points).get_edges_points(),
                hull: MultiPoint(points.clone()).convex_hull(),
            }
        })
        .collect()
}

fn hull(p1: &Polygon, p2: &Polygon) -> Polygon {
    let mut all = Vec::new();
    all.extend(p1.exterior().points());
    all.extend(p2.exterior().points());
    MultiPoint::new(all).convex_hull()
}

fn filter_hull(p: &Polygon, quant_hull_points: &HashSet<(i64, i64)>) -> Vec<Point> {
    p.exterior().lines().for_each(|line| {
        let start_on_hull = quant_hull_points.contains(&quantize(&line.start.into()));
        let end_on_hull = quant_hull_points.contains(&quantize(&line.end.into()));

        match (start_on_hull, end_on_hull) {
            (false, false) => {
                center.push(line.start.into());
            }
            (false, true) => {
                center.push(line.start.into());
            }
            (true, false) => {
                border1 = line.start.into();
            }
            (true, true) => {}
        }
    })

    unimplemented!();
}

fn bridge(s1: &Subgraph, s2: &Subgraph) -> (Polygon, Vec<(Point, Point)>) {
    let hull = hull(&s1.hull, &s2.hull);
    let quant_hull_points = hull
        .exterior()
        .points()
        .map(|p| quantize(&p))
        .collect::<HashSet<_>>();

    unimplemented!();
}

pub fn build_cbrt_bridged(n: usize, width: f64, height: f64) {
    let mut subgraphs = generate_random_blobs(n, width, height);
    let centers = subgraphs.par_iter().map(|s| s.center).collect::<Vec<_>>();

    let delaunay = delaunay(&centers);
    let mut edges = delaunay.graph.get_edges();
    edges.par_sort_unstable_by_key(|&(u, v)| {
        OrderedFloat(delaunay.positions[u].distance_2(&delaunay.positions[v]))
    });

    let mut uf: UnionFind<usize> = UnionFind::new(n);

    for (u, v) in edges {
        if uf.union(u, v) {
            let (hull, mut bridges) = bridge(&subgraphs[u], &subgraphs[v]);

            let parent = uf.find(u);
            let child = if u == parent { v } else { u };

            let mut tmp = std::mem::take(&mut subgraphs[child].edges);
            subgraphs[parent].edges.append(&mut tmp);
            subgraphs[parent].edges.append(&mut bridges);
            subgraphs[parent].node_count += subgraphs[child].node_count;
            subgraphs[parent].hull = hull;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blobs() {
        let blobs = generate_random_blobs(100, 500.0, 500.0);
        for blob in blobs {
            for edge in blob.edges {
                println!("{:?}", edge);
            }
        }
    }
}
