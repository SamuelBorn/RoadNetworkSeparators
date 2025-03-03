use geo::{polygon, Area, BooleanOps, BoundingRect, Contains, LineString, Point, Polygon};
use hashbrown::{HashMap, HashSet};
use rand::Rng;
use rand_distr::{Distribution, Exp, Uniform};
use rayon::{iter::Positions, prelude::*};
use std::{f64, path::Path};
use voronoice::{BoundingBox, Voronoi, VoronoiBuilder};

use super::{geometric_graph::GeometricGraph, Graph};

const SCALE_FACTOR: f64 = 1e6;

fn quantize(coord: geo::Coord<f64>) -> (usize, usize) {
    (
        (coord.x * SCALE_FACTOR) as usize,
        (coord.y * SCALE_FACTOR) as usize,
    )
}

fn get_bounding_box(poly: &Polygon) -> BoundingBox {
    let b = poly.bounding_rect().unwrap();
    let (x, y) = b.center().x_y();
    let center = voronoice::Point { x, y };
    let width = b.width() + 1.0;
    let height = b.height() + 1.0;
    BoundingBox::new(center, width, height)
}

fn random_polygon_point(poly: &Polygon) -> voronoice::Point {
    for _ in 0..1000 {
        let mut rng = rand::thread_rng();
        let bbox = poly.bounding_rect().unwrap();
        let min = bbox.min();
        let max = bbox.max();
        let x = rng.gen_range(min.x..max.x);
        let y = rng.gen_range(min.y..max.y);

        if poly.contains(&geo::Point::new(x, y)) {
            return voronoice::Point { x, y };
        }
    }
    unreachable!()
}

fn random_disk_point(x: f64, y: f64, radius: f64) -> voronoice::Point {
    let mut rng = rand::thread_rng();
    let distance = radius * rng.gen::<f64>().sqrt();
    let angle = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
    voronoice::Point {
        x: x + distance * angle.cos(),
        y: y + distance * angle.sin(),
    }
}

// get polygons from voronoi diagram, intersected with the poly
fn get_polygons(voronoi: &Voronoi, poly: &Polygon) -> Vec<Polygon> {
    voronoi
        .iter_cells()
        .filter_map(|cell| {
            let mut points = cell
                .iter_vertices()
                .map(|v| geo::Point::new(v.x, v.y))
                .collect::<LineString>();
            points.close();
            Polygon::new(points, vec![])
                .intersection(poly)
                .iter()
                .next()
                .cloned()
        })
        .collect()
}

pub fn subdivide_polgon_points(poly: &Polygon, points: Vec<voronoice::Point>) -> Vec<Polygon> {
    if points.len() < 3 {
        return vec![];
    }

    let voronoi = VoronoiBuilder::default()
        .set_sites(points)
        .set_bounding_box(get_bounding_box(poly))
        .build()
        .unwrap();

    let polygons = get_polygons(&voronoi, &poly);

    polygons
}

pub fn subdivide_polygon<D1: Distribution<f64>, D2: Distribution<f64>>(
    poly: &Polygon,
    n: usize,
    density: D1,
    radius: D2,
) -> Vec<Polygon> {
    let mut c = Vec::new();
    for _ in 0..n {
        let px = random_polygon_point(poly);
        //c.push(px);
        let (x, y) = (px.x, px.y);
        let alpha = density.sample(&mut rand::thread_rng());
        let r = radius.sample(&mut rand::thread_rng());
        let m = r.powf(alpha).ceil() as usize;
        c.push(px);
        for _ in 0..m {
            let p = random_disk_point(x, y, r);
            // not part of paper
            if poly.contains(&geo::Point::new(p.x, p.y)) {
                c.push(p);
            }
        }
    }
    subdivide_polgon_points(poly, c)
}

pub fn voronoi_roadnetwork() {
    let eps = 1e-6;
    let levels = 4;
    let centers = vec![
        Uniform::new(400.0, 400.0 + eps),
        Uniform::new(2.0, 10.0),
        Uniform::new(2.0, 30.0),
        Uniform::new(4.0, 20.0),
    ];
    let densities = vec![
        Uniform::new(0.2, 0.2 + eps),
        Uniform::new(0.5, 0.5 + eps),
        Uniform::new(0.7, 0.7 + eps),
        Uniform::new(0.0, eps),
    ];
    let radii = vec![
        Exp::new(0.01).unwrap(),
        Exp::new(0.1).unwrap(),
        Exp::new(2.0).unwrap(),
        Exp::new(f64::INFINITY).unwrap(),
    ];
    let fractions = vec![0.95, 0.9, 0.7, 0.0];
    let poly = polygon![
        (x: 0.0, y: 0.0),
        (x: 0.0, y: 1000.0),
        (x: 1000.0, y: 1000.0),
        (x: 1000.0, y: 0.0),
        (x: 0.0, y: 0.0),
    ];

    let mut edges: Vec<((usize, usize), (usize, usize))> = Vec::new();

    let mut s = vec![poly];
    for i in 0..3 {
        let m = (fractions[i] * s.len() as f64) as usize;
        s.select_nth_unstable_by(m, |a, b| {
            f64::total_cmp(&a.unsigned_area(), &b.unsigned_area())
        });
        s = s[0..=m]
            .par_iter()
            .flat_map(|p| {
                subdivide_polygon(
                    p,
                    centers[i].sample(&mut rand::thread_rng()) as usize,
                    //20,
                    densities[i],
                    radii[i],
                )
            })
            .collect();
        println!("{} polygons on level {}", s.len(), i);

        let mut new_edges: Vec<((usize, usize), (usize, usize))> = s
            .par_iter()
            .flat_map(|p| {
                let mut edges = Vec::new();
                p.exterior().lines().for_each(|l| {
                    let edge = (quantize(l.start), quantize(l.end));
                    edges.push(edge);
                });
                edges
            })
            .collect();

        edges.append(&mut new_edges);
    }

    let g = build_graph(edges);
    g.save(&Path::new("output/voronoi")).unwrap();
}

fn build_graph(mut edges: Vec<((usize, usize), (usize, usize))>) -> GeometricGraph {
    let mut pos: Vec<_> = edges.iter().flat_map(|(p1, p2)| vec![*p1, *p2]).collect();
    pos.sort();
    pos.dedup();
    let mapping = pos
        .iter()
        .enumerate()
        .map(|(i, p)| (*p, i))
        .collect::<HashMap<(usize, usize), usize>>();

    let mut graph = Graph::with_node_count(mapping.len());
    for (p1, p2) in edges {
        let u = *mapping.get(&p1).unwrap();
        let v = *mapping.get(&p2).unwrap();
        graph.add_edge(u, v);
    }

    let pos = pos
        .iter()
        .map(|(x, y)| Point::new(*x as f64, *y as f64))
        .collect();

    let mut g = GeometricGraph::new(graph, pos);
    g.graph.make_undirected();
    g
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn subdivision_works() {
        let poly = polygon![
            (x: 0.0, y: 0.0),
            (x: 0.0, y: 1000.0),
            (x: 1000.0, y: 1000.0),
            (x: 1000.0, y: 0.0),
            (x: 0.0, y: 0.0),
        ];

        let density = Uniform::new(0.2, 0.2001);
        let radius = Exp::new(0.01).unwrap();

        let polys = subdivide_polygon(&poly, 10, density, radius);
        //let polys = vec![poly];

        // print edge start point and end point
        for p in polys {
            for l in p.exterior().lines() {
                println!("{:?} {:?}", l.start, l.end);
            }
        }
    }

    #[test]
    fn voronoi_works() {
        voronoi_roadnetwork();
    }
}
