use geo::{
    polygon, Area, BooleanOps, BoundingRect, Centroid, Contains, Distance, Euclidean, LineString,
    Point, Polygon,
};
use hashbrown::{HashMap, HashSet};
use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;
use petgraph::{algo::dijkstra, unionfind::UnionFind};
use rand::{seq::SliceRandom, Rng};
use rand_distr::{Distribution, Exp, Uniform};
use rayon::{iter::Positions, prelude::*};
use rstar::PointDistance;
use std::{cmp::Ordering, f64, path::Path, sync::Arc};
use voronoice::{BoundingBox, Voronoi, VoronoiBuilder};

use crate::{bidirectional, library};

use super::{delaunay, geometric_graph::GeometricGraph, Graph};

const SCALE: f64 = 1e8;
const EPS: f64 = 1e-8;

fn quantize(coord: geo::Coord<f64>) -> (usize, usize) {
    ((coord.x * SCALE) as usize, (coord.y * SCALE) as usize)
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
    points.dedup_by(|a, b| (a.x - b.x) < EPS && (a.y - b.y) < EPS);

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
pub fn subdivide_polygon(poly: &Polygon, n: usize) -> Vec<Polygon> {
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
        .map(|(x, y)| Point::new(*x as f64 / SCALE, *y as f64 / SCALE))
        .collect();

    GeometricGraph::new(graph, pos)
}

pub fn prune_graph(g: &mut GeometricGraph, dist_multiplier: f64) {
    let edge_lengths = g.get_edge_lengths();
    let mut edges = g
        .get_edge_lengths_unidirectional()
        .into_iter()
        .collect::<Vec<_>>();
    edges.par_sort_by(|(_, l1), (_, l2)| l1.partial_cmp(l2).unwrap());
    let m = edges.len();

    for (i, ((u, v), length)) in edges.into_iter().enumerate() {
        if i % 10000 == 0 {
            println!("{} / {}", i, m);
        }

        // g.graph.remove_edge(u, v);
        // if !g.dijkstra_less_than(u, v, length * dist_multiplier, &edge_lengths) {
        if g.dijkstra_less_than_ignore_edge(u, v, length * dist_multiplier, &edge_lengths) {
            // g.graph.add_edge(u, v);
            g.graph.remove_edge(u, v);
        }
    }
}

pub fn prune_graph_parallel(g: &mut GeometricGraph, dist_multiplier: f64) {
    let edge_lengths = g.get_edge_lengths();
    let mut edges = g
        .get_edge_lengths_unidirectional()
        .into_iter()
        .collect::<Vec<_>>();
    edges.par_sort_by(|(_, l1), (_, l2)| l1.partial_cmp(l2).unwrap());
    let m = edges.len();
    let threads = rayon::current_num_threads().max(1);

    for chunk in edges.chunks(threads) {
        let removal = chunk
            .into_par_iter()
            .filter(|((u, v), length)| {
                g.dijkstra_less_than_ignore_edge(*u, *v, length * dist_multiplier, &edge_lengths)
            })
            .collect::<Vec<_>>();

        removal.into_iter().for_each(|((u, v), _)| {
            g.graph.remove_edge(*u, *v);
        });
    }
}

pub fn pruned_delaunay(points: &[Point], dist_multiplier: f64) -> GeometricGraph {
    let mut g = delaunay::delaunay_points(points);
    prune_graph_parallel(&mut g, dist_multiplier);
    g.graph.info();
    g
}

// compute a sparse graph spanner of the graph computed in phase one. Given a graph G a graph spanner H of G with stretch t is
// a subgraph of G such that for each pair of vertices u, v in G we have distH (u, v) ≤ t · distG (u, v).
// Paper used t = 4
pub fn prune_graph_spanner(g: &mut GeometricGraph, spanning_parameter: f64) {
    let mut uf: UnionFind<usize> = UnionFind::new(g.graph.get_num_nodes() + 1);
    let edge_lengths = g.get_edge_lengths();
    let directed_edges = g.get_edge_lengths_unidirectional();
    let mut edges = directed_edges.iter().collect::<Vec<_>>();
    edges.par_sort_by(|(_, l1), (_, l2)| l2.partial_cmp(l1).unwrap());

    let mut h = GeometricGraph::new(
        Graph::with_node_count(g.graph.get_num_nodes()),
        g.positions.clone(),
    );

    for (i, &(&(u, v), length)) in edges.iter().enumerate() {
        if i % 10000 == 0 {
            println!(
                "Pruning progress\t{:.2}%",
                i as f64 / edges.len() as f64 * 100.
            );
        }

        if uf.union(u, v) || !h.dijkstra_less_than(u, v, spanning_parameter * length, &edge_lengths)
        {
            h.graph.add_edge(u, v);
        }
    }

    // swap g and h
    g.graph = h.graph;
    g.positions = h.positions;
}

// spanning = spanning parameter, e.g. 4.0 how much longer the path in the spanner can be compared to the original path
pub fn prune_graph_spanner_parallel_approx(g: &mut GeometricGraph, spanning: f64) {
    let mut uf: UnionFind<usize> = UnionFind::new(g.graph.get_num_nodes() + 1);
    let edge_lengths = g.get_edge_lengths();
    let directed_edges = g.get_edge_lengths_unidirectional();
    let mut edges = directed_edges.iter().collect::<Vec<_>>();
    edges.par_sort_by(|(_, l1), (_, l2)| l2.partial_cmp(l1).unwrap());

    let mut h = GeometricGraph::new(
        Graph::with_node_count(g.graph.get_num_nodes()),
        g.positions.clone(),
    );

    edges
        .chunks(4 * rayon::current_num_threads())
        .progress_with_style(library::pb_style())
        .for_each(|chunk| {
            let add_edges = chunk
                .into_par_iter()
                .filter_map(|(&(u, v), &length)| {
                    if !uf.equiv(u, v)
                        // || !h.astar_less_than(u, v, spanning * length, &edge_lengths)
                        || !h.dijkstra_less_than(u, v, spanning * length, &edge_lengths)
                    {
                        Some((u, v))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            for (u, v) in add_edges {
                h.graph.add_edge(u, v);
                uf.union(u, v);
            }
        });

    g.graph = h.graph;
    g.positions = h.positions;
}

pub fn prune_v3(geo_graph: &mut GeometricGraph, t: f64) {
    let edge_weights = geo_graph.get_edge_lengths();
    let num_nodes = geo_graph.graph.data.len();
    let mut edges = Vec::new();

    for u in 0..num_nodes {
        for &v in &geo_graph.graph.data[u] {
            if u < v {
                // let length = geo_graph.positions[u].distance(&geo_graph.positions[v]);
                let length = Euclidean::distance(&geo_graph.positions[u], &geo_graph.positions[v]);
                edges.push((u, v, length));
            }
        }
    }

    edges.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(Ordering::Equal));

    let mut spanner_graph = GeometricGraph {
        graph: Graph {
            data: vec![HashSet::new(); num_nodes],
        },
        positions: geo_graph.positions.clone(),
    };
    let mut uf = UnionFind::new(num_nodes);

    for (u, v, length) in edges {
        let same_component = uf.find(u) == uf.find(v);
        let mut add_edge = false;

        if !same_component {
            add_edge = true;
        } else {
            // let dist_h = dijkstra(&spanner_graph, &geo_graph.positions, u, v, t * length);
            let connected_less_than =
                spanner_graph.dijkstra_less_than(u, v, t * length, &edge_weights);
            if !connected_less_than {
                add_edge = true;
            }
            // if dist_h > t * length {
            //     add_edge = true;
            // }
        }

        if add_edge {
            spanner_graph.graph.data[u].insert(v);
            spanner_graph.graph.data[v].insert(u);
            if !same_component {
                uf.union(u, v);
            }
        }
    }

    geo_graph.graph = spanner_graph.graph;
}

pub fn build_voronoi_road_network(
    poly: Polygon,
    levels: usize,
    centers: Vec<Uniform<f64>>,
    fractions: Vec<f64>,
) -> GeometricGraph {
    assert_eq!(centers.len(), levels);
    assert_eq!(fractions.len(), levels);

    let mut edges: Vec<((usize, usize), (usize, usize))> = Vec::new();

    let mut s = vec![poly];
    for i in 0..levels {
        let m = ((fractions[i] * (s.len() - 1) as f64) as usize);
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
    let mut g = g.largest_connected_component();
    g.graph.info();
    println!("Graph build");
    // prune_graph(&mut g, 3.0);
    // prune_graph(&mut g, 4.0);
    prune_graph_spanner(&mut g, 4.0);
    g.graph.info();
    println!("Graph pruned");
    g
}

pub fn voronoi_example() {
    let levels = 4;
    let centers = vec![
        Uniform::new(300.0, 300.1),
        Uniform::new(2.0, 60.0),
        Uniform::new(2.0, 90.0),
        Uniform::new(2.0, 60.0),
    ];
    //let densities = vec![0.2, 0.5, 0.9, 0.0];
    //let radii = vec![
    //    Exp::new(0.01).unwrap(),          // 100^0.2 = 2.5
    //    Exp::new(0.1).unwrap(),           // 10^0.5 = 3.2
    //    Exp::new(2.0).unwrap(),           // 2^0.9 = 1.3
    //    Exp::new(f64::INFINITY).unwrap(), // 0
    //];
    let fractions = vec![1.0, 0.95, 0.9, 0.7];
    let poly = polygon![
        (x: 0.0, y: 0.0),
        (x: 0.0, y: 100000.0),
        (x: 100000.0, y: 100000.0),
        (x: 100000.0, y: 0.0),
        (x: 0.0, y: 0.0),
    ];

    build_voronoi_road_network(poly, levels, centers, fractions);
}

pub fn voronoi_example_small() -> GeometricGraph {
    let levels = 4;
    let centers = vec![
        Uniform::new(10.0, 10.1),
        Uniform::new(2.0, 40.0),
        Uniform::new(2.0, 70.0),
        Uniform::new(2.0, 40.0),
    ];
    let fractions = vec![1.0, 0.95, 0.9, 0.7];
    let poly = polygon![
        (x: 0.0, y: 0.0),
        (x: 0.0, y: 1000.0),
        (x: 1000.0, y: 1000.0),
        (x: 1000.0, y: 0.0),
        (x: 0.0, y: 0.0),
    ];

    build_voronoi_road_network(poly, levels, centers, fractions)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_voronoi() {
        let g = voronoi_example_small();
        g.visualize("voronoi");
    }
}
