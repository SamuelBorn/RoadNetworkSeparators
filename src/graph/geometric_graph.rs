use std::fs;
use std::io;
use std::path::Path;

use crate::library;
use crate::Graph;
use ordered_float::OrderedFloat;

pub struct GeometricGraph {
    pub graph: Graph,
    positions: Vec<Position>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    latitude: OrderedFloat<f32>,
    longitude: OrderedFloat<f32>,
}

impl Position {
    pub fn new(lat: f32, lon: f32) -> Position {
        Position {
            latitude: OrderedFloat(lat),
            longitude: OrderedFloat(lon),
        }
    }

    pub fn new_ordered(lat: OrderedFloat<f32>, lon: OrderedFloat<f32>) -> Position {
        Position {
            latitude: lat,
            longitude: lon,
        }
    }

    pub fn random(min: f32, max: f32) -> Position {
        Position::new(
            rand::random::<f32>() * (max - min) + min,
            rand::random::<f32>() * (max - min) + min,
        )
    }

    pub fn latitude(&self) -> f32 {
        self.latitude.into_inner()
    }

    pub fn longitude(&self) -> f32 {
        self.longitude.into_inner()
    }

    pub fn latitude_mut(&mut self) -> &mut OrderedFloat<f32> {
        &mut self.latitude
    }

    pub fn longitude_mut(&mut self) -> &mut OrderedFloat<f32> {
        &mut self.longitude
    }

    // checks if self is on a line defined by a1 and a2
    // does not make sure that self is between a1 and a2
    pub fn on_line(&self, a1: Position, a2: Position) -> bool {
        let cross_product = (self.latitude.0 - a1.latitude.0) * (a2.longitude.0 - a1.longitude.0)
            - (self.longitude.0 - a1.longitude.0) * (a2.latitude.0 - a1.latitude.0);

        return cross_product.abs() < 0.0000001;
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position::new(
            self.latitude() + other.latitude(),
            self.longitude() + other.longitude(),
        )
    }
}

impl GeometricGraph {
    pub fn new(graph: Graph, positions: Vec<Position>) -> GeometricGraph {
        assert_eq!(graph.get_num_nodes(), positions.len());
        GeometricGraph { graph, positions }
    }

    pub fn from_file(dir: &Path) -> io::Result<Self> {
        let g = Graph::from_file(dir)?;

        let latitudes = library::read_binary_vec::<f32>(&dir.join("latitude"))?;
        let longitudes = library::read_binary_vec::<f32>(&dir.join("longitude"))?;

        assert_eq!(g.get_num_nodes(), latitudes.len());
        assert_eq!(g.get_num_nodes(), longitudes.len());

        let positions = latitudes
            .into_iter()
            .zip(longitudes)
            .map(|(lat, lon)| Position::new(lat, lon))
            .collect();

        Ok(GeometricGraph::new(g, positions))
    }

    pub fn to_file(&self, dir: &Path) -> io::Result<()> {
        self.graph.to_file(dir)?;
        library::write_binary_vec(
            &self
                .positions
                .iter()
                .map(|p| p.latitude())
                .collect::<Vec<f32>>(),
            &dir.join("latitude"),
        )?;
        library::write_binary_vec(
            &self
                .positions
                .iter()
                .map(|p| p.longitude())
                .collect::<Vec<f32>>(),
            &dir.join("longitude"),
        )
    }

    pub fn get_position(&self, node: usize) -> Position {
        self.positions[node]
    }

    pub fn add_position(&mut self, pos: Position) -> usize {
        self.positions.push(pos);
        self.graph.add_node()
    }

    pub fn distance(&self, a: usize, b: usize) -> f32 {
        let pos_a = self.positions[a];
        let pos_b = self.positions[b];

        let lat_diff = pos_a.latitude() - pos_b.latitude();
        let lon_diff = pos_a.longitude() - pos_b.longitude();

        (lat_diff.powi(2) + lon_diff.powi(2)).sqrt()
    }

    pub fn save_distance_overview(&self, file: &Path) {
        let mut res = String::new();
        for (u,v) in self.graph.get_edges() {
            let distance = self.distance(u, v);
            // append distance to res
            res.push_str(&format!("{}\n", distance));
        }
        fs::write(file, res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometric_graph() {
        let g = GeometricGraph::new(
            Graph::from_edge_list(vec![(0, 1), (1, 2), (2, 3), (3, 0)]),
            vec![
                Position::new(0.0, 0.0),
                Position::new(0.0, 1.0),
                Position::new(1.0, 1.0),
                Position::new(1.0, 0.0),
            ],
        );

        assert_eq!(g.graph.get_num_nodes(), 4);
        assert_eq!(g.positions.len(), 4);

        let file = tempfile::NamedTempFile::new().unwrap();
    }

    #[test]
    fn test_on_line() {
        let p1 = Position::new(0.0, 0.0);
        let p2 = Position::new(2.0, 2.0);
        let p3 = Position::new(1.0, 1.0); // On the line segment
        let p4 = Position::new(3.0, 3.0); // Outside the segment
        let p5 = Position::new(-3.0, 4.0); // Outside the segment

        assert!(p3.on_line(p1, p2));
        assert!(p4.on_line(p1, p2));
        assert!(!p5.on_line(p1, p2));
    }
}
