// File: src/tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::collections::HashMap;
    use tempfile::NamedTempFile;
    use crate::{read_csv, remove_duplicates, write_txt, load_data, six_degrees_to_all, CharacterGraph};

    #[test]
    fn test_read_csv() {
        // Prepare a CSV string
        let csv_data = "Harry Potter,Ron Weasley\nHermione Granger,Harry Potter";
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        // Write CSV data to the temporary file
        let mut file = File::create(path).unwrap();
        file.write_all(csv_data.as_bytes()).unwrap();

        // Read CSV and check results
        let result = read_csv(path).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("Harry Potter".to_owned(), "Ron Weasley".to_owned()));
        assert_eq!(result[1], ("Hermione Granger".to_owned(), "Harry Potter".to_owned()));
    }

    #[test]
    fn test_remove_duplicates() {
        // Prepare input with duplicates
        let edges = vec![
            ("Harry Potter".to_owned(), "Ron Weasley".to_owned()),
            ("Hermione Granger".to_owned(), "Harry Potter".to_owned()),
            ("Harry Potter".to_owned(), "Ron Weasley".to_owned()), // Duplicate
        ];

        // Remove duplicates
        let result = remove_duplicates(&edges);

        // Verify result
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("Harry Potter".to_owned(), "Ron Weasley".to_owned()));
        assert_eq!(result[1], ("Hermione Granger".to_owned(), "Harry Potter".to_owned()));
    }

    #[test]
    fn test_write_txt() {
        // Prepare data to write
        let edges = vec![
            ("Harry Potter".to_owned(), "Ron Weasley".to_owned()),
            ("Hermione Granger".to_owned(), "Harry Potter".to_owned()),
        ];

        // Write to a temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        write_txt(path, &edges).unwrap();

        // Read back and verify content
        let file_content = fs::read_to_string(path).unwrap();
        assert_eq!(file_content, "Harry Potter,Ron Weasley\nHermione Granger,Harry Potter\n");
    }

    #[test]
    fn test_load_data() {
        // Prepare data for a temporary TXT file
        let txt_data = "Harry Potter,Ron Weasley\nHermione Granger,Harry Potter";
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        // Write TXT data to the temporary file
        let mut file = File::create(path).unwrap();
        file.write_all(txt_data.as_bytes()).unwrap();

        // Load data into a graph and verify
        let result = load_data(path).unwrap();
        assert_eq!(result.node_count(), 3); // 3 nodes: Harry Potter, Ron Weasley, Hermione Granger
        assert_eq!(result.edge_count(), 2); // 2 edges
    }

    #[test]
    fn test_six_degrees_to_all() {
        // Prepare a sample graph
        let mut graph = CharacterGraph::new();
        let node1 = graph.add_node("Harry Potter".to_owned());
        let node2 = graph.add_node("Ron Weasley".to_owned());
        let node3 = graph.add_node("Hermione Granger".to_owned());
        graph.add_edge(node1, node2, ());
        graph.add_edge(node2, node3, ());

        // Test degrees of separation from 'Harry Potter'
        let result = six_degrees_to_all(&graph, "Harry Potter").unwrap();

        // Verify distances
        let expected_distances: HashMap<String, usize> = [
            ("Ron Weasley".to_owned(), 1),
            ("Hermione Granger".to_owned(), 2),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(result.len(), 2); // Expect distances for 2 characters
        assert_eq!(result, expected_distances);
    }
}
