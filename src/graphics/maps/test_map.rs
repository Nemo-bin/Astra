use crate::graphics::maps::map_struct::Map;

pub fn test_map() -> Map {
    let mut bitmap = vec![vec![2; 128]; 128];
    Map::new(bitmap)
}