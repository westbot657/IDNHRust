use crate::app::App;
use crate::component::Component;
use crate::shelf_tile::ShelfTile;

pub struct ObjectTree {
    pub shelf_tiles: Vec<(String, ShelfTile)>
}

impl ObjectTree {
    pub fn new() -> Self {


        Self {
            shelf_tiles: Vec::new()
        }
    }

}

impl Component for ObjectTree {
    fn update(&mut self, app: &mut App) {

    }


    fn destroy(self) {
    }
}

