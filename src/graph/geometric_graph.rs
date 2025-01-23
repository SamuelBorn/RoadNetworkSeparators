use std::io;

use crate::library;
use crate::Graph;

pub struct GeometricGraph {
    pub graph: Graph,
    pub positions: Vec<Position>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(pub f32, pub f32);

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

impl GeometricGraph {
    pub fn new(graph: Graph, positions: Vec<Position>) -> GeometricGraph {
        assert_eq!(graph.get_num_nodes(), positions.len());
        GeometricGraph { graph, positions }
    }

    pub fn from_file(
        first_out_file: &str,
        head_file: &str,
        latitude_file: &str,
        longitude_file: &str,
    ) -> io::Result<Self> {
        let g = Graph::from_file(first_out_file, head_file)?;
        let latitudes = library::read_binary_vec::<f32>(latitude_file)?;
        let longitudes = library::read_binary_vec::<f32>(longitude_file)?;

        assert_eq!(g.get_num_nodes(), latitudes.len());
        assert_eq!(g.get_num_nodes(), longitudes.len());

        let positions = latitudes
            .into_iter()
            .zip(longitudes)
            .map(|(lat, lon)| Position(lat, lon))
            .collect();

        Ok(GeometricGraph::new(g, positions))
    }
}
