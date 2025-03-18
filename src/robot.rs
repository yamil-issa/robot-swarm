use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RobotType {
    Explorer,   // 📡 Explore the map
    Miner,      // ⛏️ Extracts minerals
    Scientist,  // 🔬 Analyzes scientific sites
}

#[derive(Debug)]
pub struct Robot {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub robot_type: RobotType,
}

impl Robot {
    pub fn new(id: usize, width: usize, height: usize, rng: &mut StdRng) -> Self {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let robot_type = match rng.gen_range(0..3) {
            0 => RobotType::Explorer,
            1 => RobotType::Miner,
            _ => RobotType::Scientist,
        };

        Self { id, x, y, robot_type }
    }

    pub fn display_info(&self) {
        println!(
            "Robot {} ({:?}) -> Position: ({}, {})",
            self.id, self.robot_type, self.x, self.y
        );
    }
}

pub fn initialize_robots(count: usize, width: usize, height: usize, seed: u32) -> Vec<Robot> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    (0..count).map(|i| Robot::new(i, width, height, &mut rng)).collect()
}