extern crate find_folder;
extern crate roll_play_ge;
extern crate piston_window;

use piston_window::EventLoop;
use roll_play_ge::game;

// Edit this map with http://www.mapeditor.org/

fn main() {
    let asset_path = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("RollPlayGE Demo", [800, 600])
            .exit_on_esc(true)
            .opengl(piston_window::OpenGL::V3_2)
            .vsync(true)
            .build()
            .unwrap();
    window.set_max_fps(60);
    window.set_ups(60);
    window.set_ups_reset(1);
    window.set_swap_buffers(true);

    let mut game = game::Game::from_path(&asset_path, &mut window).unwrap();
    while game.next(&mut window) {}
}
