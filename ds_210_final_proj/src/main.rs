use std::time::Instant;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::{HashMap, VecDeque};
use rand::prelude::SliceRandom;
use petgraph::graph::{DiGraph, NodeIndex};

type CharacterGraph = DiGraph<String, ()>;

/// This converts the CSV file to a cleaned TXT file and removes duplicates
fn csv_to_txt(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let edges = read_csv(input_path)?;
    let cleaned_edges = remove_duplicates(&edges);
    write_txt(output_path, &cleaned_edges)?;
    Ok(())
}

/// Reads the CSV file and creates a vector of edges
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

/// Removes duplicate edges from a vector (duplicate characters)
fn remove_duplicates(edges: &[(String, String)]) -> Vec<(String, String)> {
    let mut unique_edges = HashMap::new();

    for &(ref node1, ref node2) in edges {
        unique_edges.insert((node1.clone(), node2.clone()), ());
    }

    unique_edges.keys().cloned().collect()
}

/// Writes a vector of edges to a TXT file
fn write_txt(output_path: &str, edges: &[(String, String)]) -> Result<(), Box<dyn Error>> {
    let file = File::create(output_path)?;
    let mut writer = std::io::BufWriter::new(file);

    for (node1, node2) in edges {
        writeln!(writer, "{},{}", node1, node2)?;
    }

    Ok(())
}

/// Reads character relationships from TXT and loads them into a graph
fn load_data(file_path: &str) -> Result<CharacterGraph, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut graph = CharacterGraph::new();

    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let characters: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if characters.len() < 2 {
            continue; // skips lines w less than 2 characters
        }

        let source = characters[0].to_owned();
        let target = characters[1].to_owned();

        let source_node = *node_map.entry(source.clone()).or_insert_with(|| graph.add_node(source.clone()));
        let target_node = *node_map.entry(target.clone()).or_insert_with(|| graph.add_node(target.clone()));

        graph.add_edge(source_node, target_node, ());
    }

    Ok(graph)
}

/// Randomly selects a sample of characters
fn random_sample<'a, T: Clone>(items: &'a [T], sample_size: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    items.choose_multiple(&mut rng, sample_size).cloned().collect()
}

/// Calculates degrees of separation from a randomly chosen character to all others using BFS
fn six_degrees_to_all(graph: &CharacterGraph, start: &str) -> Result<HashMap<String, usize>, Box<dyn Error>> {
    let node_map: HashMap<&str, NodeIndex> = graph
        .node_indices()
        .map(|idx| (graph.node_weight(idx).unwrap().as_str(), idx))
        .collect();

    let start_node = *node_map.get(start).ok_or("Start character not found")?;

    let mut distances: HashMap<String, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back((start_node, 0));
    visited.insert(start_node, true);

    while let Some((current_node, current_distance)) = queue.pop_front() {
        let current_character = graph.node_weight(current_node).unwrap().clone();

        // shortest distance to current_character
        distances.insert(current_character, current_distance);

        for neighbor in graph.neighbors(current_node) {
            if !visited.contains_key(&neighbor) {
                queue.push_back((neighbor, current_distance + 1));
                visited.insert(neighbor, true);
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
    println!("Graph loaded successfully!");

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

    println!("Degrees of separation from '{}':", start_character);
    for (character, distance) in &distances {
        println!("{}: {} degrees", character, distance);
    }

    println!("Execution time: {} microseconds", elapsed_time);

    Ok(())
}