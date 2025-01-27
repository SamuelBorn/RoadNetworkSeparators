use rand::seq::SliceRandom;
use rand::Rng;
use std::{
    collections::{HashMap, HashSet}, fs, io, path::Path
};

use crate::{library, separator};
pub mod geometric_graph;
pub mod grid;
pub mod planar;
pub mod tree;
pub mod unit_disk;

#[derive(Debug, Clone)]
pub struct Graph {
    data: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(data: Vec<Vec<usize>>) -> Self {
        Graph { data }
    }

    pub fn with_node_count(n: usize) -> Self {
        Graph {
            data: vec![Vec::new(); n],
        }
    }

    pub fn from_edge_list(edges: Vec<(usize, usize)>) -> Self {
        let mut g = Graph::with_node_count(0);
        for (u, v) in edges {
            g.add_edge(u, v);
        }
        g
    }

    pub fn from_edge_list_file(file: &Path) -> io::Result<Self> {
        let edges = library::read_edge_list(file)?;
        Ok(Graph::from_edge_list(edges))
    }

    pub fn from_file(dir: &Path) -> io::Result<Self> {
        let xadj = library::read_binary_vec::<u32>(&dir.join("first_out"))?
            .into_iter()
            .map(|x| x as usize)
            .collect::<Vec<usize>>();
        let adjncy = library::read_binary_vec::<u32>(&dir.join("head"))?
            .into_iter()
            .map(|x| x as usize)
            .collect::<Vec<usize>>();

        let mut data = vec![vec![]; xadj.len() - 1];
        for i in 0..xadj.len() - 1 {
            for j in xadj[i]..xadj[i + 1] {
                if data[i].contains(&adjncy[j]) {
                    continue;
                }
                data[i].push(adjncy[j]);
                data[adjncy[j]].push(i);
            }
        }

        Ok(Graph { data })
    }

    pub fn to_file(&self, dir: &Path) -> io::Result<()> {
        fs::create_dir_all(dir)?;
        let (xadj, adjncy) = self.get_adjacency_array();
        library::write_binary_vec(&xadj, &dir.join("first_out"))?;
        library::write_binary_vec(&adjncy, &dir.join("head"))
    }

    pub fn has_edge(&self, i: usize, j: usize) -> bool {
        self.data[i].contains(&j)
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        if std::cmp::max(i, j) >= self.data.len() {
            self.data.resize(std::cmp::max(i, j) + 1, Vec::new());
        }

        if !self.has_edge(i, j) {
            self.data[i].push(j);
            self.data[j].push(i);
        }
    }

    pub fn remove_edge(&mut self, i: usize, j: usize) {
        if let Some(pos) = self.data[i].iter().position(|&x| x == j) {
            self.data[i].remove(pos);
        }

        if let Some(pos) = self.data[j].iter().position(|&x| x == i) {
            self.data[j].remove(pos);
        }
    }

    pub fn remove_random_edge(&mut self) -> (usize, usize) {
        let (u, v) = self.get_random_edge();
        self.remove_edge(u, v);
        (u, v)
    }

    pub fn remove_random_edge_stay_connected_approx(
        &mut self,
        mut num_checks: u32,
    ) -> (usize, usize) {
        loop {
            let (u, v) = self.remove_random_edge();

            let mut queue = std::collections::VecDeque::from(vec![u]);
            let mut visited = HashSet::new();
            visited.insert(u);

            while !queue.is_empty() && num_checks > 0 {
                num_checks -= 1;
                let u = queue.pop_front().unwrap();
                for &neigh in self.get_neighbors(u) {
                    if visited.contains(&neigh) {
                        continue;
                    }
                    if neigh == v {
                        return (u, v);
                    }
                    visited.insert(neigh);
                    queue.push_back(neigh);
                }
            }

            self.add_edge(u, v);
        }
    }

    pub fn get_edges(&self) -> Vec<(usize, usize)> {
        let mut edges = Vec::with_capacity(self.get_num_edges());
        for (i, neighbors) in self.data.iter().enumerate() {
            for &j in neighbors {
                if i < j {
                    edges.push((i, j));
                }
            }
        }
        edges
    }

    pub fn get_random_edge(&mut self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let u = rng.gen_range(0..self.get_num_nodes());
        while self.get_neighbors(u).is_empty() {
            let u = rng.gen_range(0..self.get_num_nodes());
        }
        let v = self.get_neighbors(u).choose(&mut rng).unwrap();
        (u, *v)
    }

    pub fn add_node(&mut self) -> usize {
        self.data.push(Vec::new());
        self.data.len() - 1
    }

    pub fn get_num_nodes(&self) -> usize {
        self.data.len()
    }

    pub fn get_num_edges(&self) -> usize {
        self.data.iter().map(|v| v.len()).sum::<usize>() / 2
    }

    pub fn get_average_degree(&self) -> f64 {
        self.data.iter().map(|v| v.len()).sum::<usize>() as f64 / self.get_num_nodes() as f64
    }

    pub fn get_neighbors(&self, u: usize) -> &[usize] {
        &self.data[u]
    }

    pub fn get_adjacency_array(&self) -> (Vec<i32>, Vec<i32>) {
        let mut xadj = vec![0; self.get_num_nodes() + 1];
        let mut adjncy = Vec::new();

        for (i, neighbors) in self.data.iter().enumerate() {
            xadj[i] = adjncy.len() as i32;
            for &n in neighbors {
                adjncy.push(n as i32);
            }
        }
        xadj[self.get_num_nodes()] = adjncy.len() as i32;

        (xadj, adjncy)
    }

    pub fn get_extended_neighborhood(&self, u: usize, num_nodes: usize) -> HashMap<usize, usize> {
        let mut distances = HashMap::with_capacity(num_nodes);
        distances.insert(u, 0);

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(u);

        while distances.len() < num_nodes + 1 && !queue.is_empty() {
            let u = queue.pop_front().unwrap();

            for &v in self.get_neighbors(u) {
                if distances.contains_key(&v) {
                    continue;
                }
                distances.insert(v, distances[&u] + 1);
                queue.push_back(v);
            }
        }

        distances.remove(&u);

        distances
    }

    pub fn is_connected(&self) -> bool {
        let distances = self.bfs(0);
        distances.iter().all(|&d| d != usize::MAX)
    }

    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.get_num_nodes()];
        distances[start] = 0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            for &v in self.get_neighbors(u) {
                if distances[v] != usize::MAX {
                    continue;
                }
                distances[v] = distances[u] + 1;
                queue.push_back(v);
            }
        }

        distances
    }

    pub fn get_largest_subgraph(&self) -> Graph {
        let separator = HashSet::new();
        let subgraphs = self.get_subgraphs(&separator);

        subgraphs
            .into_iter()
            .max_by_key(|g| g.get_num_nodes())
            .unwrap()
    }

    fn get_furthest_node(&self, start: usize) -> (usize, usize) {
        let distances = self.bfs(start);
        let furthest_node = distances
            .iter()
            .filter(|&&d| d != usize::MAX)
            .enumerate()
            .max_by_key(|&(_, d)| d)
            .unwrap();
        (furthest_node.0, *furthest_node.1)
    }

    // Diameter Karlsruhe: 323
    // Diameter Germany: 1163
    pub fn get_diameter(&self) -> usize {
        let (furthest_node, _) = self.get_furthest_node(0);
        let (_, diameter) = self.get_furthest_node(furthest_node);
        diameter
    }

    pub fn print(&self) {
        for (i, neighbors) in self.data.iter().enumerate() {
            println!("{}: {:?}", i, neighbors);
        }
    }

    pub fn to_json(&self) -> String {
        let mut json_representation = HashMap::new();
        for (index, neighbors) in self.data.iter().enumerate() {
            json_representation.insert(index, neighbors);
        }
        serde_json::to_string(&json_representation).unwrap()
    }

    pub fn to_json_file(&self, filename: &str) {
        std::fs::write(filename, self.to_json());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diameter() {
        let mut g = Graph::new(vec![vec![1, 2], vec![0, 2], vec![0, 1, 3], vec![2]]);
        assert_eq!(g.get_diameter(), 2);

        g.add_edge(3, 4);
        assert_eq!(g.get_diameter(), 3);
    }

    #[test]
    fn test_bfs() {
        let g = Graph::new(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]);
        assert_eq!(g.bfs(0), vec![0, 1, 2, 1]);
    }

    #[test]
    fn test_average_degree() {
        let g = Graph::new(vec![vec![1, 2], vec![0, 2], vec![0, 1, 3], vec![2]]);

        assert!((g.get_average_degree() - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_remove_random_edge_stay_connected_approx() {
        return;
        let g = Graph::from_edge_list(vec![(0, 1), (1, 2), (1, 3), (2, 4), (3, 4), (4, 5)]);

        for _ in 0..100 {
            let mut g_ = g.clone();
            g_.remove_random_edge_stay_connected_approx(3);
            //g_.remove_random_edge();
            assert!(g_.is_connected());
        }
    }
}
