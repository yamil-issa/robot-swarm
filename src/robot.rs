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
    pub energy: usize,
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
            energy: 10,
        }
    }

    pub fn display_info(&self) {
        println!(
            "Robot ({:?}) -> Position: ({}, {})",
            self.robot_type, self.x, self.y
        );
    }

    fn find_nearest(&self, map: &Map, target: Tile) -> Option<(usize, usize)> {
        let mut nearest = None;
        let mut min_dist = usize::MAX;

        for (y, row) in map.grid.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == target {
                    let dist = ((self.x as isize - x as isize).abs() + (self.y as isize - y as isize).abs()) as usize;
                    if dist < min_dist {
                        min_dist = dist;
                        nearest = Some((x, y));
                    }
                }
            }
        }
        nearest
    }

    pub fn move_robot(&mut self, map: &Map, rng: &mut StdRng) {
        if self.energy == 0 {
            return;
        }
    
        self.energy -= 1;
    
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    
        let target_tile = if self.energy <= 5 {
            // If energy is low, prioritize finding energy
            Tile::Energy
        } else {
            match self.robot_type {
                RobotType::Miner => Tile::Mineral,
                RobotType::Scientist => Tile::Scientific,
                RobotType::Explorer => Tile::Empty,
            }
        };
    
        if self.robot_type == RobotType::Explorer && self.energy > 5 {
            for _ in 0..4 {
                let (dx, dy) = directions[rng.gen_range(0..4)];
                let new_x = (self.x as isize + dx).max(0).min((map.width - 1) as isize) as usize;
                let new_y = (self.y as isize + dy).max(0).min((map.height - 1) as isize) as usize;
    
                if map.grid[new_y][new_x] != Tile::Obstacle {
                    self.x = new_x;
                    self.y = new_y;
                    return;
                }
            }
        }
    
        if let Some((target_x, target_y)) = self.find_nearest(map, target_tile) {
            let dx = (target_x as isize - self.x as isize).signum();
            let dy = (target_y as isize - self.y as isize).signum();
    
            let new_x = (self.x as isize + dx).max(0).min((map.width - 1) as isize) as usize;
            let new_y = (self.y as isize + dy).max(0).min((map.height - 1) as isize) as usize;
    
            if map.grid[new_y][new_x] != Tile::Obstacle {
                self.x = new_x;
                self.y = new_y;
                return;
            }
        }
    
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
        match map.grid[self.y][self.x] {
            Tile::Energy => {
                self.energy += 5;
                self.discoveries.push((self.x, self.y, Tile::Energy));
                map.grid[self.y][self.x] = Tile::Empty;
            }
            Tile::Mineral if self.robot_type == RobotType::Miner => {
                self.discoveries.push((self.x, self.y, Tile::Mineral));
                map.grid[self.y][self.x] = Tile::Empty;
            }
            Tile::Scientific if self.robot_type == RobotType::Scientist => {
                self.discoveries.push((self.x, self.y, Tile::Scientific));
                map.grid[self.y][self.x] = Tile::Empty;
            }
            tile if self.robot_type == RobotType::Explorer && tile != Tile::Empty && tile != Tile::Obstacle => {
                self.discoveries.push((self.x, self.y, tile));
                map.grid[self.y][self.x] = Tile::Empty;
            }
            _ => {}
        }
    }
}

pub fn initialize_robots(count: usize, width: usize, height: usize, seed: u32) -> Vec<Robot> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    
    let start_x = 0;
    let spacing = height / (count + 1);

    (0..count).map(|i| {
        let robot_type = match rng.gen_range(0..3) {
            0 => RobotType::Explorer,
            1 => RobotType::Miner,
            _ => RobotType::Scientist,
        };

        Robot {
            x: start_x,
            y: spacing * (i + 1),
            robot_type,
            discoveries: Vec::new(),
            energy: 10,
        }
    }).collect()
}
