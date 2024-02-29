use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io;
use std::iter::Enumerate;
use std::string::ToString;
use csv::{ReaderBuilder, WriterBuilder};

// Define Link struct
#[derive(Debug)]
pub struct Link {
    link_id: usize,
    start: String,
    end: String,
    capacity: usize,
    weight: usize,
}

// Define Network struct
#[derive(Debug)]
pub struct Network {
    links: Vec<Link>,
}

// Define TrafficDemand struct
#[derive(Debug)]
pub struct TrafficDemand {
    source: String,
    destination: String,
    demand: usize,
}

// Define a struct to represent a State for Dijkstra's algorithm
#[derive(Clone, Eq, PartialEq)]
pub struct State {
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

pub fn load_network(file_path: &str) -> Result<Network, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut links = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let link = Link {
            link_id: 1,
            start: record[0].to_string(),
            end: record[1].to_string(),
            capacity: record[2].parse()?, // Assuming capacity is the third column and contains integer values
            weight: record[3].parse()?,   // Assuming weight is the fourth column and contains integer values
        };
        links.push(link);
    }

    Ok(Network { links })
}

// Function to load traffic data from CSV
pub fn load_traffic(file_path: &str) -> Result<Vec<TrafficDemand>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut traffic_demands = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let source = record.get(0).ok_or("Missing source")?.trim().to_string();
        let destination = record.get(1).ok_or("Missing destination")?.trim().to_string();
        let demand_str = record.get(2).ok_or("Missing demand")?.trim();
        let demand = demand_str.parse::<usize>().map_err(|_| "Invalid demand")?;
        traffic_demands.push(TrafficDemand { source, destination, demand });
    }

    Ok(traffic_demands)
}
// Function to model traffic load on the network and produce a report
pub fn model_traffic(network: &Network, traffic_demands: &[TrafficDemand]) -> Result<(), Box<dyn Error>> {
    let mut link_utilization: HashMap<(String, String), usize> = HashMap::new();

    for demand in traffic_demands {
        for link in &network.links {
            if (link.start == demand.source) && (link.end == demand.destination) {
                let entry = link_utilization.entry((link.start.clone(), link.end.clone())).or_insert(0);
                *entry += demand.demand;
            }
        }
    }

    let mut utilization_report_writer = WriterBuilder::new().from_writer(File::create("utilization_report.csv")?);
    utilization_report_writer.write_record(&["Start Node", "End Node", "Utilization"])?;

    for ((start, end), utilization) in &link_utilization {
        utilization_report_writer.write_record(&[start, end, &utilization.to_string()])?;
    }

    Ok(())
}

pub fn dijkstra(network: &Network, source: &str, destination: &str) -> Option<Vec<usize>> {
    let mut heap = BinaryHeap::new();
    let mut visited: HashMap<String, usize> = HashMap::new();
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

        if cost > *distances.get(&node).unwrap_or(&usize::MAX) {
            // Skip this state if a shorter path to the node has already been found
            continue;
        }

        for (link_id, link) in network.links.iter().enumerate() {
            if link.start == node {
                let next_node = link.end.clone();
                let next_cost = cost + link.weight;

                if next_cost < *distances.get(&next_node).unwrap_or(&usize::MAX) {
                    distances.insert(next_node.clone(), next_cost);
                    visited.insert(next_node.clone(), link_id);
                    heap.push(State { node: next_node, cost: next_cost });
                }
            }
        }
    }

    None // No path found
}

// Function to determine Worst Case Failure (WCF)
pub fn worst_case_failure(network: &Network, traffic_demands: &[TrafficDemand]) -> Result<(), Box<dyn Error>> {
    let mut wcf_report_writer = WriterBuilder::new().from_writer(File::create("wcf_report.csv")?);

    for link in &network.links {
        let mut unreachable_nodes = Vec::new();
        unreachable_nodes.push(&link.start);
        unreachable_nodes.push(&link.end);

        for demand in traffic_demands {
            if unreachable_nodes.contains(&&demand.source) || unreachable_nodes.contains(&&demand.destination) {
                wcf_report_writer.write_record(&[&link.start, &link.end, &demand.source, &demand.destination])?;
            }
        }
    }

    Ok(())
}