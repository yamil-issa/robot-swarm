use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::map::{Map, Tile};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RobotType {
    Explorer,   // 📡 Explore the map
    Miner,      // ⛏️ Extracts minerals
    Scientist,  // 🔬 Analyzes scientific sites
}

#[derive(Debug)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub robot_type: RobotType,
    pub discoveries: Vec<(usize, usize, Tile)>,
}

impl Robot {
    pub fn new(width: usize, height: usize, rng: &mut StdRng) -> Self {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        let robot_type = match rng.gen_range(0..3) {
            0 => RobotType::Explorer,
            1 => RobotType::Miner,
            _ => RobotType::Scientist,
        };

        Self {
            x,
            y,
            robot_type,
            discoveries: Vec::new(),
        }
    }

    pub fn display_info(&self) {
        println!(
            "Robot ({:?}) -> Position: ({}, {})",
            self.robot_type, self.x, self.y
        );
    }

    pub fn move_robot(&mut self, map: &Map, rng: &mut StdRng) {
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        for _ in 0..4 {
            let (dx, dy) = directions[rng.gen_range(0..4)];
            let new_x = (self.x as isize + dx).max(0).min((map.width - 1) as isize) as usize;
            let new_y = (self.y as isize + dy).max(0).min((map.height - 1) as isize) as usize;

            if map.grid[new_y][new_x] != Tile::Obstacle {
                self.x = new_x;
                self.y = new_y;
                break;
            }
        }
    }

    pub fn perform_action(&mut self, map: &mut Map) {
        match self.robot_type {
            RobotType::Explorer => {
                let tile = map.grid[self.y][self.x];
                if tile != Tile::Empty && tile != Tile::Obstacle {
                    self.discoveries.push((self.x, self.y, tile));
                }
            }
            RobotType::Miner => {
                if map.grid[self.y][self.x] == Tile::Mineral {
                    self.discoveries.push((self.x, self.y, Tile::Mineral));
                    map.grid[self.y][self.x] = Tile::Empty;
                }
            }
            RobotType::Scientist => {
                if map.grid[self.y][self.x] == Tile::Scientific {
                    self.discoveries.push((self.x, self.y, Tile::Scientific));
                }
            }
        }
    }
}

pub fn initialize_robots(count: usize, width: usize, height: usize, seed: u32) -> Vec<Robot> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    (0..count).map(|_| Robot::new(width, height, &mut rng)).collect()
}