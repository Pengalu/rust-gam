// shorthanding the imports
extern crate graphics;
use crate::glutin_window::GlutinWindow as Window;
use crate::opengl_graphics::{GlGraphics, OpenGL};
use crate::piston::input::{RenderArgs, UpdateArgs, ButtonArgs};
use crate::piston::window::WindowSettings;
use crate::player::Player;
use crate::input_handler::*;
use crate::graphics::rectangle::square;
use std::path::Path;
use opengl_graphics::Texture;
use opengl_graphics::TextureSettings;
use std::option::Option::None;
// create new public struct type "Game" that contains glgraphics object and rotation
pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    player: Player, // Player struct from player.rs
    _inputs: InputHandler,
}

// implements functionality for struct "Game"
impl Game {
    // constructor 
    pub fn new() -> Game {
        Game {
            gl: GlGraphics::new(OpenGL::V3_2),
            player: Player::new(20f64,20f64),
            _inputs: InputHandler::new(),
        }
    }
    // function rendering for rendering to screen :stuck_out_tongue:
    pub fn render(&mut self, args: &RenderArgs) {
        // shorthand graphics library
        use graphics::*;
        
        // x and y from player
        let (x, y) = (self.player.transform.0, self.player.transform.1);

        let image = Image::new().rect(square(0.0,0.0,200.0)); 
        let texture = Texture::from_path(Path::new("./resources/PlayerSprites/PlayerPlaceHolder.png"),&TextureSettings::new()).unwrap();
        
        // draw stuff 
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear([1., 1., 1., 1.] as [f32; 4], gl);
            // create a transform variable from previous context, then transform it with custom coords
            let transform = c
                .transform
                .trans(x, y);
           image.draw(&texture, &draw_state::DrawState { scissor: None, stencil:None , blend: None } , transform, gl); 
        });
    }


    // update function
    pub fn update(&mut self, args: &UpdateArgs) {
        self.player.update(&args);
    }

    // button update function
    pub fn update_keys(&mut self, args: &ButtonArgs) {
        self.player.update_keys(&args);
    }
}

// helper function for making new window container
pub fn new_window() -> Window {
    WindowSettings::new("the game that is good (it's really good)", [200, 200])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap()
}
