extern crate find_folder;
extern crate roll_play_ge;
extern crate piston_window;

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


    let mut game = game::Game::from_path(&asset_path, &mut window).unwrap();
    while game.next(&mut window) {}
}
