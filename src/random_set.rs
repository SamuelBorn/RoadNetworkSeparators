use hashbrown::{HashMap, HashSet};
use rand::seq::SliceRandom;

// RandomSet is a data structure that supports insert, delete, and choose_random operations in O(1) time.

#[derive(Debug, Clone)]
pub struct RandomSet<T> {
    vec: Vec<T>,
    map: HashMap<T, usize>,
}

impl<T> RandomSet<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    pub fn new() -> Self {
        RandomSet {
            vec: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn from_set(set: &HashSet<T>) -> Self {
        let vec: Vec<T> = set.iter().cloned().collect();
        let map: HashMap<T, usize> = vec
            .iter()
            .enumerate()
            .map(|(i, x)| (x.clone(), i))
            .collect();
        RandomSet { vec, map }
    }
    
    pub fn to_set(&self) -> HashSet<T> {
        self.vec.iter().cloned().collect()
    }

    pub fn insert(&mut self, x: T) {
        if !self.map.contains_key(&x) {
            let index = self.vec.len();
            self.vec.push(x.clone());
            self.map.insert(x, index);
        }
    }

    pub fn remove(&mut self, x: &T) {
        if let Some(&i) = self.map.get(x) {
            let last = self.vec.len() - 1;
            if i != last {
                self.vec.swap(i, last);
                let last_elem = self.vec[i].clone();
                self.map.insert(last_elem, i);
            }
            self.vec.pop();
            self.map.remove(x);
        }
    }

    pub fn choose_random(&self) -> Option<&T> {
        self.vec.choose(&mut rand::thread_rng())
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}
