# Architecture Decision Record (ADR)

## Submitters
- Yamil ISSA

## Change Log
- [Approved](https://github.com/yamil-issa/robot-swarm/pull/1) 2025-03-18
- [Approved](https://github.com/yamil-issa/robot-swarm/pull/2) 2025-03-18
- [Approved](https://github.com/yamil-issa/robot-swarm/pull/3) 2025-03-18
- [Approved](https://github.com/yamil-issa/robot-swarm/pull/4) 2025-03-31
- [Approved](https://github.com/yamil-issa/robot-swarm/pull/4) 2025-03-31
- [Approved](https://github.com/yamil-issa/robot-swarm/pull/4) 2025-03-31


## Context
The project requires a structured approach to manage map generation, robots, and stations. Given the complexity of the simulation, decisions regarding architecture must be documented clearly. This ADR serves to justify and record architectural decisions made during development.

## Proposed Design
### Services/Modules Impacted
- `main.rs`: Entry point of the program
- `map.rs`: Handles map generation, including obstacles and resources
- `robot.rs`: Handles robot creation, movement, and interaction
- `station.rs`: Handles resource collection, discovery tracking, and statistics reporting

### New Services/Modules Added
- Dynamic ASCII rendering in `map.rs`
- `robot.rs`: Manages robot logic and movement
- use `Crossterm` to render the map in `map.rs`
- `station.rs`: Manages resource collection and discovery tracking

### Model and DTO Impact
- Addition of `Tile` enum for different terrain types
- Addition of resources: `Energy`, `Mineral`, `Scientific`
- Addition of RobotType enum with different robot specializations:
    - Explorer 📡 (Explores the map)
    - Miner ⛏️ (Extracts minerals)
    - Scientist 🔬 (Analyzes scientific sites)
- Addition of Robot struct:
    - Stores position (x, y)
    - Holds robot_type
    - Tracks energy level and discoveries
    - Implements movement logic with collision avoidance
    - Handles resource collection and station return
- Addition of Station struct:
    - Manages resource collection
    - Tracks all discoveries
    - Provides discovery statistics
- Dynamic Map Representation:
    - Robots are displayed as E, M, S directly on the ASCII map
    - Map updates dynamically as robots move
    - Resources are displayed with colored symbols
    - Station is displayed at a fixed position

### API Impact
- No external API at this stage, only internal struct-based architecture
- Thread-safe implementation using Arc<Mutex<T>> for shared resources

### General Configuration Impact
- Map generation is based on a **Perlin Noise** function for obstacles
- Uses a **random seed** to ensure reproducibility
- Map size is configurable via CLI arguments `cargo run width height`
- Number of robots is fixed at 3 (one of each type)
- Robots have energy management and return to station when low
- Terminal clears each frame for real-time simulation
- Frame update interval set to 400ms for smooth animation
- Station is placed at the left center of the map
- Resource generation probability is 3% per type per tile

### DevOps Impact
- No significant impact at this stage, as the project is in early development
- Dependencies managed through Cargo.toml:
  - noise = "0.8" for Perlin Noise generation
  - rand = "0.8" for random number generation
  - crossterm = "0.26" for terminal rendering

## Considerations
- Alternative approaches like cellular automata for terrain generation were considered but discarded in favor of Perlin Noise for smoother transitions.
- Resource placement uses a 3% probability per resource type per tile for balanced distribution.
- Robots implement basic pathfinding towards resources and station.
- Each robot type has specialized behavior:
  - Explorers move randomly when energy is high
  - Miners and Scientists actively seek their respective resources
  - All robots return to station when energy is low
- Thread-safe implementation ensures safe concurrent access to shared resources

## Decision
- Use **Perlin Noise** for procedural map generation
- Structure the project into **modular Rust files** (`map.rs`, `robot.rs`, `station.rs`)
- Render the map in **ASCII format** with colored symbols
- Add energy, mineral, and scientific resources to the map
- Enable CLI-configurable map size
- Implement robot movement system with energy management
- Ensure robots cannot move into obstacles
- Use a looped simulation with delays to animate movement
- Store robots in a Vec<Robot> for tracking movements
- Implement real-time map updates with robot positions
- Clear terminal between frames to simulate fluid motion
- Use Crossterm for rendering the map and robot movements in the terminal
- Hide the cursor during simulation and restore it at the end
- Implement thread-safe resource sharing using Arc<Mutex<T>>
- Add station for resource collection and discovery tracking
- Display final statistics at the end of simulation

## References
- [Noise Crate](https://docs.rs/noise/latest/noise/) - Used for Perlin Noise generation
- [Rust Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) - Rust module system documentation
- [Rust CLI Arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html) - Handling command-line arguments in Rust
- [Crossterm](https://docs.rs/crossterm/latest/crossterm)- Library used for terminal-based graphical rendering