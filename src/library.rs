use std::fs::{self, File};
use std::io::{self, Read};

pub fn read_binary_vec<T: Sized>(file_path: &str) -> io::Result<Vec<T>> {
    let mut file = File::open(file_path)?;
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

pub fn read_edge_list(file_path: &str) -> io::Result<Vec<(usize, usize)>> {
    Ok(std::fs::read_to_string(file_path)?
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut nums = line.split_whitespace();
            Some((nums.next()?.parse().ok()?, nums.next()?.parse().ok()?))
        })
        .collect())
}
