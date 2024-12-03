use petgraph::graph::Graph;
use petgraph::prelude::{NodeIndex, Directed};
use petgraph::algo::kosaraju_scc;
use rand::seq::IteratorRandom;
use rand::thread_rng; 
use std::collections::HashMap;

pub fn compute_degree_distribution(
    graph: &Graph<(), (), Directed>,
) -> HashMap<usize, usize> {
    let mut degree_map = HashMap::new();
    for node in graph.node_indices() {
        let degree = graph.edges(node).count();
        *degree_map.entry(degree).or_insert(0) += 1;
    }
    degree_map
}

pub fn fit_power_law(degree_distribution: &HashMap<usize, usize>) -> f64 {
    let mut total_nodes = 0;
    let mut log_sum = 0.0;
    for (&degree, &count) in degree_distribution.iter() {
        if degree > 0 {
            let degree_f64 = degree as f64;
            log_sum += count as f64 * degree_f64.ln();
            total_nodes += count;
        }
    }
    -1.0 * log_sum / total_nodes as f64
}

pub fn simulate_resilience(graph: &Graph<(), (), Directed>, num_removals: usize) {
    let mut rng = thread_rng();
    let mut graph = graph.clone();
    println!("Simulating resilience with random node failures:");
    for _ in 0..num_removals {
        if let Some(node) = graph.node_indices().choose(&mut rng) {
            graph.remove_node(node);
        }
        let largest_cc = largest_connected_component_size(&graph);
        println!("After removal: Largest connected component size = {}", largest_cc);
    }
}

pub fn identify_supernodes(graph: &Graph<(), (), Directed>, top_n: usize) -> Vec<(usize, usize)> {
    let mut degrees: Vec<(usize, usize)> = graph.node_indices().map(|node| (node.index(), graph.edges(node).count())).collect();
    degrees.sort_by(|a, b| b.1.cmp(&a.1));
    let top_supernodes = degrees.into_iter().take(top_n).collect::<Vec<_>>();
    println!("Top {} supernodes:", top_n);
    for (node, degree) in &top_supernodes {
        println!("Node: {}, Degree: {}", node, degree);
    }
    top_supernodes
}

pub fn simulate_targeted_attacks(graph: &Graph<(), (), Directed>, top_n: usize) {
    let mut graph = graph.clone();
    println!("Simulating targeted attacks on supernodes:");
    let supernodes = identify_supernodes(&graph, top_n);
    for (node, _) in supernodes {
        graph.remove_node(NodeIndex::new(node));
        let largest_cc = largest_connected_component_size(&graph);
        println!("After removing node {}: Largest connected component size = {}", node, largest_cc);
    }
}

fn largest_connected_component_size(graph: &Graph<(), (), Directed>) -> usize {
    kosaraju_scc(graph).iter().map(|component| component.len()).max().unwrap_or(0) 
}
