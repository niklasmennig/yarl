use rltk::FontCharType;

#[derive(Clone)]
pub enum MapTileType {
    EMPTY,
    WALL
}

pub fn map_glyph(t : MapTileType) -> Option<FontCharType> {
    match t {
        MapTileType::EMPTY => None,
        MapTileType::WALL => Some(rltk::to_cp437('#'))
    }
}

pub trait Map {
    fn get_tile(&self, x: usize, y: usize) -> MapTileType;
    fn set_tile(&mut self, x : usize, y : usize, v : MapTileType);
    fn get_size(&self) -> (usize, usize);
}

fn coords_to_index(x : usize, y : usize, width : usize) -> usize {
    return x + y * width;
}

pub struct StaticMap {
    tiles : Vec<MapTileType>,
    width : usize,
    height : usize
}

impl StaticMap {
    pub fn new(width : usize, height : usize) -> StaticMap {
        StaticMap{
            width : width,
            height : height,
            tiles : vec![MapTileType::EMPTY; width * height]
        }
    }
}

impl Map for StaticMap {
    fn get_tile(&self, x: usize, y: usize) -> MapTileType {
        return self.tiles.get(coords_to_index(x, y, self.width)).expect("accessed tile in StaticMap that is out of Bounds").clone();
    }

    fn set_tile(&mut self, x : usize, y : usize, v : MapTileType) {
        self.tiles[coords_to_index(x, y, self.width)] = v;
    }

    fn get_size(&self) -> (usize, usize) {
        return (self.width, self.height);
    }
}