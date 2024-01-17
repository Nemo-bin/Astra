use crate::graphics::sprites::sprite_struct::Sprite;

pub fn test_box() -> Sprite {
    let bitmap = vec![vec![1; 8]; 8];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}