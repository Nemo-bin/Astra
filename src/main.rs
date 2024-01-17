use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use Astral::graphics::gfx_drivers::*;
use Astral::graphics::sprites::test_box::*;
use Astral::graphics::sprites::font::*;
use Astral::graphics::maps::test_map::*;
use Astral::entities::*;
use Astral::game::*;

fn main() {
    let mut sdl_renderer = SDLRenderer::new(128, 128);
    let mut game = Game::new();
    let mut player = Entity::new(0, 0, test_box());

    while game.running {
        sdl_renderer.render_frame(test_map(), vec![&player]);

        for event in sdl_renderer.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    game.running = false;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    player.move_entity(Direction::Up);
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    player.move_entity(Direction::Left);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    player.move_entity(Direction::Down);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    player.move_entity(Direction::Right);
                },
                _ => {}
            }
        }
    }
}
