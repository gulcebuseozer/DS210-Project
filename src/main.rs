mod analysis;
mod graph;
mod visualization;
pub mod test;

use analysis::{compute_degree_distribution, fit_power_law};
use graph::load_graph;
use visualization::{plot_histogram, plot_log_log};

fn main() {
    let graph = load_graph("p2p-Gnutella05.txt").expect("Failed to load graph.");
    let degree_distribution = compute_degree_distribution(&graph);
    let exponent = fit_power_law(&degree_distribution);
    println!("Power-law exponent: {:.2}", exponent);

    if let Err(e) = plot_histogram(&degree_distribution, "./output") {
        eprintln!("Failed to generate histogram: {}", e);
    }
    if let Err(e) = plot_log_log(&degree_distribution, "./output") {
        eprintln!("Failed to generate log-log plot: {}", e);
    }
    
    analysis::simulate_resilience(&graph, 100);
    analysis::simulate_targeted_attacks(&graph, 5);
}
