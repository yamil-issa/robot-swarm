use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
    style::{Print, SetForegroundColor, Color},
};
use std::io::{stdout, Write};
use crate::robot::Robot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Obstacle,
    Energy,      // ⚡ Energy Resource
    Mineral,     // ⛏️ Mineral Resource
    Scientific,  // 🔬 Scientific Point of Interest
    Station,    // 📡 Station
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

        grid[5][0] = Tile::Station;

        for y in 0..height {
            for x in 0..width {
                if x == 0 && y == 5 {
                    continue;
                }

                let noise_value = perlin.get([x as f64 / 5.0, y as f64 / 5.0]);

                if noise_value > 0.5 {
                    grid[y][x] = Tile::Obstacle;
                } else {
                    let resource_chance: f64 = rng.gen();
                    if resource_chance < 0.03 {
                        grid[y][x] = Tile::Energy;
                    } else if resource_chance < 0.06 {
                        grid[y][x] = Tile::Mineral;
                    } else if resource_chance < 0.09 {
                        grid[y][x] = Tile::Scientific;
                    }
                }
            }
        }

        Self { grid, width, height }
    }

    pub fn display_map(&self, robots: &[Robot]) {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                let (symbol, color) = match tile {
                    Tile::Empty => ('.', Color::White),
                    Tile::Obstacle => ('#', Color::DarkGrey),
                    Tile::Energy => ('⚡', Color::Yellow),
                    Tile::Mineral => ('⛏', Color::Green),
                    Tile::Scientific => ('🔬', Color::Cyan),
                    Tile::Station => ('📡', Color::Magenta),
                };

                execute!(stdout, MoveTo(x as u16 * 2, y as u16), SetForegroundColor(color), Print(symbol)).unwrap();
            }
        }

        for robot in robots {
            if self.grid[robot.y][robot.x] == Tile::Station {
                continue;
            }

            let symbol = match robot.robot_type {
                crate::robot::RobotType::Explorer => 'E',
                crate::robot::RobotType::Miner => 'M',
                crate::robot::RobotType::Scientist => 'S',
            };

            execute!(
                stdout,
                MoveTo(robot.x as u16 * 2, robot.y as u16),
                SetForegroundColor(Color::Blue),
                Print(symbol)
            )
            .unwrap();
        }

        stdout.flush().unwrap();
    }
}