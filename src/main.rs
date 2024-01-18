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
    let mut player = Entity::new(120, 120, test_box());
    
    let logo = Entity::new(0, 114, logo());
    let big_a = Entity::new(0, 0, big_A());
    let big_b = Entity::new(7, 0, big_B());
    let big_c = Entity::new(15, 0, big_C());
    let big_d = Entity::new(23, 0, big_D());
    let big_e = Entity::new(31, 0, big_E());
    let big_f = Entity::new(39, 0, big_F());
    let big_g = Entity::new(47, 0, big_G());
    let big_h = Entity::new(55, 0, big_H());
    let big_i = Entity::new(63, 0, big_I());
    let big_j = Entity::new(68, 0, big_J());
    let big_k = Entity::new(76, 0, big_K());
    let big_l = Entity::new(84, 0, big_L());
    let big_m = Entity::new(92, 0, big_M());
    let big_n = Entity::new(100, 0, big_N());
    let big_o = Entity::new(108, 0, big_O());
    let big_p = Entity::new(116, 0, big_P());
    let big_q = Entity::new(0, 8, big_Q());
    let big_r = Entity::new(8, 8, big_R());
    let big_s = Entity::new(15, 8, big_S());
    let big_t = Entity::new(22, 8, big_T());
    let big_u = Entity::new(29, 8, big_U());
    let big_v = Entity::new(37, 8, big_V());
    let big_w = Entity::new(45, 8, big_W());
    let big_x = Entity::new(53, 8, big_X());
    let big_y = Entity::new(60, 8, big_Y());
    let big_z = Entity::new(69, 8, big_Z());

    while game.running {
        sdl_renderer.render_frame(test_map(), vec![&player, &logo, 
        &big_a, &big_b, &big_c, &big_d, &big_e, &big_f, &big_g, &big_h, &big_i, &big_j, &big_k, &big_l, &big_m,
        &big_n, &big_o, &big_p, &big_q, &big_r, &big_s, &big_t, &big_u, &big_v, &big_w, &big_x, &big_y, &big_z,
        ]);

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
