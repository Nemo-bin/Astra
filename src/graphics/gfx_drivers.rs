use sdl2::{
    pixels::PixelFormatEnum,
    render::{
        Canvas,
        Texture
    },
    video::Window,
    event::Event,
    EventPump,
};

use crate::graphics::sprites::sprite_struct::Sprite;
use crate::graphics::maps::map_struct::Map;
use crate::entities::*;

pub struct SDLRenderer {
    width: u32,
    height: u32,
    canvas: Canvas<Window>,
    texture: Texture,
    pub displaybuffer: Vec<u8>,
    pub event_pump: EventPump,
}

impl SDLRenderer {
    const PIXELSIZE:usize = 4;

    pub fn new(width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().expect("failed to create sdl context");

        let event_pump = sdl_context.event_pump().expect("failed to create event pump");

        let video_subsystem = sdl_context.video().expect("failed to get video context");

        let window = video_subsystem.window("Astral", 512, 512)
        .build()
        .expect("failed to build window");
    
        let mut canvas: Canvas<Window> = window.into_canvas()
        .build()
        .expect("failed to build window's canvas");

        let texture_creator = canvas.texture_creator();
        let t = texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB888,
            width,
            height,
        );
        let texture = if t.is_ok() { t.unwrap() } else { panic!("failed to create texture") };

        SDLRenderer{
            width,
            height,
            canvas,
            texture,
            displaybuffer: vec![0; width as usize * height as usize * Self::PIXELSIZE],
            event_pump,
        }
    }

    fn update(&mut self) {
        self.texture
            .update(None, &self.displaybuffer, self.width as usize * Self::PIXELSIZE);
        self.canvas
            .copy(&self.texture, None, None);
        self.canvas.present();
    }
    
    fn get_rgb(id: &u8) -> (u8, u8, u8) {
        let (r, g, b) = match id { // The rgb values are little endian... for some reason
            1 => (255, 255, 255),
            2 => (0, 0, 255),
            3 => (0, 255, 0),
            4 => (255, 0, 0),
            _ => unreachable!(),
        };
        (r, g, b)
    }

    pub fn blit(&mut self, entity: &Entity) {
        let mut internal_y = 0;
        for row_index in 0..entity.sprite.height {
            let row = &entity.sprite.bitmap[row_index as usize];
            let mut internal_x = 0;

            for pixel in row.iter() {
                if *pixel != 0 {
                    let (r, g, b) = Self::get_rgb(pixel);
                    let mut y_offset = ((entity.y.wrapping_add(internal_y) & !128) as usize * 128 * Self::PIXELSIZE);
                    let mut x_offset = ((entity.x.wrapping_add(internal_x) & !128) as usize * Self::PIXELSIZE);
                    /*if y_offset > 127 * 128 * Self::PIXELSIZE{
                        y_offset = 0;
                    }
                    if x_offset > 127 * Self::PIXELSIZE {
                        x_offset = 0;
                    }*/
                    self.displaybuffer[y_offset.wrapping_add(x_offset)] = r;
                    self.displaybuffer[y_offset.wrapping_add(x_offset + 1)] = g;
                    self.displaybuffer[y_offset.wrapping_add(x_offset + 2)] = b;
                }
                internal_x += 1;
            }
            internal_y += 1;
        }
    }

    pub fn render_map(&mut self, map: Map) {
        let mut internal_y = 0;
        for row in map.bitmap.iter() {
            let mut internal_x = 0;
            for pixel in row.iter() {
                let (r, g, b) = Self::get_rgb(pixel);
                let mut y_offset = internal_y as usize *  128 * Self::PIXELSIZE;
                let mut x_offset = internal_x as usize * Self::PIXELSIZE;
                /*if y_offset > 127 * 128 * Self::PIXELSIZE {
                    y_offset = 0;
                }
                if x_offset > 127 * Self::PIXELSIZE {
                    x_offset = 0;
                }*/
                self.displaybuffer[y_offset.wrapping_add(x_offset)] = r;
                self.displaybuffer[y_offset.wrapping_add(x_offset + 1)] = g;
                self.displaybuffer[y_offset.wrapping_add(x_offset + 2)] = b;
                internal_x += 1;
            }
            internal_y += 1;
        }
    }

    pub fn render_frame(&mut self, map: Map, entity_buffer: Vec<&Entity>) {
        self.render_map(map);
        for entity in entity_buffer.iter() {
            self.blit(entity);
        }
        self.update();
    }
}