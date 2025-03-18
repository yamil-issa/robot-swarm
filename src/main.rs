mod map;
use map::Map;
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default map size
    let default_width = 20;
    let default_height = 10;

    let width = args.get(1).and_then(|w| w.parse().ok()).unwrap_or(default_width);
    let height = args.get(2).and_then(|h| h.parse().ok()).unwrap_or(default_height);

    let seed = rand::thread_rng().gen_range(0..10000);
    let map = Map::new(seed, width, height);
    
    println!("Map generated with seed: {}", seed);
    println!("Size: {} x {}", width, height);
    map.display();
}
