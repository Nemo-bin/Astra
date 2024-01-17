pub struct Map {
    pub bitmap: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(bitmap: Vec<Vec<u8>>) -> Self {
        Map {
            bitmap,
        }
    }
}