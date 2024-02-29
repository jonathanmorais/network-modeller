mod network;

use clap::{App, Arg};

use network::{load_network, load_traffic, model_traffic, worst_case_failure};
use std::error::Error;

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
    println!("Network loaded successfully.");

    // Load traffic data
    let traffic_demands = load_traffic(traffic_file_path)?;
    println!("Traffic data loaded successfully.");

    // Model traffic and generate utilization report
    model_traffic(&network, &traffic_demands)?;
    println!("Utilization report generated successfully.");

    // Determine Worst Case Failure and generate report
    worst_case_failure(&network, &traffic_demands)?;
    println!("Worst Case Failure report generated successfully.");

    Ok(())
}