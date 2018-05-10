use piston::input::*;
use opengl_graphics::{ GlGraphics };
use piston::window::Size;
use std::cmp::*;
use std::cmp::min;

pub mod pieces;

use super::gui::{App, Data};
use self::pieces::{Piece, Move};

pub struct Game {
    width: u32, height: u32,
    pieces: Vec<Piece>
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            width, height, 
            pieces: Vec::new()
        }
    }

    pub fn from_vec(width: u32, height: u32, pieces: &Vec<Piece>) -> Game {
        let mut game = Game::new(width, height);

        game.pieces.extend_from_slice(&pieces);

        game
    }
}

const BACKGROUND: [f32; 4] = [0.2, 0.35, 0.5, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

impl App for Game {
    fn render(&self, args: &RenderArgs, mut gl: &mut GlGraphics, data: &Data) {
        use graphics::*;

        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);
        let s = (min(data.screen_width, data.screen_height) / min(self.width, self.height)) as f64;
        let sz = (s as u32 * min(self.width, self.height)) as f64;
        
        gl.draw(args.viewport(), |c, g| {
            // Clear the screen.
            clear(BACKGROUND, g);
            let transform = c.transform.trans((data.screen_width as f64 - sz) / 2.0, (data.screen_height as f64 - sz) / 2.0);
            for i in 0..self.width {
                for j in 0..self.height {
                    let c = {
                        if (i + j) % 2 == 0 {
                            WHITE
                        } else {
                            BLACK
                        }
                    };
                   rectangle(c, rectangle::square(s * i as f64, s * j as f64, s - 2.0), transform, g);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, data: &Data) {
        
    }
    
    fn handle_button(&mut self, args: &ButtonArgs, data: &Data) {

    }

    fn button_held(&mut self, args: &Button, data: &Data) {

    }
    
    fn mouse_moved(&mut self, args: &Motion, data: &Data) {

    }
    
    fn handle_cursor(&mut self, cursor: bool, data: &Data) {

    }

    fn handle_focus(&mut self, focus: bool, data: &Data) {

    }

    fn handle_resize(&mut self, width: u32, height: u32, data: &Data) {

    }
}
