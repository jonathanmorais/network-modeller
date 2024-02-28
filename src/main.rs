// Import necessary libraries
use std::error::Error;
use std::fs::File;
use std::collections::{BinaryHeap, HashMap, HashSet};
use csv::{ReaderBuilder, WriterBuilder};
use clap::{App, Arg};

// Define Link struct
#[derive(Debug)]
struct Link {
    link_id: usize,
    start: String,
    end: String,
    capacity: usize,
    weight: usize,
}

// Define Network struct
#[derive(Debug)]
struct Network {
    links: Vec<Link>,
}

// Define TrafficDemand struct
#[derive(Debug)]
struct TrafficDemand {
    source: String,
    destination: String,
    demand: usize,
}

// Define a struct to represent a State for Dijkstra's algorithm
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: String,
    cost: usize,
}

// Implement Ord and PartialOrd traits for State to enable comparison in BinaryHeap
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Function to load network from CSV
fn load_network(file_path: &str) -> Result<Network, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut links = Vec::new();

    for (link_id, result) in rdr.records().enumerate() {
        let record = result?;
        let link = Link {
            link_id,
            start: record[1].to_string(),
            end: record[2].to_string(),
            capacity: record[3].parse()?,
            weight: record[4].parse()?,
        };
        links.push(link);
    }

    Ok(Network { links })
}

// Function to load traffic data from CSV
fn load_traffic(file_path: &str) -> Result<Vec<TrafficDemand>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut traffic_demands = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let demand = TrafficDemand {
            source: record[0].to_string(),
            destination: record[1].to_string(),
            demand: record[2].parse()?,
        };
        traffic_demands.push(demand);
    }

    Ok(traffic_demands)
}

// Function to implement Dijkstra's algorithm to find the shortest path
fn dijkstra(network: &Network, source: &str, destination: &str) -> Option<Vec<usize>> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();
    let mut distances = HashMap::new();

    // Initialize distances with infinity, except for the source node
    for link in &network.links {
        distances.insert(link.start.clone(), usize::MAX);
        distances.insert(link.end.clone(), usize::MAX);
    }
    distances.insert(source.to_string(), 0);

    heap.push(State { node: source.to_string(), cost: 0 });

    while let Some(State { node, cost }) = heap.pop() {
        if node == destination {
            // Destination reached, reconstruct the path
            let mut path = Vec::new();
            let mut current_node = destination.to_string();

            while let Some(link_id) = visited.get(&current_node) {
                path.push(*link_id);
                current_node = network.links[*link_id].start.clone();
            }

            path.reverse();
            return Some(path);
        }

        if cost > distances[&node] {
            // Skip this state if a shorter path to the node has already been found
            continue;
        }

        for (link_id, link) in network.links.iter().enumerate() {
            if link.start == node {
                let next_node = link.end.clone();
                let next_cost = cost + link.weight;

                if next_cost < distances[&next_node] {
                    distances.insert(next_node.clone(), next_cost);
                    visited.insert(next_node.clone(), link_id);
                    heap.push(State { node: next_node, cost: next_cost });
                }
            }
        }
    }

    None // No path found
}

// Function to model traffic load on the network and produce a report
fn model_traffic(network: &Network, traffic_demands: &[TrafficDemand]) -> Result<(), Box<dyn Error>> {
    let mut link_utilization: HashMap<usize, usize> = HashMap::new();

    for demand in traffic_demands {
        if let Some(path) = dijkstra(&network, &demand.source, &demand.destination) {
            for &link_id in &path {
                let entry = link_utilization.entry(link_id).or_insert(0);
                *entry += demand.demand;
            }
        } else {
            println!("Warning: No path found for traffic demand from {} to {}.", demand.source, demand.destination);
        }
    }

    let mut utilization_report_writer = WriterBuilder::new().from_writer(File::create("utilization_report.csv")?);
    utilization_report_writer.write_record(&["Link ID", "Utilization"])?;

    for (link_id, utilization) in &link_utilization {
        utilization_report_writer.write_record(&[link_id.to_string(), utilization.to_string()])?;
    }

    println!("Utilization report generated successfully.");
    Ok(())
}

// Function to determine Worst Case Failure (WCF)
fn worst_case_failure(network: &Network, traffic_demands: &[TrafficDemand]) -> Result<(), Box<dyn Error>> {
    let mut wcf_report_writer = WriterBuilder::new().from_writer(File::create("wcf_report.csv")?);

    for (link_id, link) in network.links.iter().enumerate() {
        let mut unreachable_nodes = HashSet::new();
        unreachable_nodes.insert(link.start.clone());
        unreachable_nodes.insert(link.end.clone());

        for demand in traffic_demands {
            if unreachable_nodes.contains(&demand.source) || unreachable_nodes.contains(&demand.destination) {
                wcf_report_writer.write_record(&[link_id.to_string(), demand.source.clone(), demand.destination.clone()])?;
            }
        }
    }

    println!("Worst Case Failure report generated successfully.");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Define CLI arguments
    let matches = App::new("Network Traffic Modeling CLI")
        .arg(Arg::with_name("network")
            .required(true)
            .takes_value(true)
            .help("Path to the network CSV file"))
        .arg(Arg::with_name("traffic")
            .required(true)
            .takes_value(true)
            .help("Path to the traffic CSV file"))
        .get_matches();

    // Get values from command-line arguments
    let network_file_path = matches.value_of("network").unwrap();
    let traffic_file_path = matches.value_of("traffic").unwrap();

    // Load network
    let network = load_network(network_file_path)?;

    // Load traffic data
    let traffic_demands = load_traffic(traffic_file_path)?;

    // Model traffic and generate utilization report
    model_traffic(&network, &traffic_demands)?;

    // Determine Worst Case Failure and generate report
    worst_case_failure(&network, &traffic_demands)?;

    Ok(())
}
