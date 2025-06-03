use rand_distr::uniform::SampleUniform;
use rand_distr::Normal;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;

use geo::{Point, Rect};
use rand::Rng;

pub fn read_to_usize_vec(file: &Path) -> Vec<usize> {
    let mut file = File::open(file).unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();
    assert!(buffer.len() % std::mem::size_of::<u32>() == 0);
    let num_elements = buffer.len() / std::mem::size_of::<u32>();
    let mut vec = Vec::with_capacity(num_elements);

    for chunk in buffer.chunks(std::mem::size_of::<u32>()) {
        let elem = unsafe { std::ptr::read(chunk.as_ptr() as *const u32) } as usize;
        vec.push(elem as usize);
    }

    vec
}

pub fn read_binary_vec<T: Sized>(file: &Path) -> io::Result<Vec<T>> {
    let mut file = File::open(file)?;
    let mut buffer = Vec::new();

    // Read the entire file into the buffer
    file.read_to_end(&mut buffer)?;

    // Convert the byte buffer into a vector of type T
    let element_size = std::mem::size_of::<T>();
    if buffer.len() % element_size != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File size is not a multiple of element size",
        ));
    }

    let num_elements = buffer.len() / element_size;
    let mut vec: Vec<T> = Vec::with_capacity(num_elements);

    for chunk in buffer.chunks(element_size) {
        // Safely interpret the bytes as a reference to type T
        let elem = unsafe { std::ptr::read(chunk.as_ptr() as *const T) };
        vec.push(elem);
    }

    Ok(vec)
}

pub fn write_binary_vec<T: Sized>(input: &[T], file: &Path) -> io::Result<()> {
    let element_size = std::mem::size_of::<T>();
    let buffer_size = input.len() * element_size;
    let mut buffer = Vec::with_capacity(buffer_size);

    for elem in input {
        // Convert the reference to type T into a byte slice
        let elem_bytes =
            unsafe { std::slice::from_raw_parts((elem as *const T) as *const u8, element_size) };
        buffer.extend_from_slice(elem_bytes);
    }

    fs::write(file, buffer)
}

pub fn read_text_vec<T: FromStr>(file: &Path) -> io::Result<Vec<T>> {
    fs::read_to_string(file)?
        .lines()
        .map(|line| {
            line.parse()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid data"))
        })
        .collect()
}

pub fn write_text_vec<T: std::fmt::Display>(input: &[T], file: &Path) -> io::Result<()> {
    fs::write(
        file,
        input
            .iter()
            .map(|elem| elem.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
    )
}

pub fn write_point_vec(file: &Path, points: &[Point<f64>]) -> io::Result<()> {
    fs::write(
        file,
        points
            .iter()
            .map(|point| format!("{} {}\n", point.x(), point.y()))
            .collect::<String>(),
    )
}

pub fn read_edge_list(file: &Path) -> io::Result<Vec<(usize, usize)>> {
    Ok(std::fs::read_to_string(file)?
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut nums = line.split_whitespace();
            Some((nums.next()?.parse().ok()?, nums.next()?.parse().ok()?))
        })
        .collect())
}

pub fn read_position_list(file: &Path) -> io::Result<Vec<Point>> {
    Ok(std::fs::read_to_string(file)?
        .lines()
        .filter_map(|line| {
            let mut nums = line.split(',');
            Some(Point::new(
                nums.next()?.parse().ok()?,
                nums.next()?.parse().ok()?,
            ))
        })
        .collect())
}

pub fn clear_file(file: &Path) {
    fs::write(file, "");
}

pub fn append_to_file(file: &Path, content: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file)
        .unwrap();

    file.write_all(content.as_bytes());
}

pub fn optional_append_to_file(file: Option<&Path>, content: &str) {
    if let Some(file) = file {
        append_to_file(file, content);
    }
}

pub fn random_point_in_circle<R>(center: Point<f64>, radius: R) -> Point<f64>
where
    R: Into<f64>,
{
    let mut rng = rand::thread_rng();
    let theta = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
    let u = rng.gen::<f64>();
    let r = radius.into() * u.sqrt();
    let dx = r * theta.cos();
    let dy = r * theta.sin();
    Point::new(center.x() + dx, center.y() + dy)
}

pub fn random_points_in_circle<R>(
    center: Point<f64>,
    radius: R,
    num_points: usize,
) -> Vec<Point<f64>>
where
    R: Into<f64> + Copy,
{
    (0..num_points)
        .map(|_| random_point_in_circle(center, radius))
        .collect()
}

pub fn random_point_in_rect(rect: Rect<f64>) -> Point<f64> {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(rect.min().x..rect.max().x);
    let y = rng.gen_range(rect.min().y..rect.max().y);
    Point::new(x, y)
}

pub fn random_points_in_rect(rect: Rect<f64>, num_points: usize) -> Vec<Point<f64>> {
    (0..num_points)
        .map(|_| random_point_in_rect(rect))
        .collect()
}

pub fn random_point_normal_dist<R>(center: Point<f64>, std_dev: R) -> Point<f64>
where
    R: Into<f64>,
{
    let mut rng = rand::thread_rng();
    let std_dev_f64 = std_dev.into();
    let normal_x = Normal::new(center.x(), std_dev_f64).unwrap();
    let normal_y = Normal::new(center.y(), std_dev_f64).unwrap();
    let x = rng.sample(normal_x);
    let y = rng.sample(normal_y);
    Point::new(x, y)
}

pub fn random_points_normal_dist<R>(
    center: Point<f64>,
    std_dev: R,
    num_points: usize,
) -> Vec<Point<f64>>
where
    R: Into<f64> + Copy,
{
    (0..num_points)
        .map(|_| random_point_normal_dist(center, std_dev))
        .collect()
}

pub fn random_point_in_rect_tuple<T>(bottom_left: (T, T), top_right: (T, T)) -> (T, T)
where
    T: PartialOrd + SampleUniform,
{
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(bottom_left.0..top_right.0);
    let y = rng.gen_range(bottom_left.1..top_right.1);
    (x, y)
}

pub fn random_points_in_rect_tuple<T>(
    bottom_left: (T, T),
    top_right: (T, T),
    num_points: usize,
) -> Vec<(T, T)>
where
    T: PartialOrd + SampleUniform + Copy,
{
    (0..num_points)
        .map(|_| random_point_in_rect_tuple(bottom_left, top_right))
        .collect()
}

pub fn histogram<I>(data: I, bin_edges: &[f64]) -> Vec<usize>
where
    I: IntoIterator<Item = f64>,
{
    let mut counts = vec![0; bin_edges.len() - 1];

    for value in data {
        for (i, edge) in bin_edges.windows(2).enumerate() {
            if value > edge[0] && value <= edge[1] {
                counts[i] += 1;
                break;
            }
        }
    }

    counts
}

pub fn write_histogram_to_file(name: &str, bin_edges: &[f64], counts: &[usize]) -> io::Result<()> {
    assert_eq!(
        bin_edges.len() - 1,
        counts.len(),
        "Bin edges and counts must have compatible lengths"
    );

    let bin_edges = bin_edges
        .iter()
        .map(|edge| format!("{:.4} ", edge))
        .collect::<String>();

    let counts = counts
        .iter()
        .map(|count| format!("{:.4} ", count))
        .collect::<String>();

    fs::write(
        format!("./output/histogram/{name}"),
        format!("{bin_edges}\n{counts}"),
    )
}

pub fn get_bin_edges(max: f64, num_bins: usize) -> Vec<f64> {
    let bin_size = (max + 1e-6) / num_bins as f64;
    (0..=num_bins).map(|i| i as f64 * bin_size).collect()
}

pub fn add_vecs<T: std::ops::Add<Output = T> + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    assert_eq!(a.len(), b.len(), "Vectors must be of the same length");
    a.iter().zip(b.iter()).map(|(&x, &y)| x + y).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_read_write_binary_vec() -> Result<(), Box<dyn std::error::Error>> {
        //let input = vec![1, 2, 3, 4, 5];
        //let mut tmpfile = tempfile::NamedTempFile::new()?;
        //
        //write_binary_vec(&input, tmpfile.path())?;
        //let output = read_binary_vec::<i32>()?;
        //
        //assert_eq!(input, output);
        //
        Ok(())
    }
}
