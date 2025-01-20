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
}
