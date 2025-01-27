use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::io::Write;
use std::path::Path;
use std::{fs, ptr};

use chrono::format;

use crate::graph::Graph;
use crate::library::optional_append_to_file;
use crate::{graph, library, separator};

#[link(name = "kahip")]
extern "C" {
    fn node_separator(
        n: *const i32,
        vwgt: *const i32,
        xadj: *const i32,
        adjcwgt: *const i32,
        adjncy: *const i32,
        nparts: *const i32,
        imbalance: *const f64,
        suppress_output: bool,
        seed: i32,
        mode: i32,
        num_separator_vertices: *mut i32,
        separator: *mut *mut i32,
    );
}

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Fast = 0,
    Eco = 1,
    Strong = 2,
    FastSocial = 3,
    EcoSocial = 4,
    StrongSocial = 5,
}

impl Graph {
    pub fn get_separator(
        &self,
        nparts: i32,
        imbalance: f64,
        seed: i32,
        mode: Mode,
    ) -> HashSet<usize> {
        let n = self.get_num_nodes() as i32;
        let (xadj, adjncy) = self.get_adjacency_array();
        let mut num_separator_vertices = 0;
        let mut sep = vec![0; self.get_num_nodes() as usize];
        let mut separator_raw = sep.as_mut_ptr();

        unsafe {
            node_separator(
                &n,
                ptr::null(),
                xadj.as_ptr(),
                ptr::null(),
                adjncy.as_ptr(),
                &nparts,
                &imbalance,
                true,
                seed,
                mode as i32,
                &mut num_separator_vertices,
                &mut separator_raw,
            );

            return std::slice::from_raw_parts(separator_raw, num_separator_vertices as usize)
                .iter()
                .map(|&x| x as usize)
                .collect();
        }
    }

    pub fn get_separator_wrapper(&self, mode: Mode) -> HashSet<usize> {
        self.get_separator(2, 0.33, rand::random(), mode)
    }

    pub fn get_separator_size(&self, mode: Mode) -> usize {
        self.get_separator_wrapper(mode).len()
    }

    pub fn get_subgraphs(&self, separator: &HashSet<usize>) -> Vec<Graph> {
        let mut used = vec![false; self.get_num_nodes()];
        let mut subgraphs = Vec::new();

        for i in 0..self.get_num_nodes() {
            if used[i] || separator.contains(&i) {
                continue;
            }

            let mut subgraph: HashMap<usize, Vec<usize>> = HashMap::new();
            let mut q = vec![i];
            used[i] = true;

            while !q.is_empty() {
                let u = q.pop().unwrap();
                for &v in self.get_neighbors(u) {
                    if separator.contains(&v) {
                        continue;
                    }

                    if !used[v] {
                        q.push(v);
                        used[v] = true;
                    }
                    let x = subgraph.entry(u).or_insert(Vec::new());
                    if !x.contains(&v) {
                        x.push(v);
                    }
                    let y = subgraph.entry(v).or_insert(Vec::new());
                    if !y.contains(&u) {
                        y.push(u);
                    }
                }
            }
            subgraphs.push(subgraph);
        }

        subgraphs.iter().map(|x| get_graph(&x)).collect()
    }

    pub fn recurse_separator(&self, mode: Mode, file: Option<&Path>) {
        let separator = self.get_separator_wrapper(mode);
        let subgraphs = self.get_subgraphs(&separator);

        println!("{} {}", self.get_num_nodes(), separator.len());

        library::optional_append_to_file(
            file,
            &format!("{} {}\n", self.get_num_nodes(), separator.len()),
        );

        for i in 0..subgraphs.len() {
            if subgraphs[i].get_num_nodes() > 200 {
                subgraphs[i].recurse_separator(mode, file);
                break;
            }
        }
    }

    pub fn queue_separator(&self, mode: Mode, file: Option<&Path>) {
        let mut queue = VecDeque::from(vec![self.clone()]);
        if let Some(file) = file {
            fs::write(file, "");
        }

        let mut remaining = 300;
        while (!queue.is_empty() && remaining > 0) {
            remaining -= 1;
            let g = queue.pop_front().unwrap();
            let separator = g.get_separator_wrapper(mode);
            let mut subgraphs = g.get_subgraphs(&separator);

            println!("{} {}", g.get_num_nodes(), separator.len());
            library::optional_append_to_file(
                file,
                &format!("{} {}\n", g.get_num_nodes(), separator.len()),
            );

            for i in 0..subgraphs.len() {
                let subgraph = subgraphs.swap_remove(0);
                if subgraph.get_num_nodes() > 200 {
                    queue.push_back(subgraph);
                }
            }
        }
    }
}

fn get_graph(g_map: &HashMap<usize, Vec<usize>>) -> Graph {
    let mut mapping = HashMap::new();
    let mut data = vec![Vec::new(); g_map.len()];

    for (&from, to_nodes) in g_map.iter() {
        let next_id = mapping.len();
        let from_idx = *mapping.entry(from).or_insert(next_id);
        for &to in to_nodes {
            let next_id = mapping.len();
            let to_idx = *mapping.entry(to).or_insert(next_id);
            data[from_idx].push(to_idx);
        }
    }

    Graph::new(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seperator() {
        let g = Graph::new({ vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]] });
        let s = g.get_separator(2, 0.33, 42, Mode::Fast);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_get_graph() {
        let mut g = HashMap::new();
        g.insert(99, vec![0, 2]);
        g.insert(0, vec![99, 2]);
        g.insert(2, vec![99, 0]);
        let g = get_graph(&g);
        assert_eq!(g.get_num_nodes(), 3);
        assert_eq!(g.get_num_edges(), 3);

        let set1: HashSet<usize> = [1, 2].iter().cloned().collect();
        let set2: HashSet<usize> = [0, 1].iter().cloned().collect();
        let set3: HashSet<usize> = [0, 2].iter().cloned().collect();
        for i in 0..g.get_num_nodes() {
            let test_set: HashSet<usize> = g.get_neighbors(i).iter().cloned().collect();
            assert!(test_set == set1 || test_set == set2 || test_set == set3);
        }
    }

    #[test]
    fn test_get_subgraphs() {
        let g = Graph::new({ vec![vec![1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3]] });
        let s = vec![2].into_iter().collect();
        let subgraphs = g.get_subgraphs(&s);
        assert_eq!(subgraphs.len(), 2);
        assert_eq!(subgraphs[0].get_num_nodes(), 2);
        assert_eq!(subgraphs[1].get_num_edges(), 1);
        assert_eq!(subgraphs[0].get_num_nodes(), 2);
        assert_eq!(subgraphs[1].get_num_edges(), 1);
    }

    #[test]
    fn test_get_subgraphs2() {
        let g = Graph::new({
            vec![
                vec![1, 2],
                vec![0, 2],
                vec![0, 1, 3],
                vec![2, 4, 5],
                vec![3, 5],
                vec![3, 4],
            ]
        });
        let s = vec![3].into_iter().collect();
        let subgraphs = g.get_subgraphs(&s);

        assert_eq!(subgraphs.len(), 2);
        for subgraph in subgraphs {
            for i in 0..subgraph.get_num_nodes() {
                //println!("{:?}", subgraph.get_neighbors(i));
            }
            //println!();
        }
    }
}
