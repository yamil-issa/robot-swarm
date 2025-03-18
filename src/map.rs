use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::robot::Robot;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Obstacle,
    Energy,      // ⚡ Energy Resource
    Mineral,     // ⛏️ Mineral Resource
    Scientific,  // 🔬 Scientific Point of Interest
}

pub struct Map {
    pub grid: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(seed: u32, width: usize, height: usize) -> Self {
        let perlin = Perlin::new(seed);
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let mut grid = vec![vec![Tile::Empty; width]; height];

        for y in 0..height {
            for x in 0..width {
                let noise_value = perlin.get([x as f64 / 5.0, y as f64 / 5.0]);

                if noise_value > 0.3 {
                    grid[y][x] = Tile::Obstacle;
                } else {
                    // 5% chance for each resource type
                    let resource_chance: f64 = rng.gen();
                    if resource_chance < 0.05 {
                        grid[y][x] = Tile::Energy;
                    } else if resource_chance < 0.10 {
                        grid[y][x] = Tile::Mineral;
                    } else if resource_chance < 0.15 {
                        grid[y][x] = Tile::Scientific;
                    }
                }
            }
        }

        Self { grid, width, height }
    }

    pub fn display_with_robots(&self, robots: &[Robot]) {
        print!("\x1B[2J\x1B[1;1H");
    
        let mut display_grid: Vec<Vec<char>> = self.grid.iter()
            .map(|row| row.iter().map(|&tile| match tile {
                Tile::Empty => '.',
                Tile::Obstacle => '#',
                Tile::Energy => '⚡',
                Tile::Mineral => '⛏',
                Tile::Scientific => '🔬',
            }).collect())
            .collect();
    
        for robot in robots {
            let symbol = match robot.robot_type {
                crate::robot::RobotType::Explorer => 'E',  // Explorer
                crate::robot::RobotType::Miner => 'M',     // Miner
                crate::robot::RobotType::Scientist => 'S', // Scientist
            };
            display_grid[robot.y][robot.x] = symbol;
        }
    
        for row in display_grid.iter() {
            for &tile in row {
                print!("{} ", tile);
            }
            println!();
        }
    }    
}