use geo::{
    polygon, Area, BooleanOps, BoundingRect, Centroid, Contains, Distance, Euclidean, LineString,
    Point, Polygon,
};
use hashbrown::{HashMap, HashSet};
use petgraph::unionfind::UnionFind;
use rand::Rng;
use rand_distr::{Distribution, Exp, Uniform};
use rayon::{iter::Positions, prelude::*};
use rstar::PointDistance;
use std::{f64, path::Path};
use voronoice::{BoundingBox, Voronoi, VoronoiBuilder};

use super::{geometric_graph::GeometricGraph, Graph};

const SCALE_FACTOR: f64 = 1e8;

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

    // if we can't find a point in 1000 tries, just return the centroid
    let p = poly.centroid().unwrap();
    voronoice::Point { x: p.x(), y: p.y() }
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

pub fn subdivide_polgon_points(poly: &Polygon, mut points: Vec<voronoice::Point>) -> Vec<Polygon> {
    // deduplicate points
    points.sort_by(|a, b| a.x.total_cmp(&b.x));
    points.dedup_by(|a, b| (a.x - b.x) < 1e-6 && (a.y - b.y) < 1e-6);

    if points.len() < 3 {
        return vec![];
    }

    let voronoi = VoronoiBuilder::default()
        .set_sites(points)
        .set_bounding_box(get_bounding_box(poly))
        .build();

    match voronoi {
        Some(v) => get_polygons(&v, poly),
        None => vec![],
    }
}

//pub fn subdivide_polygon<D1: Distribution<f64>>(
//    poly: &Polygon,
//    n: usize,
//    density: f64,
//    radius: D1,
pub fn subdivide_polygon(
    poly: &Polygon,
    n: usize,
) -> Vec<Polygon> {
    //let mut c = Vec::new();
    //for _ in 0..n {
    //    let px = random_polygon_point(poly);
    //    let (x, y) = (px.x, px.y);
    //    let alpha = density;
    //    let r = radius.sample(&mut rand::thread_rng());
    //    let m = r.powf(alpha).ceil() as usize;
    //    c.push(px);
    //    for _ in 0..m {
    //        let p = random_disk_point(x, y, r);
    //        // not part of paper
    //        if poly.contains(&geo::Point::new(p.x, p.y)) {
    //            c.push(p);
    //        }
    //    }
    //}
    let mut c = (0..n)
        .into_par_iter()
        .map(|_| random_polygon_point(poly))
        .collect::<Vec<_>>();
    subdivide_polgon_points(poly, c)
}

pub fn voronoi_roadnetwork() {
    let eps = 1e-6;
    let levels = 5;
    let centers = vec![
        Uniform::new(1700.0, 1700.0 + eps),
        Uniform::new(2.0, 30.0),
        Uniform::new(2.0, 30.0),
        Uniform::new(2.0, 30.0),
        Uniform::new(4.0, 30.0),
    ];
    //let centers = vec![
    //    Uniform::new(1700.0, 1700.0 + eps),
    //    Uniform::new(2.0, 40.0),
    //    Uniform::new(2.0, 70.0),
    //    Uniform::new(4.0, 40.0),
    //];
    // mean of densities and radii combined
    let densities = vec![0.2, 0.5, 0.9, 0.0];
    let radii = vec![
        Exp::new(0.01).unwrap(),          // 100^0.2 = 2.5
        Exp::new(0.1).unwrap(),           // 10^0.5 = 3.2
        Exp::new(2.0).unwrap(),           // 2^0.9 = 1.3
        Exp::new(f64::INFINITY).unwrap(), // 0
    ];
    let fractions = vec![0.95, 0.9, 0.7, 0.5, 0.0];
    let poly = polygon![
        (x: 0.0, y: 0.0),
        (x: 0.0, y: 100000.0),
        (x: 100000.0, y: 100000.0),
        (x: 100000.0, y: 0.0),
        (x: 0.0, y: 0.0),
    ];

    let mut edges: Vec<((usize, usize), (usize, usize))> = Vec::new();

    let mut s = vec![poly];
    for i in 0..4 {
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
                    //densities[i],
                    //radii[i],
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

    let mut g = build_graph(edges);
    g.graph.info();
    println!("Graph build");
    prune_graph(&mut g, 4.0);
    g.graph.info();
    println!("Graph pruned");
    g.save(&Path::new("output/voronoi")).unwrap();
}

fn build_graph(mut edges: Vec<((usize, usize), (usize, usize))>) -> GeometricGraph {
    let mut pos: Vec<_> = edges.iter().flat_map(|(p1, p2)| vec![*p1, *p2]).collect();
    pos.par_sort();
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
    g
}

// compute a sparse graph spanner of the graph computed in phase one. Given a graph G a graph spanner H of G with stretch t is
// a subgraph of G such that for each pair of vertices u, v in G we have distH (u, v) ≤ t · distG (u, v).
// Paper used t = 4
pub fn prune_graph(g: &mut GeometricGraph, spanning_parameter: f64) {
    let mut uf: UnionFind<usize> = UnionFind::new(g.graph.get_num_nodes() + 1);
    let edge_lengths = g.get_edge_lengths();
    let mut sorted = edge_lengths.iter().collect::<Vec<_>>();
    sorted.par_sort_by(|(_, l1), (_, l2)| l1.partial_cmp(l2).unwrap());

    for ((u, v), length) in sorted {
        g.graph.remove_edge(*u, *v);
        if uf.find(*u) != uf.find(*v)
            || !g.connected_with_prune_distance(*u, *v, length * spanning_parameter, &edge_lengths)
        {
            g.graph.add_edge(*u, *v);
        }
        uf.union(*u, *v);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn voronoi_works() {
        voronoi_roadnetwork();
    }
}
