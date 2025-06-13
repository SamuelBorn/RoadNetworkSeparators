use crate::random_set::RandomSet;
use hashbrown::{HashMap, HashSet};
use planar::planarize;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::fmt::format;
use std::hash::Hash;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::vec;
use std::{collections::BTreeSet, fs, io, path::Path};
use tempfile::NamedTempFile;

use crate::{library, separator};
pub mod cbrt_bridged;
pub mod cbrt_grid;
pub mod cbrt_maximal;
pub mod delaunay;
pub mod example;
pub mod gabriel_graph;
pub mod geometric_graph;
pub mod grid;
pub mod hierachical_delaunay;
pub mod hierachical_disks;
pub mod highway;
pub mod knn;
pub mod nested_grid;
pub mod nested_sparse;
pub mod noise;
pub mod planar;
pub mod relative_neighborhood;
pub mod tree;
pub mod unit_disk;
pub mod voronoi;

// representation of bidirectional graph
// all algorithms assume that if a,b is in the graph, then b,a is also in the graph
#[derive(Debug, Clone)]
pub struct Graph {
    pub data: Vec<HashSet<usize>>,
}

impl Graph {
    pub fn new(edges: Vec<Vec<usize>>) -> Self {
        Graph {
            data: edges
                .into_iter()
                .map(|(neighbors)| neighbors.into_iter().collect::<HashSet<usize>>())
                .collect(),
        }
    }

    pub fn with_node_count(n: usize) -> Self {
        Graph {
            data: vec![HashSet::new(); n],
        }
    }

    pub fn from_edge_list(edges: Vec<(usize, usize)>) -> Self {
        let max_idx = edges
            .par_iter()
            .map(|(a, b)| usize::max(*a, *b))
            .max()
            .unwrap_or(0);

        let mut data = (0..max_idx + 1)
            .into_par_iter()
            .map(|_| Mutex::new(HashSet::new()))
            .collect::<Vec<_>>();

        edges.par_iter().for_each(|(u, v)| {
            {
                data[*u].lock().unwrap().insert(*v);
            }
            {
                data[*v].lock().unwrap().insert(*u);
            }
        });

        Graph {
            data: data
                .into_iter()
                .map(|mutex| mutex.into_inner().unwrap())
                .collect(),
        }
    }

    pub fn from_edge_list_directed(edges: Vec<(usize, usize)>) -> Self {
        let max_idx = edges
            .iter()
            .map(|(a, b)| usize::max(*a, *b))
            .max()
            .unwrap_or(0);

        let mut g = Graph::with_node_count(max_idx + 1);
        for (u, v) in edges {
            g.add_directed_edge(u, v);
        }
        g
    }

    pub fn from_pace(file: &Path) -> Self {
        let edges = fs::read_to_string(file)
            .unwrap()
            .lines()
            .skip(1)
            .map(|line| {
                let mut parts = line.split_whitespace();
                let u = parts.next().unwrap().parse::<usize>().unwrap() - 1;
                let v = parts.next().unwrap().parse::<usize>().unwrap() - 1;
                (u, v)
            })
            .collect::<Vec<_>>();

        Graph::from_edge_list(edges)
    }

    pub fn info(&self) {
        println!(
            "n={}\tm={}\tdeg={:.4}\tconn:{}\tbi:{}",
            self.get_num_nodes(),
            self.get_num_edges(),
            self.get_average_degree(),
            self.is_connected(),
            self.is_undirected(),
        );
    }

    pub fn from_edge_list_file(file: &Path) -> io::Result<Self> {
        let edges = library::read_edge_list(file)?;
        Ok(Graph::from_edge_list(edges))
    }

    pub fn from_file(dir: &Path) -> io::Result<Self> {
        let mut xadj = library::read_to_usize_vec(&dir.join("first_out"));
        xadj.pop();
        let adjncy = library::read_to_usize_vec(&dir.join("head"));

        let data: Vec<HashSet<_>> = xadj
            .par_iter()
            .enumerate()
            .map(|(i, &start)| {
                let end = xadj.get(i + 1).copied().unwrap_or(adjncy.len());
                adjncy[start..end].iter().copied().collect()
            })
            .collect();

        Ok(Graph { data })
    }

    pub fn from_file_directed(dir: &Path) -> io::Result<Self> {
        let xadj = library::read_to_usize_vec(&dir.join("first_out"));
        let adjncy = library::read_to_usize_vec(&dir.join("head"));

        let mut g = Graph::with_node_count(xadj.len() - 1);

        for i in 0..xadj.len() - 1 {
            for j in xadj[i]..xadj[i + 1] {
                g.add_directed_edge(i, adjncy[j]);
            }
        }

        Ok(g)
    }

    pub fn save(&self, dir: &Path) -> io::Result<()> {
        fs::create_dir_all(dir)?;
        let (xadj, adjncy) = self.get_adjacency_array();
        library::write_binary_vec(&xadj, &dir.join("first_out"))?;
        library::write_binary_vec(&adjncy, &dir.join("head"))
    }

    pub fn save_metis(&self, file: &Path) {
        let mut res = String::new();
        res.push_str(&format!(
            "{} {}\n",
            self.get_num_nodes(),
            self.get_num_edges()
        ));
        for neighbors in &self.data {
            for neighbor in neighbors {
                res.push_str(&format!("{} ", neighbor + 1));
            }
            res.push('\n');
        }
        fs::write(file, res).expect("Unable to write file");
    }

    pub fn save_pace(&self, file: &Path) {
        let mut res = String::new();
        res.push_str(&format!(
            "p tw {} {}\n",
            self.get_num_nodes(),
            self.get_num_edges()
        ));
        for (u, v) in self.get_directed_edges() {
            res.push_str(&format!("{} {}\n", u + 1, v + 1));
        }
        fs::write(file, res).expect("Unable to write file");
    }

    pub fn has_edge(&self, i: usize, j: usize) -> bool {
        self.data[i].contains(&j)
    }

    pub fn add_edges(&mut self, edges: &[(usize, usize)]) {
        for (i, j) in edges {
            self.add_edge(*i, *j);
        }
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        self.data[i].insert(j);
        self.data[j].insert(i);
    }

    // warning! using this method breaks the assumption of undirected graphs
    // Only use it if you know what you are doing
    pub fn add_directed_edge(&mut self, i: usize, j: usize) {
        self.data[i].insert(j);
    }

    pub fn invert(&mut self) {
        let mut new_data = vec![HashSet::new(); self.get_num_nodes()];

        for (u, neighbors) in self.data.iter().enumerate() {
            for &v in neighbors {
                new_data[v].insert(u);
            }
        }

        self.data = new_data;
    }

    pub fn make_undirected(&mut self) {
        for (u, v) in self.get_edges() {
            self.add_directed_edge(v, u);
        }
    }

    pub fn remove_edge(&mut self, i: usize, j: usize) {
        self.data[i].remove(&j);
        self.data[j].remove(&i);
    }

    pub fn remove_random_edge(&mut self) -> (usize, usize) {
        println!(
            "WARNING: deprecated function remove_random_edge\n use remove_reandom_edges instead"
        );
        let (u, v) = self.get_random_edge();
        self.remove_edge(u, v);
        (u, v)
    }

    pub fn remove_random_edges(&mut self, num_edges: usize) {
        let mut edges = self.get_directed_edges();
        edges.shuffle(&mut thread_rng());
        edges.truncate(edges.len() - num_edges);
        let g = Graph::from_edge_list(edges);
        self.data = g.data;
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
        self.data
            .par_iter()
            .enumerate()
            .flat_map(|(i, neighbors)| neighbors.iter().map(|&j| (i, j)).collect::<Vec<_>>())
            .collect()
    }

    pub fn get_directed_edges(&self) -> Vec<(usize, usize)> {
        self.get_edges()
            .par_iter()
            .filter(|&(i, j)| i < j)
            .cloned()
            .collect()
    }

    pub fn get_random_neighbor(&self, u: usize) -> Option<&usize> {
        let mut rng = rand::thread_rng();
        self.get_neighbors(u).iter().choose(&mut rng)
    }

    pub fn get_random_node(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..self.get_num_nodes())
    }

    pub fn get_random_edge(&mut self) -> (usize, usize) {
        loop {
            let u = self.get_random_node();
            if let Some(v) = self.get_random_neighbor(u) {
                return (u, *v);
            }
        }
    }

    pub fn increase_size_to(&mut self, n: usize) {
        if n < self.data.len() {
            return;
        }
        self.data.resize(n, HashSet::new());
    }

    pub fn clear_vertex_edges(&mut self, u: usize) {
        for v in self.get_neighbors(u).clone() {
            self.data[v].remove(&u);
        }
        self.data[u].clear();
    }

    // returns index of new node
    pub fn add_node(&mut self) -> usize {
        self.data.push(HashSet::new());
        self.data.len() - 1
    }

    pub fn get_num_nodes(&self) -> usize {
        self.data.len()
    }

    pub fn nodes_iter(&self) -> std::ops::Range<usize> {
        0..self.get_num_nodes()
    }

    pub fn get_num_edges(&self) -> usize {
        self.data.iter().map(|v| v.len()).sum::<usize>() / 2
    }

    pub fn get_average_degree(&self) -> f64 {
        self.data.iter().map(|v| v.len()).sum::<usize>() as f64 / self.get_num_nodes() as f64
    }

    pub fn degree_distribution(&self) -> Vec<f64> {
        let max_degree = self.data.par_iter().map(|v| v.len()).max().unwrap();
        let mut distribution = vec![0; max_degree + 1];
        for set in self.data.iter() {
            distribution[set.len()] += 1;
        }
        let sum = distribution.iter().sum::<usize>() as f64;
        distribution.iter().map(|&d| d as f64 / sum).collect()
    }

    pub fn cummulative_degree_distribution(&self) -> Vec<f64> {
        let dd = self.degree_distribution();
        dd.iter()
            .scan(0., |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect()
    }

    pub fn get_neighbors(&self, u: usize) -> &HashSet<usize> {
        &self.data[u]
    }

    pub fn get_neighbors_mut(&mut self, u: usize) -> &mut HashSet<usize> {
        &mut self.data[u]
    }

    pub fn add_neighbors(&mut self, u: usize, neighbors: &HashSet<usize>) {
        self.data[u].extend(neighbors);
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

    pub fn is_connected(&self) -> bool {
        let distances = self.bfs(0);
        distances.par_iter().all(|&d| d != usize::MAX)
    }

    pub fn is_undirected(&self) -> bool {
        self.get_edges()
            .par_iter()
            .all(|&(u, v)| self.has_edge(v, u))
    }

    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.get_num_nodes()];
        distances[start] = 0;
        let mut queue = VecDeque::with_capacity(self.get_num_nodes().isqrt() * 2);
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

    pub fn bfs_bounded(&self, start: usize, bound: usize) -> HashMap<usize, usize> {
        let mut distances = HashMap::with_capacity(bound);
        let mut queue = VecDeque::with_capacity(2 * self.get_num_nodes().isqrt());
        queue.push_back((start, 0));

        while distances.len() < bound + 1 && !queue.is_empty() {
            let (u, depth) = queue.pop_front().unwrap();
            if distances.contains_key(&u) {
                continue;
            }
            distances.insert(u, depth);

            for &v in self.get_neighbors(u) {
                if !distances.contains_key(&v) {
                    queue.push_back((v, depth + 1));
                }
            }
        }

        distances.remove(&start);
        distances
    }

    // warning: this is actually slower
    pub fn bfs_parallel(&self, start: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.get_num_nodes()];
        distances[start] = 0;
        let mut current_front = HashSet::new();
        current_front.insert(&start);
        let mut depth = 1;

        while !current_front.is_empty() {
            current_front = current_front
                .par_iter()
                .flat_map(|&&u| self.get_neighbors(u))
                .filter(|&&v| distances[v] == usize::MAX)
                .collect::<HashSet<_>>();

            current_front.iter().for_each(|&&v| {
                distances[v] = depth;
            });
            depth += 1;
        }

        distances
    }

    pub fn dijkstra(&self, start: usize, end: usize) -> usize {
        let n = self.data.len();
        let mut distances = vec![usize::MAX; n];
        distances[start] = 0;
        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), start));

        while let Some((Reverse(hops), u)) = pq.pop() {
            if u == end {
                return hops;
            }
            if hops > distances[u] {
                continue;
            }
            for &v in &self.data[u] {
                let new_hops = hops + 1;
                if new_hops < distances[v] {
                    distances[v] = new_hops;
                    pq.push((Reverse(new_hops), v));
                }
            }
        }
        usize::MAX
    }

    pub fn dijkstra_multi(&self, start: usize, ends: HashSet<usize>) -> Vec<usize> {
        let n = self.data.len();
        let mut distances = vec![usize::MAX; n];
        let mut result = vec![usize::MAX; ends.len()];
        let mut end_indices: Vec<usize> = ends.into_iter().collect(); // Convert HashSet to Vec for indexing
        let mut found_count = 0;

        distances[start] = 0;
        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), start));

        while let Some((Reverse(hops), u)) = pq.pop() {
            // Check if this is one of our target ends
            if let Some(pos) = end_indices.iter().position(|&x| x == u) {
                if result[pos] == usize::MAX {
                    // Only update if we haven't found it yet
                    result[pos] = hops;
                    found_count += 1;
                    if found_count == end_indices.len() {
                        return result;
                    }
                }
            }

            if hops > distances[u] {
                continue;
            }

            for &v in &self.data[u] {
                let new_hops = hops + 1;
                if new_hops < distances[v] {
                    distances[v] = new_hops;
                    pq.push((Reverse(new_hops), v));
                }
            }
        }

        result // Return vector with distances (some may still be MAX if unreachable)
    }

    pub fn hop_overview(&self, n: usize) -> Vec<usize> {
        (0..n)
            .into_par_iter()
            .flat_map(|_| self.bfs(thread_rng().gen_range(0..self.get_num_nodes())))
            .collect::<Vec<_>>()
    }

    pub fn hop_overview_write(&self, n: usize, file: &Path) {
        let mutex = Arc::new(Mutex::new(0));
        fs::write(file, "").expect("Unable to write file");
        (0..n).into_par_iter().for_each(|_| {
            let hops = self.bfs(thread_rng().gen_range(0..self.get_num_nodes()));
            mutex.lock().unwrap();
            library::append_to_file(
                file,
                &hops.iter().map(|&h| format!("{}\n", h)).collect::<String>(),
            );
        });
    }

    pub fn hop_overview_contracted_bins(&self, n: usize, bins: usize, name: &str) {
        let mut g = self.clone();
        g.contract_degree_2_nodes();
        let mut g = g.largest_connected_component();
        let diameter = g.get_hop_diameter_approx();
        let bin_edges = library::get_bin_edges(diameter as f64, bins);

        let hist = (0..n)
            .into_par_iter()
            .map(|_| {
                library::histogram(
                    g.bfs(thread_rng().gen_range(0..g.get_num_nodes()))
                        .iter()
                        .map(|&h| h as f64),
                    &bin_edges,
                )
            })
            .reduce(
                || vec![0; &bin_edges.len() - 1],
                |acc, x| library::add_vecs(&acc, &x),
            );

        library::write_histogram_to_file(name, &bin_edges, &hist);
    }

    pub fn largest_connected_component(&self) -> Graph {
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
            .unwrap_or((start, &0));
        (furthest_node.0, *furthest_node.1)
    }

    // Diameter Karlsruhe: 323
    // Diameter Germany: 1163
    pub fn get_hop_diameter_approx(&self) -> usize {
        let (furthest_node, _) = self.get_furthest_node(0);
        let (_, diameter) = self.get_furthest_node(furthest_node);
        diameter
    }

    pub fn get_hop_diameter_approx_n(&self, n: usize) -> usize {
        (0..n)
            .into_par_iter()
            .map(|_| {
                let (furthest_node, _) =
                    self.get_furthest_node(thread_rng().gen_range(0..self.get_num_nodes()));
                let (_, diameter) = self.get_furthest_node(furthest_node);
                diameter
            })
            .max()
            .unwrap_or(0)
    }

    pub fn get_hop_diameter_primitive(&self) -> usize {
        let chunksize = 3_000;
        let starttime = std::time::Instant::now();

        (0..self.get_num_nodes())
            .collect::<Vec<_>>()
            .chunks(chunksize)
            .enumerate()
            .map(|(i, chunk)| {
                println!(
                    "Computing diameter: {}% ({:.2} seconds)",
                    (i * chunksize) as f64 / self.get_num_nodes() as f64 * 100.0,
                    starttime.elapsed().as_secs_f64()
                );
                chunk
                    .into_par_iter()
                    .map(|&n| self.bfs(n).iter().max().cloned().unwrap_or(0))
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap()
    }

    pub fn get_hop_diameter_contracted_approx(&self) -> usize {
        let mut g = self.clone();
        g.contract_degree_2_nodes();
        let g = g.largest_connected_component();
        g.get_hop_diameter_approx()
    }

    pub fn recurse_diameter(&self, file: Option<&Path>) {
        if let Some(x) = file {
            fs::write(x, "");
        }
        let mut g = self.clone();
        println!("{} {}", g.get_num_nodes(), g.get_hop_diameter_approx());
        library::optional_append_to_file(
            file,
            &format!("{} {}\n", g.get_num_nodes(), g.get_hop_diameter_approx()),
        );
        while g.get_num_nodes() > 100 {
            let sep = g.get_separator_wrapper(separator::Mode::Fast);
            let sub = g.get_subgraphs(&sep);
            sub.par_iter().for_each(|g| {
                let diam = g.diameter_ifub().unwrap_or(0);
                println!("{} {}", g.get_num_nodes(), diam);
                library::optional_append_to_file(
                    file,
                    &format!("{} {}\n", g.get_num_nodes(), diam),
                );
            });
            g = sub.into_iter().max_by_key(|g| g.get_num_nodes()).unwrap();
        }
    }

    pub fn get_degree(&self, u: usize) -> usize {
        self.get_neighbors(u).len()
    }

    // deletes random edges to match avg degree, and picks the largest connected component
    pub fn enforce_average_degree_connected(&mut self, target_degree: f64) {
        let mut edges = self.get_directed_edges();
        edges.shuffle(&mut thread_rng());
        edges.truncate((self.get_num_nodes() as f64 * target_degree) as usize / 2);
        let g = Graph::from_edge_list(edges);
        let g = g.largest_connected_component();
        self.data = g.data;
    }

    pub fn approx_degrees(&mut self, target_dist: &[f64]) {
        let mut g = self
            .data
            .iter()
            .map(|set| RandomSet::from_set(set))
            .collect::<Vec<_>>();
        let target_node_degrees = target_dist
            .into_iter()
            .map(|&d| (d * self.get_num_nodes() as f64) as usize)
            .collect::<Vec<_>>();

        let mut nodes_with_degree = vec![RandomSet::new(); self.degree_distribution().len()];
        for i in 0..g.len() {
            nodes_with_degree[g[i].len()].insert(i);
        }

        let mut current = nodes_with_degree.len() - 1;
        while current > 1 {
            let Some(&v) = nodes_with_degree[current].choose_random() else {
                current -= 1;
                continue;
            };
            let Some(&u) = g[v].choose_random() else {
                continue;
            };

            // move node and neigbor one degree down
            nodes_with_degree[current].remove(&v);
            nodes_with_degree[current - 1].insert(v);
            nodes_with_degree[g[u].len()].remove(&u);
            nodes_with_degree[g[u].len() - 1].insert(u);

            // update graph
            g[v].remove(&u);
            g[u].remove(&v);

            let compensation_factor = 1.2;
            if nodes_with_degree[current].len() as f64
                <= compensation_factor
                    * target_node_degrees.get(current).copied().unwrap_or(0) as f64
            {
                dbg!(current);
                current -= 1;
            }
        }

        // transform back to graph
        self.data = g.into_iter().map(|set| set.to_set()).collect::<Vec<_>>();

        let g_connected = self.largest_connected_component();
        self.data = g_connected.data;
    }

    pub fn print(&self) {
        for (i, neighbors) in self.data.iter().enumerate() {
            println!("{}: {:?}", i, neighbors);
        }
    }

    pub fn visualize(&self, name: &str) {
        if self.get_num_nodes() > 1_000 {
            println!("Graph too large to visualize with this method, use GeometricGraph instead.");
            return;
        }

        let g_path = format!("./output/graphs/{}", name);
        self.save(Path::new(&g_path));

        Command::new("python3")
            .arg("scripts/visualize_graph.py")
            .arg("--auto-layout")
            .arg(g_path)
            .spawn();
    }

    pub fn visualize_small(&self) {
        let f = NamedTempFile::new().unwrap();
        self.save_metis(f.path());

        Command::new("python3")
            .arg("scripts/visualize_metis.py")
            .arg(f.path())
            .spawn()
            .expect("Failed to execute command")
            .wait();
    }

    pub fn contract_degree_2_nodes(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for v in 0..self.get_num_nodes() {
                if self.get_neighbors(v).len() == 2 {
                    let mut iter = self.get_neighbors(v).iter();
                    let u1 = *iter.next().unwrap();
                    let u2 = *iter.next().unwrap();

                    self.remove_edge(v, u1);
                    self.remove_edge(v, u2);
                    self.add_edge(u1, u2);
                    changed = true;
                }
            }
        }
    }

    fn get_path(&self, start: usize, end: usize) -> Vec<usize> {
        let distances = self.bfs(start);
        let mut path = Vec::new();
        if distances[end] == usize::MAX {
            return path;
        }

        let mut curr = end;
        path.push(curr);

        while curr != start {
            for &neighbor in self.get_neighbors(curr) {
                if distances[neighbor] == distances[curr] - 1 {
                    curr = neighbor;
                    path.push(curr);
                    break;
                }
            }
        }
        path.reverse();
        path
    }

    fn get_midpoint(&self, start: usize, end: usize) -> usize {
        let path = self.get_path(start, end);
        if path.is_empty() {
            return start;
        }
        path[path.len() / 2]
    }

    pub fn meshedness_coefficient(&self) -> f64 {
        let m = self.get_num_edges() as f64;
        let n = self.get_num_nodes() as f64;
        (m - n + 1.) / (2. * n - 5.)
    }

    pub fn diameter_ifub(&self) -> Option<usize> {
        if self.get_num_nodes() <= 1 {
            return Some(0);
        }

        let (r1, _) = self.get_furthest_node(0);
        if self.get_degree(r1) == 0 {
            return None;
        }
        let (a1, ecc_a1) = self.get_furthest_node(r1);
        let (b1, _) = self.get_furthest_node(a1);
        let r2 = self.get_midpoint(a1, b1);
        let (a2, ecc_a2) = self.get_furthest_node(r2);
        let (b2, _) = self.get_furthest_node(a2);
        let u = self.get_midpoint(a2, b2);

        let initial_lb = ecc_a1.max(ecc_a2);

        let dists_from_u = self.bfs(u);
        let ecc_u = match dists_from_u.iter().filter(|&&d| d != usize::MAX).max() {
            Some(&max_dist) => max_dist,
            None => return None,
        };

        if dists_from_u.iter().any(|&d| d == usize::MAX) {
            return None;
        }

        let mut lb = initial_lb.max(ecc_u);
        let mut ub = 2 * ecc_u;

        if ub <= lb {
            return Some(lb);
        }

        let mut nodes_by_level: Vec<Vec<usize>> = vec![Vec::new(); ecc_u + 1];
        for (node, &dist) in dists_from_u.iter().enumerate() {
            nodes_by_level[dist].push(node);
        }

        // warning: this is just an approximation (but a very good one)
        // normally we need to checkk all levels, but this is a good heuristic and got all the
        // optimal values we checked
        for i in (1..=ecc_u).rev().take(8) {
            // for i in (1..=ecc_u).rev() {
            println!("Checking level {}: lb={}, ub={}", i, lb, ub);
            if ub <= lb {
                break;
            }

            let fringe_nodes = &nodes_by_level[i];
            let max_ecc_in_fringe = fringe_nodes
                .par_iter()
                .map(|&node| {
                    self.bfs(node)
                        .into_iter()
                        .filter(|&d| d != usize::MAX)
                        .max()
                        .unwrap_or(0)
                })
                .max()
                .unwrap_or(0);

            lb = lb.max(max_ecc_in_fringe);

            if lb > 2 * (i - 1) {
                return Some(lb);
            } else {
                ub = 2 * (i - 1);
            }
        }

        Some(lb)
    }
}

#[cfg(test)]
mod test {
    use crate::graph::example;

    use super::example::{europe, karlsruhe};

    #[test]
    fn simple_dijsktra_multi() {
        let g = example::example_c4().graph;
        let u = 0;
        let ends = vec![1, 2].into_iter().collect();
        let result = g.dijkstra_multi(u, ends);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
    }

    #[test]
    fn diameter_overview() {
        let cut_off = 100;
        let mut g = europe();
        println!("{} {}", g.get_num_nodes(), g.get_hop_diameter_approx());

        while g.get_num_nodes() > cut_off {
            let sep = g.get_separator_wrapper(crate::separator::Mode::Eco);
            let sub = g.get_subgraphs(&sep);
            sub.iter()
                .filter(|g| g.get_num_nodes() > cut_off)
                .for_each(|g| {
                    println!("{} {}", g.get_num_nodes(), g.get_hop_diameter_approx());
                });
            g = sub.into_iter().max_by_key(|g| g.get_num_nodes()).unwrap();
        }
    }
}
