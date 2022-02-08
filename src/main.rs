// imports
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
mod player;

// shorthanding the imports
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, ButtonArgs, ButtonEvent};
use piston::window::WindowSettings;
use player::*;

// create new public struct type "App" that contains glgraphics object and rotation
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player: player, // Player struct from player.rs
    inputs: Inputs,
}

// implements functionality for struct "App"
impl App {

    // function rendering for rendering to screen :stuck_out_tongue:
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // create colors (deprecate soon since we will soon be loading texture)
        const WHITE: [f32; 4usize] = [1.0, 1.0, 1.0, 1.0];
        const GREEN: [f32; 4usize] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4usize] = [1.0, 0.0, 0.0, 1.0];
  
        // create square as placeholder for the player (deprecate soon since refactor)
        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = (self.transform.x * {args.window_size[0] / 16.0}, self.transform.y * {args.window_size[1] / 16.0});
        let image = Image::new().rect(square(0.0,0.0,200.0));
        
        // draw stuff 
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }
}

fn main() {
    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(OpenGL::V3_2),
    };

    // create event listener
    let mut events = Events::new(EventSettings::new());
    
    // while loop that gets events from window
    while let Some(e) = events.next(&mut window) {
        // gets arguemnts from event and renders
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        // getys arguements from update and updates
        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        // gets aguements from update and updates buttons pressed
        if let Some(args) = e.button_args() {
            app.updateButtons(&args);
        }
    }
}
