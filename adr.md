# Architecture Decision Record (ADR)

## Submitters
- Yamil ISSA

## Change Log
- [Approved](URL of pull request) 2025-03-18

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
- Possible extension for handling resource collection

### API Impact
- No external API at this stage, only internal struct-based architecture

### General Configuration Impact
- Map generation is based on a **Perlin Noise** function for obstacles
- Uses a **random seed** to ensure reproducibility

### DevOps Impact
- No significant impact at this stage, as the project is in early development

## Considerations
- Alternative approaches like cellular automata for terrain generation were considered but discarded in favor of Perlin Noise for smoother transitions.
- Future extensibility into an ECS-based architecture (Bevy) was considered but postponed.

## Decision
- Use **Perlin Noise** for procedural map generation
- Structure the project into **modular Rust files** (`map.rs`, `robot.rs`, `station.rs`)
- Render the map in **ASCII format** for simplicity

## Other Related ADRs
- [ADR-002: Procedural Map Generation with Perlin Noise](URL) - Defines how the map is generated
- [ADR-003: ASCII Map Rendering](URL) - Defines how the map is displayed

## References
- [Noise Crate](https://docs.rs/noise/latest/noise/) - Used for Perlin Noise generation
- [Rust Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) - Rust module system documentation