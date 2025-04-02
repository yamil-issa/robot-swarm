use crate::robot::Robot;
use crate::map::Tile;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Station {
    pub x: usize,
    pub y: usize,
    pub discoveries: HashSet<(usize, usize, Tile)>,
}

impl Station {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            discoveries: HashSet::new(),
        }
    }

    pub fn collect_discoveries(&mut self, robot: &mut Robot) {
        if !robot.discoveries.is_empty() {
            println!("Robot at station collected: {:?}", robot.discoveries);
        }
        for discovery in &robot.discoveries {
            self.discoveries.insert(*discovery);
        }
        robot.discoveries.clear();
    }

    pub fn display_discoveries(&self) {
        use std::collections::HashMap;
    
        let mut counts: HashMap<Tile, usize> = HashMap::new();
    
        for &(_, _, tile) in &self.discoveries {
            *counts.entry(tile).or_insert(0) += 1;
        }
    
        println!("\n📡 Station discoveries:");
        println!("--------------------------");
    
        for (tile, count) in &counts {
            let label = match tile {
                Tile::Energy => "⚡ Energy",
                Tile::Mineral => "⛏️ Minerals",
                Tile::Scientific => "🔬 Scientific sites",
                Tile::Station => "📡 Station",
                Tile::Obstacle => "🧱 Obstacles",
                Tile::Empty => "⬜ Empty",
            };
            println!(" - {}: {}", label, count);
        }
    
        println!("\n📍 All discovery positions:");
        for (x, y, tile) in &self.discoveries {
            println!("   ({}, {}) → {:?}", x, y, tile);
        }
    }    
}