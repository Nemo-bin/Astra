use crate::graphics::sprites::sprite_struct::Sprite;

// Fonts are to be matched with ASCII encoding. Not all ASCII characters may end up implemented. 
// https://urbanfonts-files.s3.amazonaws.com/samples/38151/26a39b2389aade80159efc5205c055ef.jpg

pub fn Big_A() -> Sprite {
    let bitmap = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}