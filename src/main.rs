// imports
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
mod player;
mod entity;
mod game;
mod inputHandler;

// shorthanding
pub use game::*;
use piston::event_loop::Events;
use piston::EventSettings;
use crate::piston::input::{RenderEvent, UpdateEvent, ButtonEvent};

fn main() {
    // Create a Glutin window.
    let mut window = game::new_window();

    // Create a new game and run it.
    let mut Game = Game::new();

    // create event listener
    let mut events = Events::new(EventSettings::new());
    
    // while loop that gets events from window
    while let Some(e) = events.next(&mut window) {
        // gets arguemnts from event and renders
        if let Some(args) = e.render_args() {
            Game.render(&args);
        }

        // getys arguements from update and updates
        if let Some(args) = e.update_args() {
            Game.update(&args);
        }

        // gets aguements from update and updates buttons pressed
        if let Some(args) = e.button_args() {
            Game.update_keys(&args);
        }
    }
}
