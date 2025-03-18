mod map;
mod robot;

use map::Map;
use rand::{Rng, SeedableRng};
use robot::initialize_robots;
use std::env;
use std::thread::sleep;
use std::time::Duration;
use crossterm::{
    execute,
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType},
    style::Print,
};
use std::io::{stdout, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_width = 20;
    let default_height = 10;
    let width = args.get(1).and_then(|w| w.parse().ok()).unwrap_or(default_width);
    let height = args.get(2).and_then(|h| h.parse().ok()).unwrap_or(default_height);

    let seed = rand::thread_rng().gen_range(0..10000);
    let mut map = Map::new(seed, width, height);

    let count = 3;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let mut robots = initialize_robots(count, width, height, seed);

    let mut stdout = stdout();
    execute!(stdout, Hide, Clear(ClearType::All)).unwrap();

    println!("\nStarting simulation...\n");

    for _ in 0..10 {
        sleep(Duration::from_millis(400));
        for robot in &mut robots {
            robot.move_robot(&map, &mut rng);
        }
        map.display_map(&robots);
    }

    execute!(stdout, MoveTo(0, height as u16 + 2), Print("\nsimulation completed."), Show).unwrap();
    
    stdout.flush().unwrap();

    sleep(Duration::from_secs(2));
}