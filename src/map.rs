use noise::{NoiseFn, Perlin};
use rand::Rng;

const THRESHOLD: f64 = 0.3;

const WIDTH: usize = 20;
const HEIGHT: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Obstacle,
}

pub struct Map {
    grid: [[Tile; WIDTH]; HEIGHT],
}

impl Map {
    pub fn new(seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        let mut grid = [[Tile::Empty; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let noise_value = perlin.get([x as f64 / 5.0, y as f64 / 5.0]);
                if noise_value > THRESHOLD {
                    grid[y][x] = Tile::Obstacle;
                }
            }
        }

        Self { grid }
    }

    pub fn display(&self) {
        for row in &self.grid {
            for &tile in row {
                match tile {
                    Tile::Empty => print!(". "),
                    Tile::Obstacle => print!("# "),
                }
            }
            println!();
        }
    }
}
