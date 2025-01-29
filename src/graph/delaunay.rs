use ordered_float::OrderedFloat;
use spade::{DelaunayTriangulation, InsertionError, Point2, Triangulation};
use std::collections::{HashMap, HashSet};

use super::geometric_graph::Position;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Triangle {
    vertices: [usize; 3],
}

impl Triangle {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Triangle {
            vertices: [a, b, c],
        }
    }

    fn contains_vertex(&self, vertex: usize) -> bool {
        self.vertices.contains(&vertex)
    }

    fn get_edges(&self) -> [(usize, usize); 3] {
        [
            (self.vertices[0], self.vertices[1]),
            (self.vertices[1], self.vertices[2]),
            (self.vertices[2], self.vertices[0]),
        ]
    }
}

fn compute_circumcenter(points: &[Position], triangle: &Triangle) -> (Position, f32) {
    let [a, b, c] = [
        points[triangle.vertices[0]],
        points[triangle.vertices[1]],
        points[triangle.vertices[2]],
    ];

    let d = 2.0
        * (a.latitude() * (b.longitude() - c.longitude())
            + b.latitude() * (c.longitude() - a.longitude())
            + c.latitude() * (a.longitude() - b.longitude()));

    let ux = ((a.latitude().powi(2) + a.longitude().powi(2)) * (b.longitude() - c.longitude())
        + (b.latitude().powi(2) + b.longitude().powi(2)) * (c.longitude() - a.longitude())
        + (c.latitude().powi(2) + c.longitude().powi(2)) * (a.longitude() - b.longitude()))
        / d;

    let uy = ((a.latitude().powi(2) + a.longitude().powi(2)) * (c.latitude() - b.latitude())
        + (b.latitude().powi(2) + b.longitude().powi(2)) * (a.latitude() - c.latitude())
        + (c.latitude().powi(2) + c.longitude().powi(2)) * (b.latitude() - a.latitude()))
        / d;

    let center = Position::new(ux, uy);
    let radius = ((center.latitude() - a.latitude()).powi(2)
        + (center.longitude() - a.longitude()).powi(2))
    .sqrt();

    (center, radius)
}

fn point_in_circumcircle(point: &Position, points: &[Position], triangle: &Triangle) -> bool {
    let (center, radius) = compute_circumcenter(points, triangle);
    let distance = ((point.latitude() - center.latitude()).powi(2)
        + (point.longitude() - center.longitude()).powi(2))
    .sqrt();
    distance < radius
}

pub fn delaunay_triangulation(points: Vec<Position>) -> Vec<(usize, usize)> {
    if points.len() < 3 {
        return vec![];
    }

    // Find the bounding box
    let mut min_lat = f32::INFINITY;
    let mut min_lon = f32::INFINITY;
    let mut max_lat = f32::NEG_INFINITY;
    let mut max_lon = f32::NEG_INFINITY;

    for point in &points {
        min_lat = min_lat.min(point.latitude());
        min_lon = min_lon.min(point.longitude());
        max_lat = max_lat.max(point.latitude());
        max_lon = max_lon.max(point.longitude());
    }

    // Add super-triangle vertices
    let margin = 10.0;
    let d_lat = max_lat - min_lat;
    let d_lon = max_lon - min_lon;
    let delta_max = d_lat.max(d_lon);
    let mid_lat = (min_lat + max_lat) / 2.0;
    let mid_lon = (min_lon + max_lon) / 2.0;

    let mut all_points = points.clone();
    let p1 = Position::new(mid_lat - 20.0 * delta_max, mid_lon - delta_max);
    let p2 = Position::new(mid_lat, mid_lon + 20.0 * delta_max);
    let p3 = Position::new(mid_lat + 20.0 * delta_max, mid_lon - delta_max);

    all_points.push(p1);
    all_points.push(p2);
    all_points.push(p3);

    let super_triangle = Triangle::new(points.len(), points.len() + 1, points.len() + 2);

    let mut triangulation = vec![super_triangle];

    // Add points one at a time to the triangulation
    for i in 0..points.len() {
        let mut bad_triangles = HashSet::new();

        // Find all triangles that are no longer valid due to the insertion
        for triangle in &triangulation {
            if point_in_circumcircle(&points[i], &all_points, triangle) {
                bad_triangles.insert(triangle.clone());
            }
        }

        // Find the boundary of the polygonal hole
        let mut boundary = HashSet::new();
        for triangle in &bad_triangles {
            for edge in triangle.get_edges() {
                if !boundary.remove(&(edge.1, edge.0)) {
                    boundary.insert(edge);
                }
            }
        }

        // Remove bad triangles from the triangulation
        triangulation.retain(|triangle| !bad_triangles.contains(triangle));

        // Re-triangulate the polygonal hole
        for edge in boundary {
            triangulation.push(Triangle::new(edge.0, edge.1, i));
        }
    }

    // Remove triangles that contain vertices of the super-triangle
    triangulation.retain(|triangle| {
        !triangle.contains_vertex(points.len())
            && !triangle.contains_vertex(points.len() + 1)
            && !triangle.contains_vertex(points.len() + 2)
    });

    // Convert triangles to unique edges
    let mut edges = HashSet::new();
    for triangle in triangulation {
        for edge in triangle.get_edges() {
            let (a, b) = edge;
            if a < b {
                edges.insert((a, b));
            } else {
                edges.insert((b, a));
            }
        }
    }

    edges.into_iter().collect()
}

fn delaunay_crate() -> Result<(), InsertionError> {
    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();

    triangulation.insert(Point2::new(0.0, 1.0))?;
    triangulation.insert(Point2::new(1.0, 1.0))?;
    triangulation.insert(Point2::new(0.5, -1.0))?;


    assert_eq!(triangulation.num_vertices(), 3);
    assert_eq!(triangulation.num_inner_faces(), 1);
    assert_eq!(triangulation.num_undirected_edges(), 3);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delaunay_triangulation() {
        let points = vec![
            Position::new(0.0, 0.0),
            Position::new(0.0, 1.0),
            Position::new(1.0, 1.0),
            Position::new(1.0, 0.0),
        ];

        let edges = delaunay_triangulation(points);
        println!("{:?}", edges);
        //assert_eq!(edges.len(), 6);
    }

    #[test]
    fn test_large_delauny() {
        let (lat_min, lat_max, lon_min, lon_max) = (48.3, 49.2, 8.0, 9.0);
        let points = (0..10000)
            .map(|_| Position::random(lat_min, lat_max, lon_min, lon_max))
            .collect::<Vec<_>>();

        let edges = delaunay_triangulation(points);

        println!("{:?}", edges.len());
    }
}
