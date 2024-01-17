pub struct Sprite {
    pub bitmap: Vec<Vec<u8>>,
    pub width: u8,
    pub height: u8,
}

impl Sprite {
    pub fn new(bitmap: Vec<Vec<u8>>, width: u8, height: u8) -> Self {
        Sprite {
            bitmap,
            width,
            height,
        }
    }
}