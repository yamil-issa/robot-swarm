use robot_swarm::robot::{Robot, RobotType};
use robot_swarm::map::{Map, Tile};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[test]
fn test_robot_creation() {
    let mut rng = StdRng::seed_from_u64(42);
    let robot = Robot::new(10, 10, &mut rng);
    
    assert!(robot.x < 10);
    assert!(robot.y < 10);
    assert!(robot.energy == 10);
    assert!(!robot.returning);
    assert!(robot.discoveries.is_empty());
}

#[test]
fn test_robot_movement() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut robot = Robot::new(10, 10, &mut rng);
    let map = Map::new(42, 10, 10);
    
    let initial_x = robot.x;
    let initial_y = robot.y;
    
    robot.move_robot(&map, &mut rng, 0, 5);
    
    // check if the robot has moved
    assert!(robot.x != initial_x || robot.y != initial_y);
    // check if the robot is still in the map
    assert!(robot.x < 10);
    assert!(robot.y < 10);
}

#[test]
fn test_robot_resource_collection() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut robot = Robot::new(10, 10, &mut rng);
    let mut map = Map::new(42, 10, 10);
    
    map.grid[robot.y][robot.x] = match robot.robot_type {
        RobotType::Miner => Tile::Mineral,
        RobotType::Scientist => Tile::Scientific,
        RobotType::Explorer => Tile::Energy,
    };
    
    robot.perform_action(&mut map);
    
    assert!(!robot.discoveries.is_empty());
    assert_eq!(map.grid[robot.y][robot.x], Tile::Empty);
}

#[test]
fn test_robot_type_specialization() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut map = Map::new(42, 10, 10);
    
    // test for each robot type
    let robot_types = [
        RobotType::Explorer,
        RobotType::Miner,
        RobotType::Scientist,
    ];
    
    for &robot_type in &robot_types {
        let mut robot = Robot::new(10, 10, &mut rng);
        robot.robot_type = robot_type;
        
        // place a resource at the robot's position
        map.grid[robot.y][robot.x] = match robot_type {
            RobotType::Miner => Tile::Mineral,
            RobotType::Scientist => Tile::Scientific,
            RobotType::Explorer => Tile::Energy,
        };
        
        robot.perform_action(&mut map);
        
        // check if the robot has collected a resource
        assert!(!robot.discoveries.is_empty());
        let (_, _, tile) = robot.discoveries[0];
        match robot_type {
            RobotType::Miner => assert_eq!(tile, Tile::Mineral),
            RobotType::Scientist => assert_eq!(tile, Tile::Scientific),
            RobotType::Explorer => assert_eq!(tile, Tile::Energy),
        }
    }
} 