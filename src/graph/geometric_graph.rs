use std::io;
use std::path::Path;

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

    pub fn from_file(dir: &Path) -> io::Result<Self> {
        let g = Graph::from_file(dir)?;

        let latitudes = library::read_binary_vec::<f32>(&dir.join("latitude"))?;
        let longitudes = library::read_binary_vec::<f32>(&dir.join("longitude"))?;

        assert_eq!(g.get_num_nodes(), latitudes.len());
        assert_eq!(g.get_num_nodes(), longitudes.len());

        let positions = latitudes
            .into_iter()
            .zip(longitudes)
            .map(|(lat, lon)| Position(lat, lon))
            .collect();

        Ok(GeometricGraph::new(g, positions))
    }

    pub fn to_file(self, dir: &Path) -> io::Result<()> {
        self.graph.to_file(dir)?;
        library::write_binary_vec(
            &self.positions.iter().map(|p| p.0).collect::<Vec<f32>>(),
            &dir.join("latitude"),
        )?;
        library::write_binary_vec(
            &self.positions.iter().map(|p| p.1).collect::<Vec<f32>>(),
            &dir.join("longitude"),
        )
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
                Position(0.0, 0.0),
                Position(0.0, 1.0),
                Position(1.0, 1.0),
                Position(1.0, 0.0),
            ],
        );

        assert_eq!(g.graph.get_num_nodes(), 4);
        assert_eq!(g.positions.len(), 4);

        let file = tempfile::NamedTempFile::new().unwrap();
    }
}
