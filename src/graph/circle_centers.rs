use geo::Point;

fn generate_circle_center_graph(
    points_per_level: &[usize],
    city_percentage: &[f64],
    disk_sizes: &[f64],
    disk_center: Point,
) {
    assert_eq!(points_per_level.len(), disk_sizes.len());
    assert_eq!(points_per_level.len(), city_percentage.len());

}
