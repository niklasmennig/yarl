use rltk::FontCharType;

#[derive(Clone, PartialEq)]
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

impl rltk::BaseMap for StaticMap {
    fn is_opaque(&self, _idx: usize) -> bool {
        return self.tiles[_idx] != MapTileType::EMPTY;
    }

    fn get_available_exits(&self, _idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        rltk::SmallVec::new()
    }

    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        1.0
    }
}

impl rltk::Algorithm2D for StaticMap {
    fn point2d_to_index(&self, pt: rltk::Point) -> usize {
        return coords_to_index(pt.x as usize, pt.y as usize, self.width);
    }

    fn index_to_point2d(&self, idx: usize) -> rltk::Point {
        rltk::Point::new(idx % self.width, idx / self.width)
    }

    fn dimensions(&self) -> rltk::Point {
        return rltk::Point{x:self.width as i32, y:self.height as i32};
    }

    fn in_bounds(&self, pos: rltk::Point) -> bool {
        let bounds = self.dimensions();
        pos.x >= 0 && pos.x < bounds.x && pos.y >= 0 && pos.y < bounds.y
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