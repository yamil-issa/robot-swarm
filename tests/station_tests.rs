use robot_swarm::station::Station;
use robot_swarm::robot::{Robot, RobotType};
use robot_swarm::map::Tile;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[test]
fn test_station_creation() {
    let station = Station::new(0, 5);
    
    assert_eq!(station.x, 0);
    assert_eq!(station.y, 5);
    assert!(station.discoveries.is_empty());
}

#[test]
fn test_station_collect_discoveries() {
    let mut station = Station::new(0, 5);
    let mut rng = StdRng::seed_from_u64(42);
    let mut robot = Robot::new(10, 10, &mut rng);
    
    robot.discoveries.push((1, 2, Tile::Energy));
    robot.discoveries.push((3, 4, Tile::Mineral));
    
    station.collect_discoveries(&mut robot);
    
    assert_eq!(station.discoveries.len(), 2);
    assert!(robot.discoveries.is_empty());
    
    assert!(station.discoveries.contains(&(1, 2, Tile::Energy)));
    assert!(station.discoveries.contains(&(3, 4, Tile::Mineral)));
}

#[test]
fn test_station_discovery_tracking() {
    let mut station = Station::new(0, 5);
    
    station.discoveries.insert((1, 1, Tile::Energy));
    station.discoveries.insert((2, 2, Tile::Energy));
    station.discoveries.insert((3, 3, Tile::Mineral));
    station.discoveries.insert((4, 4, Tile::Scientific));
    
    let mut energy_count = 0;
    let mut mineral_count = 0;
    let mut scientific_count = 0;
    
    for &(_, _, tile) in &station.discoveries {
        match tile {
            Tile::Energy => energy_count += 1,
            Tile::Mineral => mineral_count += 1,
            Tile::Scientific => scientific_count += 1,
            _ => {}
        }
    }
    
    assert_eq!(energy_count, 2);
    assert_eq!(mineral_count, 1);
    assert_eq!(scientific_count, 1);
}

#[test]
fn test_station_unique_discoveries() {
    let mut station = Station::new(0, 5);
    
    // add the same discovery multiple times
    station.discoveries.insert((1, 1, Tile::Energy));
    station.discoveries.insert((1, 1, Tile::Energy));
    station.discoveries.insert((1, 1, Tile::Energy));
    
    // check if the duplicates are removed
    assert_eq!(station.discoveries.len(), 1);
}

#[test]
fn test_station_display_format() {
    let mut station = Station::new(0, 5);
    
    // add some discoveries
    station.discoveries.insert((1, 1, Tile::Energy));
    station.discoveries.insert((2, 2, Tile::Mineral));
    station.discoveries.insert((3, 3, Tile::Scientific));
    
    // check if the display format is correct
    let output = format!("{:?}", station);
    assert!(output.contains("Energy"));
    assert!(output.contains("Mineral"));
    assert!(output.contains("Scientific"));
} 