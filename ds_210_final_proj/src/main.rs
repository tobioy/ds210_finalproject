use std::error::Error;
use std::fs::File;
use std::time::Instant;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use rand::prelude::SliceRandom;

type CharacterGraph = petgraph::Graph<String, ()>;

/// This converts the CSV file to a cleaned TXT file and also removes duplicates
fn csv_to_txt(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let edges = read_csv(input_path)?;
    let cleaned_edges = remove_duplicates(&edges);
    write_txt(output_path, &cleaned_edges)?;
    Ok(())
}

/// This reads the CSV then creates a vector of edges
fn read_csv(input_path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let nodes: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if nodes.len() >= 2 {
            let name1 = nodes[0].to_owned();
            let name2 = nodes.get(1).map_or("", |&s| s).to_owned();
            edges.push((name1, name2));
        }
    }

    Ok(edges)
}

/// Removes duplicate edges from a vector
fn remove_duplicates(edges: &[(String, String)]) -> Vec<(String, String)> {
    let mut unique_edges = HashMap::new();

    for &(ref node1, ref node2) in edges {
        unique_edges.insert((node1.clone(), node2.clone()), ());
    }

    unique_edges.keys().cloned().collect()
}

/// Uses data retried from CSV to create TXT file - writes edges to vector
fn write_txt(output_path: &str, edges: &[(String, String)]) -> Result<(), Box<dyn Error>> {
    let file = File::create(output_path)?;
    let mut writer = std::io::BufWriter::new(file);

    for (node1, node2) in edges {
        writeln!(writer, "{},{}", node1, node2)?;
    }

    Ok(())
}

/// Reads character relationship from TXT and laods it into a graph
fn load_data(file_path: &str) -> Result<CharacterGraph, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut graph = CharacterGraph::new();

    let mut node_map: HashMap<String, _> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let characters: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if characters.len() < 2 {
            continue; // Skip invalid lines
        }

        let source = characters[0].to_owned();
        let target = characters[1].to_owned();

        let source_node = *node_map.entry(source.clone()).or_insert_with(|| graph.add_node(source.clone()));
        let target_node = *node_map.entry(target.clone()).or_insert_with(|| graph.add_node(target.clone()));

        graph.add_edge(source_node, target_node, ());
    }

    Ok(graph)
}

/// randomly selects a character
fn random_sample<'a, T: Clone>(items: &'a [T], sample_size: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    items.choose_multiple(&mut rng, sample_size).cloned().collect()
}

// calculates degrees of seperation from randomly chosen character to all 
fn six_degrees_to_all(graph: &CharacterGraph, start: &str) -> Result<HashMap<String, usize>, Box<dyn Error>> {
    let node_map: HashMap<&str, _> = graph
        .node_indices()
        .map(|idx| graph.node_weight(idx).unwrap())
        .map(|s| s.as_str()) 
        .zip(graph.node_indices()) 
        .collect();

    let start_node = *node_map.get(start).ok_or("Start character not found")?;

    let mut distances: HashMap<String, usize> = HashMap::new();

    for node in graph.node_indices() {
        if node != start_node {
            let target_character = graph.node_weight(node).unwrap().clone();
            let result = petgraph::algo::astar(
                &graph,
                start_node,
                |finish| finish == node,
                |_| 1,
                |_| 0,
            );

            if let Some((distance, _)) = result {
                distances.insert(target_character, distance);
            }
        }
    }

    Ok(distances)
}


fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "hp_character_network.csv";
    let output_path = "hp_character_network.txt";

    csv_to_txt(input_path, output_path)?;
    println!("CSV to TXT successful!");

    let graph = load_data(output_path)?;
    println!("Graph successfull!");

    let sample_size = 5;
    let node_indices: Vec<_> = graph.node_indices().collect();
    let random_characters = random_sample(&node_indices, sample_size)
        .into_iter()
        .map(|idx| graph.node_weight(idx).unwrap())
        .collect::<Vec<_>>();

    let start_character = random_characters.choose(&mut rand::thread_rng()).unwrap();
    println!("Randomly selected start character: {}", start_character);

    let start_time = Instant::now();
    let distances = six_degrees_to_all(&graph, start_character)?;
    let elapsed_time = start_time.elapsed().as_micros();

    // Output distances
    println!("Degrees of separation from '{}':", start_character);
    for (character, distance) in &distances {
        println!("{}: {} degrees", character, distance);
    }

    println!("Execution time: {} microseconds", elapsed_time);

    Ok(())
}
