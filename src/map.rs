use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::env;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Obstacle,
    Energy,      // ⚡ Energy Resource
    Mineral,     // ⛏️ Mineral Resource
    Scientific,  // 🔬 Scientific Point of Interest
}

pub struct Map {
    grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
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

    pub fn display(&self) {
        for row in &self.grid {
            for &tile in row {
                match tile {
                    Tile::Empty => print!(". "),
                    Tile::Obstacle => print!("# "),
                    Tile::Energy => print!("⚡ "),
                    Tile::Mineral => print!("⛏️ "),
                    Tile::Scientific => print!("🔬 "),
                }
            }
            println!();
        }
    }
}