// imports
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// shorthanding the imports
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

// create new public struct type "App" that contains glgraphics object and rotation
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

// implements functionality for struct "App"
impl App {

    // function rendering for rendering to screen :stuck_out_tongue:
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

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

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 0.005 * args.dt;
    }
}

fn main() {
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(OpenGL::V3_2),
        rotation: 0.0,
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
    }
}
