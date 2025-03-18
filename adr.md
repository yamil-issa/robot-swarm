# Architecture Decision Record (ADR)

## Submitters
- Yamil ISSA

## Change Log
- [Approved](https://github.com/yamil-issa/robot-swarm/pull/1) 2025-03-18
- [Approved](https://github.com/yamil-issa/robot-swarm/pull/2) 2025-03-18

## Referenced Use Case(s)
- [Use Case Name](URL)

## Context
The project requires a structured approach to manage map generation, robots, and stations. Given the complexity of the simulation, decisions regarding architecture must be documented clearly. This ADR serves to justify and record architectural decisions made during development.

## Proposed Design
### Services/Modules Impacted
- `map.rs`: Handles map generation, including obstacles and resources
- `robot.rs`: Handles robot creation, movement, and interaction
- `main.rs`: Entry point of the program

### New Services/Modules Added
- Dynamic ASCII rendering in `map.rs`
- `robot.rs`: Manages robot logic and movement
- `station.rs` (future module for the station management)

### Model and DTO Impact
- Addition of `Tile` enum for different terrain types
- Addition of resources: `Energy`, `Mineral`, `Scientific`
- Addition of RobotType enum with different robot specializations:
    - Explorer 📡 (Explores the map)
    - Miner ⛏️ (Extracts minerals)
    - Scientist 🔬 (Analyzes scientific sites)
- Addition of Robot struct:
    - Stores position (x, y)
    - Holds id and robot_type
    - Implements movement logic with collision avoidance
- Dynamic Map Representation:
    - Robots are displayed as E, M, S directly on the ASCII map
    - Map updates dynamically as robots move
- Possible extension for handling resource collection

### API Impact
- No external API at this stage, only internal struct-based architecture

### General Configuration Impact
- Map generation is based on a **Perlin Noise** function for obstacles
- Uses a **random seed** to ensure reproducibility
- Map size is configurable via CLI arguments `cargo run width height`
- Number of robots is currently fixed at 3, but can be made configurable
- Robots now move randomly, avoiding obstacles
- Terminal clears each frame for real-time simulation
- Frame update interval set to 500ms to ensure smooth animation

### DevOps Impact
- No significant impact at this stage, as the project is in early development

## Considerations
- Alternative approaches like cellular automata for terrain generation were considered but discarded in favor of Perlin Noise for smoother transitions.
- Random resource placement was chosen with a 5% probability per resource type per tile.
- Future work could implement pathfinding (A or Dijkstra)* for smarter movement.
- Each robot moves one step at a time, but we may introduce different movement behaviors per robot type.
- Consideration for graphical rendering with crossterm or Bevy in the future.

## Decision
- Use **Perlin Noise** for procedural map generation
- Structure the project into **modular Rust files** (`map.rs`, `robot.rs`, `station.rs`)
- Render the map in **ASCII format** for simplicity
- Add energy, mineral, and scientific resources to the map
- Enable CLI-configurable map size
- Introduce robot movement system with basic pathfinding rules
- Ensure robots cannot move into obstacles
- Use a looped simulation with delays to animate movement
- Store robots in a Vec<Robot> for tracking movements
- Implement real-time map updates with robot positions
- Clear terminal between frames to simulate fluid motion

## References
- [Noise Crate](https://docs.rs/noise/latest/noise/) - Used for Perlin Noise generation
- [Rust Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) - Rust module system documentation
- [Rust CLI Arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html) - Handling command-line arguments in Rust