mod map;
mod robot;

use map::Map;
use rand::{Rng, SeedableRng};
use robot::{initialize_robots, Robot};
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default map size
    let default_width = 20;
    let default_height = 10;

    let width = args.get(1).and_then(|w| w.parse().ok()).unwrap_or(default_width);
    let height = args.get(2).and_then(|h| h.parse().ok()).unwrap_or(default_height);

    let seed = rand::thread_rng().gen_range(0..10000);
    let map = Map::new(seed, width, height);
    
    let count = 3;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let mut robots = initialize_robots(count, width, height, seed);

    println!("\nStarting simulation...\n");
    for _ in 0..10 {
        sleep(Duration::from_millis(500));
        for robot in &mut robots {
            robot.move_robot(&map, &mut rng);
        }
        map.display_with_robots(&robots);
    }
}