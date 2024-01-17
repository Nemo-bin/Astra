use crate::graphics::maps::map_struct::Map;

pub fn menu_map() -> Map {
    let mut bitmap = vec![vec![0; 128]; 128];
    Map::new(bitmap)
}