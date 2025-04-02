# Robot Swarm Simulation 🤖

## Description
A Rust-based simulation of a robot swarm system where different types of robots explore, mine, and conduct scientific research in a procedurally generated environment.

## Features 🌟

### Map Generation 🗺️
- Procedurally generated using Perlin Noise
- Dynamic obstacles placement
- Resource distribution (Energy, Minerals, Scientific sites)
- Real-time ASCII visualization using Crossterm
- Configurable map dimensions via CLI arguments

### Robot Types 🚀
1. **Explorer** 📡
   - Specializes in map exploration
   - Collects energy resources
   - Random exploration pattern

2. **Miner** ⛏️
   - Focuses on mineral extraction
   - Efficient pathfinding to mineral resources
   - Resource collection capabilities

3. **Scientist** 🔬
   - Analyzes scientific sites
   - Strategic movement patterns
   - Data collection and analysis

### Station Management 🏠
- Central collection point for resources
- Resource tracking and statistics
- Energy replenishment for robots
- Discovery logging system

### Robot Behavior 🤖
- Energy management system
- Automatic return to station when energy is low
- Collision avoidance with obstacles
- Resource collection mechanics
- Specialized behavior based on robot type

### Technical Features 💻
- Thread-safe implementation using Arc<Mutex<T>>
- Real-time terminal rendering
- Configurable simulation parameters
- Comprehensive test coverage
- Modular architecture

## Requirements 📋

### System Requirements
- Rust 1.70 or higher
- Terminal with ANSI support
- Windows/Linux/MacOS compatible

### Dependencies
```toml
[dependencies]
noise = "0.8"      # For Perlin Noise generation
rand = "0.8"       # For random number generation
crossterm = "0.26" # For terminal rendering
```

## Installation 🔧

1. Clone the repository:
```bash
git clone https://github.com/your-username/robot-swarm.git
cd robot-swarm
```

2. Build the project:
```bash
cargo build --release
```

## Usage 🎮

### Running the Simulation
```bash
cargo run [width] [height]
```
Example:
```bash
cargo run 50 30
```

### Running Tests
```bash
cargo test
```

## Architecture 🏗️

The project follows a modular architecture with the following components:

- `main.rs`: Entry point and simulation loop
- `map.rs`: Map generation and rendering
- `robot.rs`: Robot behavior and movement logic
- `station.rs`: Resource management and statistics

## Testing 🧪

Comprehensive test suite including:
- Unit tests for robot behavior
- Integration tests for map generation
- Station functionality tests
- Resource management tests

## Performance Considerations 📊

- Efficient map generation using Perlin Noise
- Optimized pathfinding algorithms
- Thread-safe resource sharing
- Memory-efficient data structures

## License 📄

This project is licensed under the MIT License - see the LICENSE file for details.

## Author 👨‍💻

Yamil ISSA
