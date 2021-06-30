use rand::Rng;

use crate::map::{Map, StaticMap, MapTileType};

pub fn generate_random_map(width : usize, height : usize) -> StaticMap {
    let mut rng = rand::thread_rng();
    let mut map = StaticMap::new(width, height);
    for y in 0..height {
        for x in 0..width {
            if rng.gen_ratio(1, 10) {
                map.set_tile(x, y, MapTileType::WALL);
            }
        }
    }
    return map;
}