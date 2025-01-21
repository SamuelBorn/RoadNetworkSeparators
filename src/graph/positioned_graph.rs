use std::io;

use crate::library;
use crate::Graph;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(f32, f32);

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

pub struct PositionedGraph {
    pub graph: Graph,
    pub positions: Vec<Position>,
}

impl PositionedGraph {
    pub fn new(graph: Graph, positions: Vec<Position>) -> PositionedGraph {
        assert_eq!(graph.get_num_nodes(), positions.len());
        PositionedGraph { graph, positions }
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

        Ok(PositionedGraph::new(g, positions))
    }
}
