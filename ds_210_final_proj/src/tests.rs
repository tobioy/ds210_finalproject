#[cfg(test)]
mod tests {
    use super::*;

    // Define test cases for csv_to_txt function
    #[test]
    fn test_csv_to_txt() {
        // Define test input data directly
        let edges = vec![
            ("Harry Potter".to_string(), "Hermione Granger".to_string()),
            ("Ron Weasley".to_string(), "Hermione Granger".to_string()),
        ];

        // Define test output path
        let output_path = "test_output.txt";

        // Test csv_to_txt function
        let result = csv_to_txt_from_edges(&edges, output_path);
        assert!(result.is_ok());

        // Additional assertions can be added if required
    }

    // Define test cases for read_csv function
    #[test]
    fn test_read_csv() {
        // Define test input content directly
        let file_content = "Harry Potter, Hermione Granger\nRon Weasley, Hermione Granger\n";

        // Test read_csv function with inline content
        let result = read_csv_from_string(file_content);
        assert!(result.is_ok());
        let edges = result.unwrap();
        assert_eq!(edges.len(), 2);
    }

    // Define test cases for remove_duplicates function
    #[test]
    fn test_remove_duplicates() {
        // Define test edges with duplicates
        let edges = vec![
            ("Harry Potter".to_string(), "Hermione Granger".to_string()),
            ("Ron Weasley".to_string(), "Hermione Granger".to_string()),
            ("Harry Potter".to_string(), "Hermione Granger".to_string()),
        ];

        // Test remove_duplicates function
        let unique_edges = remove_duplicates(&edges);
        assert_eq!(unique_edges.len(), 2);
    }

    #[test]
    fn test_load_data() {
        // Define test input content directly
        let file_content = "Harry Potter, Hermione Granger\nRon Weasley, Hermione Granger\n";

        // Test load_data function with inline content
        let result = load_data_from_string(file_content);
        assert!(result.is_ok());
        let graph = result.unwrap();

        // Validate the loaded graph
        let node_indices: Vec<_> = graph.node_indices().collect();
        assert_eq!(node_indices.len(), 3); // 3 nodes expected (Harry Potter, Hermione Granger, Ron Weasley)
    }

    #[test]
    fn test_six_degrees_to_all() {
        // Prepare a sample graph
        let mut graph = CharacterGraph::new();
        let node1 = graph.add_node("Harry Potter".to_string());
        let node2 = graph.add_node("Hermione Granger".to_string());
        let node3 = graph.add_node("Ron Weasley".to_string());
        graph.add_edge(node1, node2, ());
        graph.add_edge(node2, node3, ());

        // Test six_degrees_to_all function
        let result = six_degrees_to_all(&graph, "Harry Potter");
        assert!(result.is_ok());
        let distances = result.unwrap();

        // Validate the computed distances
        assert_eq!(distances.len(), 2); // Expect distances to all other characters
        assert_eq!(distances.get("Hermione Granger"), Some(&1));
        assert_eq!(distances.get("Ron Weasley"), Some(&2));
    }
}
