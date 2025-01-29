use spade::{DelaunayTriangulation, InsertionError, Point2, Triangulation};

use super::{
    geometric_graph::{GeometricGraph, Position},
    Graph,
};

fn random_triangulation(n: usize) -> GeometricGraph {
    let (lat_min, lat_max, lon_min, lon_max) = (48.3, 49.2, 8.0, 9.0);
    let positions = (0..n)
        .map(|_| Position::random(lat_min, lat_max, lon_min, lon_max))
        .collect::<Vec<_>>();

    let triangulation = DelaunayTriangulation::<Point2<f32>>::bulk_load_stable(
        positions
            .iter()
            .map(|p| Point2::new(p.latitude(), p.longitude()))
            .collect(),
    )
    .unwrap();

    let g = Graph::from_edge_list(
        triangulation
            .undirected_edges()
            .map(|edge| {
                let a = edge.as_directed().from().index();
                let b = edge.as_directed().to().index();
                (a, b)
            })
            .collect(),
    );

    GeometricGraph::new(g, positions)
}

#[cfg(test)]
mod tests {
    use super::*;


}
