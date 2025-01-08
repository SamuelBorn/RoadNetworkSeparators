use crate::Graph;

use bimap::BiMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub i32, pub i32);

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

pub struct PositionedGraph {
    pub graph: Graph,
    pub positions: BiMap<usize, Position>,
}

impl PositionedGraph {
    pub fn new(graph: Graph, positions: BiMap<usize, Position>) -> PositionedGraph {
        assert_eq!(graph.get_num_nodes(), positions.len());
        PositionedGraph { graph, positions }
    }
}
