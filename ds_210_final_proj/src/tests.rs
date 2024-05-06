#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_to_txt() {
        // defined test input data directly 
        let edges = vec![
            ("Harry Potter".to_string(), "Hermione Granger".to_string()),
            ("Ron Weasley".to_string(), "Hermione Granger".to_string()),
        ];

        let output_path = "test_output.txt";

        let result = csv_to_txt_from_edges(&edges, output_path);
        assert!(result.is_ok());

    }

    #[test]
    fn test_read_csv() {
        let file_content = "Harry Potter, Hermione Granger\nRon Weasley, Hermione Granger\n";

        let result = read_csv_from_string(file_content);
        assert!(result.is_ok());
        let edges = result.unwrap();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_remove_duplicates() {
        let edges = vec![
            ("Harry Potter".to_string(), "Hermione Granger".to_string()),
            ("Ron Weasley".to_string(), "Hermione Granger".to_string()),
            ("Harry Potter".to_string(), "Hermione Granger".to_string()),
        ];

        let unique_edges = remove_duplicates(&edges);
        assert_eq!(unique_edges.len(), 2);
    }

    #[test]
    fn test_load_data() {
        let file_content = "Harry Potter, Hermione Granger\nRon Weasley, Hermione Granger\n";

        let result = load_data_from_string(file_content);
        assert!(result.is_ok());
        let graph = result.unwrap();

        let node_indices: Vec<_> = graph.node_indices().collect();
        assert_eq!(node_indices.len(), 3); // 3 nodes expected 
    }

    #[test]
    fn test_six_degrees_to_all() {
        let mut graph = CharacterGraph::new();
        let node1 = graph.add_node("Harry Potter".to_string());
        let node2 = graph.add_node("Hermione Granger".to_string());
        let node3 = graph.add_node("Ron Weasley".to_string());
        graph.add_edge(node1, node2, ());
        graph.add_edge(node2, node3, ());

        let result = six_degrees_to_all(&graph, "Harry Potter");
        assert!(result.is_ok());
        let distances = result.unwrap();

        assert_eq!(distances.len(), 2); 
        assert_eq!(distances.get("Hermione Granger"), Some(&1));
        assert_eq!(distances.get("Ron Weasley"), Some(&2));
    }
}
