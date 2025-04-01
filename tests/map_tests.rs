use robot_swarm::map::{Map, Tile};

#[test]
fn test_map_creation_with_fixed_seed() {
    let seed = 42;
    let width = 10;
    let height = 10;
    let map1 = Map::new(seed, width, height);
    let map2 = Map::new(seed, width, height);
    
    assert_eq!(map1.grid, map2.grid);
}

#[test]
fn test_map_dimensions() {
    let width = 15;
    let height = 20;
    let map = Map::new(42, width, height);
    
    assert_eq!(map.width, width);
    assert_eq!(map.height, height);
    assert_eq!(map.grid.len(), height);
    assert_eq!(map.grid[0].len(), width);
}

#[test]
fn test_resource_distribution() {
    let map = Map::new(42, 100, 100);
    let mut energy_count = 0;
    let mut mineral_count = 0;
    let mut scientific_count = 0;

    for row in &map.grid {
        for &tile in row {
            match tile {
                Tile::Energy => energy_count += 1,
                Tile::Mineral => mineral_count += 1,
                Tile::Scientific => scientific_count += 1,
                _ => {}
            }
        }
    }

    // Check that the resources are distributed in a reasonable way
    let total_tiles = 100 * 100;
    let energy_ratio = energy_count as f64 / total_tiles as f64;
    let mineral_ratio = mineral_count as f64 / total_tiles as f64;
    let scientific_ratio = scientific_count as f64 / total_tiles as f64;

    assert!(energy_ratio < 0.05);
    assert!(mineral_ratio < 0.05);
    assert!(scientific_ratio < 0.05);
}

#[test]
fn test_obstacle_generation() {
    let map = Map::new(42, 100, 100);
    let mut obstacle_count = 0;

    for row in &map.grid {
        for &tile in row {
            if tile == Tile::Obstacle {
                obstacle_count += 1;
            }
        }
    }

    // Check that the obstacles are distributed in a reasonable way
    let total_tiles = 100 * 100;
    let obstacle_ratio = obstacle_count as f64 / total_tiles as f64;
    assert!(obstacle_ratio > 0.15 && obstacle_ratio < 0.5);
}

#[test]
fn test_tile_enum() {
    let empty = Tile::Empty;
    let obstacle = Tile::Obstacle;
    let energy = Tile::Energy;
    let mineral = Tile::Mineral;
    let scientific = Tile::Scientific;
    let station = Tile::Station;

    assert_eq!(empty, Tile::Empty);
    assert_ne!(empty, obstacle);

    let empty_copy = empty;
    assert_eq!(empty, empty_copy);

    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(empty);
    set.insert(obstacle);
    set.insert(energy);
    set.insert(mineral);
    set.insert(scientific);
    set.insert(station);
    assert_eq!(set.len(), 6);
} 