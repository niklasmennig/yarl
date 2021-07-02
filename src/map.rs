use std::usize;

use rltk::FontCharType;

#[derive(Clone, PartialEq, Copy)]
pub enum MapTileType {
    EMPTY,
    WALL
}

pub fn map_glyph(t : MapTileType) -> FontCharType {
    match t {
        MapTileType::EMPTY => rltk::to_cp437('.'),
        MapTileType::WALL => rltk::to_cp437('#')
    }
}

pub trait Map {
    fn get_tile(&self, x: i32, y: i32) -> MapTileType;
    fn set_tile(&mut self, x : i32, y : i32, v : MapTileType);
    fn get_size(&self) -> (i32, i32);
}

fn coords_to_index(x : i32, y : i32, width : i32) -> usize {
    return (x + y * width) as usize;
}

pub struct StaticMap {
    tiles : Vec<MapTileType>,
    width : i32,
    height : i32
}

impl StaticMap {
    pub fn new(width : i32, height : i32) -> StaticMap {
        StaticMap{
            width : width,
            height : height,
            tiles : vec![MapTileType::EMPTY; (width * height) as usize]
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
        return coords_to_index(pt.x, pt.y, self.width) as usize;
    }

    fn index_to_point2d(&self, idx: usize) -> rltk::Point {
        rltk::Point::new(idx % self.width as usize, idx / self.width as usize)
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
    fn get_tile(&self, x: i32, y: i32) -> MapTileType {
        return self.tiles.get(coords_to_index(x, y, self.width)).expect("accessed tile in StaticMap that is out of Bounds").clone();
    }

    fn set_tile(&mut self, x : i32, y : i32, v : MapTileType) {
        self.tiles[coords_to_index(x, y, self.width)] = v;
    }

    fn get_size(&self) -> (i32, i32) {
        return (self.width, self.height);
    }
}