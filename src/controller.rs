use std;
use input;
use piston_window;
use piston_window::{TouchEvent, ReleaseEvent, PressEvent, UpdateEvent, Button, keyboard};
use std::iter::FromIterator;

fn default_keybindings() -> std::collections::BTreeMap<keyboard::Key, input::Command> {
    std::collections::BTreeMap::from_iter(
        vec![
            (keyboard::Key::W, input::Command::Up),
            (keyboard::Key::A, input::Command::Up),
            (keyboard::Key::S, input::Command::Down),
            (keyboard::Key::D, input::Command::Right),
            (keyboard::Key::Space, input::Command::Attack),
            (keyboard::Key::K, input::Command::Defend),
            (keyboard::Key::Escape, input::Command::Menu),
        ].into_iter(),
    )
}

pub struct Controller {
    /// Controller input, e.g. walking direction.
    pub input: input::Input,

    /// Bindings between keyboard keys and input commands.
    pub keybinding: std::collections::BTreeMap<keyboard::Key, input::Command>,

    /// Walk-Rate multiplier. Normal ~= 90.0.
    pub walk_rate: f32,

    /// Time since last game cycle.
    pub dt_s: f64,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            input: input::Input::new(),
            keybinding: default_keybindings(),
            walk_rate: 90.0,
            dt_s: 0.0,
        }
    }

    pub fn process_event(&mut self, event: &piston_window::Event) {
        if let Some(update_args) = event.update_args() {
            self.dt_s = update_args.dt;
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            if let Some(ref cmd) = self.keybinding.get(&key) {
                // A key that maps to an input command was pressed.
                self.input.set_command(&cmd, true);
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            if let Some(ref cmd) = self.keybinding.get(&key) {
                // A key that maps to an input command was released.
                self.input.set_command(&cmd, false);
            }
        }

        if let Some(touch) = event.touch_args() {
            // http://docs.piston.rs/piston_window/input/struct.TouchArgs.html
            // TODO When on a mobile platform, display touch interface and
            // use this event to register "button" presses.
            println!("Touch occurred '{:?}'", touch);
        }
    }
}
