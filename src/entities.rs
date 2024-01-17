use crate::graphics::sprites::sprite_struct::Sprite;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Entity {
    pub x: u8,
    pub y: u8,
    pub sprite: Sprite,
}

impl Entity {
    pub fn new(x: u8, y: u8, sprite: Sprite) -> Self {
        Entity {
            x,
            y,
            sprite,
        }
    }

    pub fn move_entity(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {
                self.y = self.y.wrapping_sub(1);
            },
            Direction::Down => {
                self.y = self.y.wrapping_add(1);
            },
            Direction::Left => {
                self.x = self.x.wrapping_sub(1);
            },
            Direction::Right => {
                self.x = self.x.wrapping_add(1);
            },
        }
    }
}