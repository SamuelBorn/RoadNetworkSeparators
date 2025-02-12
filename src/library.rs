use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;

use crate::graph::geometric_graph::Position;

pub fn read_bin_u32_vec_to_usize(file: &Path) -> Vec<usize> {
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

pub fn read_position_list(file: &Path) -> io::Result<Vec<Position>> {
    Ok(std::fs::read_to_string(file)?
        .lines()
        .filter_map(|line| {
            let mut nums = line.split(',');
            Some(Position::new(
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
