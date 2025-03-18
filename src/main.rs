mod map;

use map::Map;
use rand::Rng;

fn main() {
    let seed = rand::thread_rng().gen_range(0..10000);
    let map = Map::new(seed);
    
    map.display();
}
