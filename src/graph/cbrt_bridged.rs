use std::{cmp::max, f64::consts::PI};

use geo::{ConcaveHull, ConvexHull, Intersects, Line, MultiPoint, Point, Polygon};
use hashbrown::HashSet;
use ordered_float::OrderedFloat;
use petgraph::unionfind::UnionFind;
use rand::{seq::SliceRandom, thread_rng, Rng};
use rand_distr::{Distribution, Exp, Normal, Uniform};
use rayon::prelude::*;
use rstar::PointDistance;

use crate::graph::geometric_graph::quantize;

use super::{delaunay::delaunay, geometric_graph::GeometricGraph, Graph};

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
                rng.gen_range(1000.0..width + 1000.0),
                rng.gen_range(1000.0..height + 1000.0),
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
                edges: super::delaunay::delaunay(&points).get_edges_points(),
                hull: MultiPoint(points.clone()).concave_hull(2.0),
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

fn scale(p: &Point, scale: f64) -> Point {
    Point::new(p.x() * scale, p.y() * scale)
}

fn bridge(s1: &Subgraph, s2: &Subgraph) -> Vec<(Point, Point)> {
    let dir_s1_to_s2 = s2.center - s1.center;
    let s1_exterior = s1.hull.exterior().points().collect::<Vec<_>>();
    let s1_facing_exterior = s1_exterior
        .into_par_iter()
        .filter(|&p| {
            let line = Line::new(p + scale(&dir_s1_to_s2, 1e-10), p + dir_s1_to_s2);
            !line.intersects(&s1.hull)
        })
        .collect::<Vec<_>>();

    let dir_s2_to_s1 = s1.center - s2.center;
    let s2_exterior = s2.hull.exterior().points().collect::<Vec<_>>();
    let s2_facing_exterior = s2_exterior
        .into_par_iter()
        .filter(|&p| {
            let line = Line::new(p + scale(&dir_s2_to_s1, 1e-10), p + dir_s2_to_s1);
            !line.intersects(&s2.hull)
        })
        .collect::<Vec<_>>();

    let cbrt = ((s1.node_count + s2.node_count) as f64).powf(1.0 / 3.0) as usize;
    let s1_anchors = s1_facing_exterior.choose_multiple(&mut thread_rng(), cbrt);
    let s2_anchors = s2_facing_exterior.choose_multiple(&mut thread_rng(), cbrt);

    let mut bridges = Vec::new();
    for (p1, p2) in s1_anchors.zip(s2_anchors) {
        bridges.push((*p1, *p2));
    }
    bridges
}

pub fn build_cbrt_bridged(n: usize, width: f64, height: f64) -> GeometricGraph {
    let mut subgraphs = generate_random_blobs(n, width, height);
    let centers = subgraphs.par_iter().map(|s| s.center).collect::<Vec<_>>();

    let delaunay = delaunay(&centers);
    let mut edges = delaunay.graph.get_edges();
    edges.par_sort_unstable_by_key(|&(u, v)| {
        OrderedFloat(delaunay.positions[u].distance_2(&delaunay.positions[v]))
    });

    let mut uf: UnionFind<usize> = UnionFind::new(n);

    for (u, v) in edges {
        let (u, v) = (uf.find(u), uf.find(v));
        if u != v {
            uf.union(u, v);
            let mut bridges = bridge(&subgraphs[u], &subgraphs[v]);

            let parent = uf.find(u);
            let child = if u == parent { v } else { u };

            let mut tmp = std::mem::take(&mut subgraphs[child].edges);
            subgraphs[parent].edges.append(&mut tmp);
            subgraphs[parent].edges.append(&mut bridges);
            subgraphs[parent].node_count += subgraphs[child].node_count;
            subgraphs[parent].hull = hull(&subgraphs[parent].hull, &subgraphs[child].hull);
        }
    }

    GeometricGraph::from_edges_point(&subgraphs[uf.find(0)].edges)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

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

    #[test]
    fn test_bridged() {
        let g = build_cbrt_bridged(1000, 1000.0, 1000.0);
        g.graph.info();
        g.save(Path::new("output/cbrt_bridged"));
    }
}
