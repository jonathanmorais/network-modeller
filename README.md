# Network Traffic Modeling CLI

## Overview

This Rust CLI tool models network traffic flow and evaluates network utilization based on provided network topology and traffic demands. The project implements Dijkstra's algorithm for finding shortest paths in the network and simulates traffic to determine link utilizations.

## Features

- **Load Network:** Read network information from a CSV file, representing links between nodes with capacities and weights.

- **Shortest Path:** Implement Dijkstra's algorithm to find the shortest path between specified source and destination nodes.

- **Load Traffic Data:** Read traffic demands from a CSV file, specifying sources, destinations, and demands.

- **Traffic Modeling:** Simulate traffic flow in the network, routing demands along the shortest paths, and calculate link utilizations.

- **Worst Case Failure Analysis:** Determine the worst-case failure scenario by simulating link failures and analyzing network behavior.

## Usage

1. **Build the Project:**
    ```bash
    cargo build --release
    ```

2. **Run the CLI:**
    ```bash
    ./target/release/network_traffic_modeling input_network.csv input_traffic.csv output_report.csv
    ```

    Replace `input_network.csv`, `input_traffic.csv`, and `output_report.csv` with your actual file paths.

## Dependencies

- [csv](https://crates.io/crates/csv): A CSV parsing library for Rust.

## Contributing

Feel free to contribute to this project by opening issues or submitting pull requests.

## License

This project is licensed under the [MIT License](LICENSE).

