use hashbrown::{HashMap, HashSet};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;

use crate::graph::Graph;

impl Graph {
    pub fn from_osm_xml<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let f_reader = BufReader::new(file);
        let mut xml_reader = Reader::from_reader(f_reader);
        // xml_reader.trim_text(true);

        let mut buf = Vec::new();
        let mut current_way_nodes: Vec<i64> = Vec::new();
        let mut in_way_element = false;

        let mut all_edge_osm_pairs: Vec<(i64, i64)> = Vec::new();
        let mut nodes_participating_in_ways: HashSet<i64> = HashSet::new();

        loop {
            match xml_reader.read_event_into(&mut buf)? {
                Event::Start(e) => {
                    match e.name().as_ref() {
                        b"way" => {
                            in_way_element = true;
                            current_way_nodes.clear();
                        }
                        b"nd" if in_way_element => {
                            for attr_result in e.attributes() {
                                let attr = attr_result?;
                                if attr.key.as_ref() == b"ref" {
                                    let node_id_str = String::from_utf8(attr.value.into_owned())?;
                                    let node_id = node_id_str.parse::<i64>()?;
                                    current_way_nodes.push(node_id);
                                    nodes_participating_in_ways.insert(node_id);
                                }
                            }
                        }
                        _ => (),
                    }
                }
                Event::Empty(e) => {
                     match e.name().as_ref() {
                        b"nd" if in_way_element => {
                            for attr_result in e.attributes() {
                                let attr = attr_result?;
                                if attr.key.as_ref() == b"ref" {
                                    let node_id_str = String::from_utf8(attr.value.into_owned())?;
                                    let node_id = node_id_str.parse::<i64>()?;
                                    current_way_nodes.push(node_id);
                                    nodes_participating_in_ways.insert(node_id);
                                }
                            }
                        }
                        _ => (),
                    }
                }
                Event::End(e) => {
                    if e.name().as_ref() == b"way" {
                        in_way_element = false;
                        if current_way_nodes.len() >= 2 {
                            for i in 0..(current_way_nodes.len() - 1) {
                                all_edge_osm_pairs.push((current_way_nodes[i], current_way_nodes[i + 1]));
                            }
                        }
                        current_way_nodes.clear();
                    }
                }
                Event::Eof => break,
                _ => (),
            }
            buf.clear();
        }

        let mut osm_id_to_internal_index: HashMap<i64, usize> = HashMap::new();

        let mut internal_idx_counter = 0;
        
        let mut sorted_node_ids: Vec<i64> = nodes_participating_in_ways.into_iter().collect();
        sorted_node_ids.sort_unstable();

        for osm_node_id in sorted_node_ids {
            osm_id_to_internal_index.insert(osm_node_id, internal_idx_counter);
            internal_idx_counter += 1;
        }

        let num_nodes = internal_idx_counter;
        let mut adj_list: Vec<HashSet<usize>> = vec![HashSet::new(); num_nodes];

        for (osm_id1, osm_id2) in all_edge_osm_pairs {
            if let (Some(&internal_id1), Some(&internal_id2)) = (
                osm_id_to_internal_index.get(&osm_id1),
                osm_id_to_internal_index.get(&osm_id2),
            ) {
                adj_list[internal_id1].insert(internal_id2);
                adj_list[internal_id2].insert(internal_id1);
            }
        }

        Ok(Graph { data: adj_list })
    }
}
