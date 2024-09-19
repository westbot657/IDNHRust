use std::collections::HashMap;
use crate::shelf_tile::ShelfTile;

pub struct ObjectTree {
    pub shelf_tiles: HashMap<String, Vec<ShelfTile>>
}

impl ObjectTree {
    pub fn new() -> Self {


        Self {
            shelf_tiles: HashMap::new()
        }
    }

}
