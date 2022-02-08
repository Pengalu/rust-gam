// shorthanding the imports
use crate::glutin_window::GlutinWindow as Window;
use crate::opengl_graphics::{GlGraphics, OpenGL};
use crate::piston::input::{RenderArgs, UpdateArgs, ButtonArgs};
use crate::piston::window::WindowSettings;
use crate::player::*;
use crate::inputHandler::*;

// create new public struct type "Game" that contains glgraphics object and rotation
pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    player: Player, // Player struct from player.rs
    inputs: Inputs,
}

// implements functionality for struct "Game"
impl Game {

    pub fn new() -> Game {
        Game {
            gl: GlGraphics::new(OpenGL::V3_2),
            player: (),
            inputs: (),
        }
    }

    // function rendering for rendering to screen :stuck_out_tongue:
    pub fn render(&mut self, args: &RenderArgs) {
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


    // update function
    pub fn update(&mut self, args: &UpdateArgs) {

    }

    // button update function
    pub fn update_keys(&mut self, args: &ButtonArgs) {
        
    }
}

pub fn new_window() -> Window {
    WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap()
}