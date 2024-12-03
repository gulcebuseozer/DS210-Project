use petgraph::graph::Graph;
use petgraph::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_graph(file_path: &str) -> Result<Graph<(), (), Directed>, std::io::Error> {
    let mut graph = Graph::<(), (), Directed>::new();
    let mut node_map = std::collections::HashMap::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') || line.is_empty() {
            continue; 
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue; 
        }
        let from = parts[0].parse::<usize>().unwrap();
        let to = parts[1].parse::<usize>().unwrap();
        let from_node = *node_map.entry(from).or_insert_with(|| graph.add_node(()));
        let to_node = *node_map.entry(to).or_insert_with(|| graph.add_node(()));
        graph.add_edge(from_node, to_node, ());
    }
    Ok(graph)
}
