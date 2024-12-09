#[cfg(test)]
mod tests {
    use crate::graph::load_graph;
    use crate::analysis::{compute_degree_distribution, fit_power_law, simulate_resilience};
    use crate::visualization::{plot_histogram, plot_log_log};
    use petgraph::graph::{Graph};
    use petgraph::Directed;
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn test_load_graph_success() {
        let data = "1 2\n2 3\n3 1";
        let file_path = "test_graph.txt";
        std::fs::write(file_path, data).unwrap();
        let graph = load_graph(file_path).expect("Failed to load graph");
        std::fs::remove_file(file_path).unwrap();
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 3);
    }

    #[test]
    fn test_load_graph_invalid_file() {
        let result = load_graph("non_existent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_degree_distribution() {
        let mut graph = Graph::<(), (), Directed>::new();
        let a = graph.add_node(());
        let b = graph.add_node(()); 
        let c = graph.add_node(()); 
        graph.add_edge(a, b, ()); 
        graph.add_edge(a, c, ()); 
        graph.add_edge(b, c, ()); 
        let degree_distribution = compute_degree_distribution(&graph);
        let expected: HashMap<usize, usize> = [(0, 1), (1, 1), (2, 1)].iter().cloned().collect();
        assert_eq!(degree_distribution, expected);
    }
    
    #[test]
    fn test_fit_power_law() {
        let degree_distribution: HashMap<usize, usize> = [(1, 100), (2, 50), (3, 25), (4, 10)].iter().cloned().collect();
        let exponent = fit_power_law(&degree_distribution);
        assert!(exponent < 0.0 && exponent > -3.0, "Unexpected power-law exponent: {}", exponent);
    }

    #[test]
    fn test_simulate_resilience() {
        let mut graph = Graph::<(), (), Directed>::new();
        let a = graph.add_node(());
        let b = graph.add_node(());
        let c = graph.add_node(());
        let d = graph.add_node(());
        graph.add_edge(a, b, ());
        graph.add_edge(a, c, ());
        graph.add_edge(c, d, ());
        simulate_resilience(&graph, 2); 
    }

    #[test]
    fn test_plot_histogram() {
        let degree_distribution: HashMap<usize, usize> = [(1, 100), (2, 50), (3, 25)].iter().cloned().collect();
        let output_path = "./test_output";
        std::fs::create_dir_all(output_path).unwrap();
        let result = plot_histogram(&degree_distribution, output_path);
        let file_path = Path::new(output_path).join("degree_distribution.png");
        assert!(file_path.exists());
        std::fs::remove_dir_all(output_path).unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn test_plot_log_log_alternative() {
        use std::fs;
        let degree_distribution: HashMap<usize, usize> = [(1, 10), (2, 20), (3, 5)].iter().cloned().collect();
        let output_path = "./test_output_alt";
        fs::create_dir_all(output_path).unwrap();
        let result = plot_log_log(&degree_distribution, output_path);
        assert!(result.is_ok(), "Plot log-log function failed: {:?}", result);
        let file_path = std::path::Path::new(output_path).join("log_log_degree_distribution.png");
        assert!(
            file_path.exists(),
            "Expected log-log plot file to exist at {:?}, but it was not found.",
            file_path
        );
        if file_path.exists() {
            fs::remove_file(&file_path).unwrap();
        }
        fs::remove_dir_all(output_path).unwrap();
    }
}