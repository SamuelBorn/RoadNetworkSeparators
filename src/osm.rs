use geo::Point;
use hashbrown::{HashMap, HashSet};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::graph::geometric_graph::GeometricGraph;
use crate::graph::Graph;

impl GeometricGraph {
    pub fn from_osm_xml<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let f_reader = BufReader::new(file);
        let mut xml_reader = Reader::from_reader(f_reader);
        xml_reader.config_mut().trim_text(true);

        let mut buf = Vec::new();

        let mut osm_id_to_position: HashMap<i64, Point<f64>> = HashMap::new();
        let mut current_way_nodes: Vec<i64> = Vec::new();
        let mut in_way_element = false;
        let mut all_edge_osm_pairs: Vec<(i64, i64)> = Vec::new();
        let mut nodes_referenced_in_ways: HashSet<i64> = HashSet::new();

        loop {
            match xml_reader.read_event_into(&mut buf)? {
                Event::Start(ref e_start) => match e_start.name().as_ref() {
                    b"node" => {
                        let mut node_id_opt: Option<i64> = None;
                        let mut lat_opt: Option<f64> = None;
                        let mut lon_opt: Option<f64> = None;
                        for attr_result in e_start.attributes() {
                            let attr = attr_result?;
                            match attr.key.as_ref() {
                                b"id" => {
                                    node_id_opt =
                                        Some(String::from_utf8(attr.value.into_owned())?.parse()?)
                                }
                                b"lat" => {
                                    lat_opt =
                                        Some(String::from_utf8(attr.value.into_owned())?.parse()?)
                                }
                                b"lon" => {
                                    lon_opt =
                                        Some(String::from_utf8(attr.value.into_owned())?.parse()?)
                                }
                                _ => {}
                            }
                        }
                        if let (Some(id), Some(lat), Some(lon)) = (node_id_opt, lat_opt, lon_opt) {
                            osm_id_to_position.insert(id, Point::new(lon, lat));
                        }
                    }
                    b"way" => {
                        in_way_element = true;
                        current_way_nodes.clear();
                    }
                    b"nd" if in_way_element => {
                        for attr_result in e_start.attributes() {
                            let attr = attr_result?;
                            if attr.key.as_ref() == b"ref" {
                                let node_ref_id =
                                    String::from_utf8(attr.value.into_owned())?.parse::<i64>()?;
                                current_way_nodes.push(node_ref_id);
                                nodes_referenced_in_ways.insert(node_ref_id);
                                break;
                            }
                        }
                    }
                    _ => (),
                },
                Event::Empty(ref e_empty) => match e_empty.name().as_ref() {
                    b"node" => {
                        let mut node_id_opt: Option<i64> = None;
                        let mut lat_opt: Option<f64> = None;
                        let mut lon_opt: Option<f64> = None;
                        for attr_result in e_empty.attributes() {
                            let attr = attr_result?;
                            match attr.key.as_ref() {
                                b"id" => {
                                    node_id_opt =
                                        Some(String::from_utf8(attr.value.into_owned())?.parse()?)
                                }
                                b"lat" => {
                                    lat_opt =
                                        Some(String::from_utf8(attr.value.into_owned())?.parse()?)
                                }
                                b"lon" => {
                                    lon_opt =
                                        Some(String::from_utf8(attr.value.into_owned())?.parse()?)
                                }
                                _ => {}
                            }
                        }
                        if let (Some(id), Some(lat), Some(lon)) = (node_id_opt, lat_opt, lon_opt) {
                            osm_id_to_position.insert(id, Point::new(lon, lat));
                        }
                    }
                    b"way" => {
                        current_way_nodes.clear();
                    }
                    b"nd" if in_way_element => {
                        for attr_result in e_empty.attributes() {
                            let attr = attr_result?;
                            if attr.key.as_ref() == b"ref" {
                                let node_ref_id =
                                    String::from_utf8(attr.value.into_owned())?.parse::<i64>()?;
                                current_way_nodes.push(node_ref_id);
                                nodes_referenced_in_ways.insert(node_ref_id);
                                break;
                            }
                        }
                    }
                    _ => (),
                },
                Event::End(ref e_end) => {
                    if e_end.name().as_ref() == b"way" {
                        in_way_element = false;
                        if current_way_nodes.len() >= 2 {
                            for i in 0..(current_way_nodes.len() - 1) {
                                all_edge_osm_pairs
                                    .push((current_way_nodes[i], current_way_nodes[i + 1]));
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

        let mut final_osm_id_to_internal_index: HashMap<i64, usize> = HashMap::new();
        let mut final_positions_ordered: Vec<Point<f64>> = Vec::new();
        let mut internal_idx_counter = 0;

        let mut p_nodes_for_graph_sorted: Vec<i64> = nodes_referenced_in_ways
            .iter()
            .filter(|osm_id| osm_id_to_position.contains_key(*osm_id))
            .cloned()
            .collect();
        p_nodes_for_graph_sorted.sort_unstable();

        for osm_id in p_nodes_for_graph_sorted {
            if let Some(pos) = osm_id_to_position.get(&osm_id) {
                final_osm_id_to_internal_index.insert(osm_id, internal_idx_counter);
                final_positions_ordered.push(*pos);
                internal_idx_counter += 1;
            }
        }

        let num_final_nodes = internal_idx_counter;
        let mut adj_list: Vec<HashSet<usize>> = vec![HashSet::new(); num_final_nodes];

        for (osm_id1, osm_id2) in all_edge_osm_pairs {
            if let (Some(&internal_id1), Some(&internal_id2)) = (
                final_osm_id_to_internal_index.get(&osm_id1),
                final_osm_id_to_internal_index.get(&osm_id2),
            ) {
                adj_list[internal_id1].insert(internal_id2);
                adj_list[internal_id2].insert(internal_id1);
            }
        }

        let graph_struct = Graph { data: adj_list };
        Ok(GeometricGraph {
            graph: graph_struct,
            positions: final_positions_ordered,
        })
    }
}
