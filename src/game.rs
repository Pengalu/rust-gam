// shorthanding the imports
extern crate graphics;
use crate::glutin_window::GlutinWindow as Window;
use crate::opengl_graphics::{GlGraphics, OpenGL};
use crate::piston::input::{RenderArgs, UpdateArgs, ButtonArgs};
use crate::piston::window::WindowSettings;
use crate::player::*;
use crate::inputHandler::*;
use crate::graphics::rectangle::square;
use graphics::draw_state;
use crate::std::path::Path;
use opengl_graphics::Texture;
use opengl_graphics::TextureSettings;
use std::option;
use std::option::Option::None;
// create new public struct type "Game" that contains glgraphics object and rotation
pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    player: Player, // Player struct from player.rs
    inputs: Inputs,
}

// implements functionality for struct "Game"
impl Game {

    // constructor 
    pub fn new() -> Game {
        Game {
            gl: GlGraphics::new(OpenGL::V3_2),
            player: Player::new(20u64,20u64),
            inputs: Inputs::new(),
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
        //let square = rectangle::square(0.0, 0.0, 50.0);
        //HOLY FUCKIGN SHIT THERE WAS A VARIABLE CALLED SQUARE AND THE THING THOUGHT I WSAA CALLING THE VARIABLE AS A FUNCTION AND NOT THE METHOD TO MAKE A SQUARE I HATE THis -nate :)
        let (x, y) = ( self.player.transform.x as f64 * {args.window_size[0] / 16.0},  self.player.transform.y as f64 *{args.window_size[1] / 16.0}); //remember that as f64 will convert the type lol.
        let image = Image::new().rect(square(0.0,0.0,200.0)); //make a thing to plaster le texture on
        let texture = Texture::from_path(Path::new("./resources/PlayerSprites/PlayerPlaceHolder.png"),&TextureSettings::new()).unwrap();//remember to use & sign. I don't know why it just told me. Anyways this is the texture we're slapping on image
        
        // draw stuff 
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform = c
                .transform
                .trans(x, y);
                
            // Draw a box rotating around the middle of the screen.
           // rectangle(RED, square, transform, gl);
           image.draw(&texture, &draw_state::DrawState { scissor: None, stencil:None , blend: None } , c.transform, gl); //THIS FUCKING DRAWSTATE SHIT TOOK ME AN HOUR. IT TURNS OUT THAT THE EXAMPLE CODE WAS DEPRECATED (it was not said it was deprecated) and that default_draw_state() DOESNT EXIST>??????? SO I AHD TO MAKE MY OWN. FUCK
        });
    }


    // update function
    pub fn update(&mut self, args: &UpdateArgs) {

    }

    // button update function
    pub fn update_keys(&mut self, args: &ButtonArgs) {
        
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




































// i bet you cant look up AND stick out your tongue











































































// if u read this ur gay lmao