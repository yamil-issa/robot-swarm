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
- `main.rs`: Entry point of the program

### New Services/Modules Added
- `robot.rs` (future module for robot handling)
- `station.rs` (future module for the station management)

### Model and DTO Impact
- Addition of `Tile` enum for different terrain types
- Addition of resources: `Energy`, `Mineral`, `Scientific`
- Possible extension for handling resource collection

### API Impact
- No external API at this stage, only internal struct-based architecture

### General Configuration Impact
- Map generation is based on a **Perlin Noise** function for obstacles
- Uses a **random seed** to ensure reproducibility
- Map size is configurable via CLI arguments `cargo run width height`

### DevOps Impact
- No significant impact at this stage, as the project is in early development

## Considerations
- Alternative approaches like cellular automata for terrain generation were considered but discarded in favor of Perlin Noise for smoother transitions.
- Random resource placement was chosen with a 5% probability per resource type per tile.

## Decision
- Use **Perlin Noise** for procedural map generation
- Structure the project into **modular Rust files** (`map.rs`, `robot.rs`, `station.rs`)
- Render the map in **ASCII format** for simplicity
- Add energy, mineral, and scientific resources to the map
- Enable CLI-configurable map size

## Other Related ADRs
- [ADR-002: Procedural Map Generation with Perlin Noise](URL) - Defines how the map is generated
- [ADR-003: ASCII Map Rendering](URL) - Defines how the map is displayed

## References
- [Noise Crate](https://docs.rs/noise/latest/noise/) - Used for Perlin Noise generation
- [Rust Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) - Rust module system documentation
- [Rust CLI Arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html) - Handling command-line arguments in Rust