mod map;
mod robot;

use map::Map;
use rand::{Rng, SeedableRng};
use robot::initialize_robots;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
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
    let map = Arc::new(Mutex::new(Map::new(seed, width, height)));

    let count = 3;
    let robots = Arc::new(Mutex::new(initialize_robots(count, width, height, seed)));

    let mut stdout = stdout();
    execute!(stdout, Hide, Clear(ClearType::All)).unwrap();

    println!("\nStarting simulation...\n");

    for _ in 0..10 {
        sleep(Duration::from_millis(400));

        let mut handles = vec![];

        for i in 0..count {
            let map_clone = Arc::clone(&map);
            let robots_clone = Arc::clone(&robots);

            let handle = thread::spawn(move || {
                let mut rng = rand::rngs::StdRng::from_entropy();
                let mut robots_lock = robots_clone.lock().unwrap();
                let robot = &mut robots_lock[i];

                robot.move_robot(&map_clone.lock().unwrap(), &mut rng);
                robot.perform_action(&mut map_clone.lock().unwrap());
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let map_final = map.lock().unwrap();
        let robots_final = robots.lock().unwrap();
        map_final.display_map(&robots_final);
    }

    execute!(stdout, MoveTo(0, height as u16 + 2), Print("\nsimulation completed."), Show).unwrap();

    println!("\nRobots' discoveries:");
    let robots_final = robots.lock().unwrap();
    for (i, robot) in robots_final.iter().enumerate() {
        println!("Robot {} ({:?}) discoveries:", i + 1, robot.robot_type);
        for (x, y, tile) in &robot.discoveries {
            println!("  - Position: ({}, {}), Resource: {:?}", x, y, tile);
        }
    }

    stdout.flush().unwrap();

    sleep(Duration::from_secs(2));
}