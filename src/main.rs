// Libs import
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// Other files import
mod piece;
use std::usize;

use crate::piece::Piece;

mod constant;

// Important values
use crate::constant::BOXES_GRID_WIDTH;
use crate::constant::BOXES_GRID_HEIGTH;
use crate::constant::SQUARE_SIZE;
use crate::constant::BACKGROUND;
use crate::constant::PIXEL_GRID_HEIGTH;
use crate::constant::PIXEL_GRID_WIDTH;

// Colors
use crate::constant::RED;

// Pieces
use crate::constant::T_TETRIMINO;
use crate::constant::O_TETRIMINO;
use crate::constant::I_TETRIMINO;
use crate::constant::L_TETRIMINO;
use crate::constant::J_TETRIMINO;
use crate::constant::S_TETRIMINO;
use crate::constant::Z_TETRIMINO;

use piston::UpdateEvent;
// lets put some shortcuts that could be usefuls
//use std::process; // Will be used later
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct AppState{
    gl: GlGraphics,
    score: i32,
    piece_speed: i32, // Speed of the descent of the current piece to place
    grid: [[u8; BOXES_GRID_WIDTH as usize]; BOXES_GRID_HEIGTH as usize],
    current_piece: Piece
}

impl AppState {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let pos_x = self.current_piece.pos_x as f64;
        let pos_y = self.current_piece.pos_y as f64;

        // We wont draw it currently -> (TO DO)
        let square = rectangle::square(0.0, 0.0, SQUARE_SIZE as f64); // x -> x starting position | y -> y starting position

        // function included inside our opengl lib, when we will draw our window and everything, let's draw 
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl); // clear the grid

            // Writing the pieces already placed
            for (line_index, line) in self.grid.iter().enumerate() { // for every line
                for (case_index, case) in line.iter().enumerate() {
                    if case != &0 {
                        rectangle(RED, square, c.transform.trans((case_index*SQUARE_SIZE as usize) as f64, (line_index*SQUARE_SIZE as usize) as f64), gl); // TO-DO -> put the correct color instead of RED
                    }
                }
            }

            // Draw the piece
            for (line_index, line) in self.current_piece.matrix.iter().enumerate() {
                for (case_index, case) in line.iter().enumerate() {
                    if *case != 0 {
                        rectangle(RED, square, c.transform.trans(pos_x + (case_index as u32*SQUARE_SIZE) as f64, pos_y + (line_index as u32*SQUARE_SIZE) as f64), gl); // transform to enlarge our piece shape
                    }
                }
            }
            
        })
    }

    // Updating data
    fn update(&mut self, _args: &UpdateArgs) {
        // We don't want that the piece throw down of our window
        if !self.current_piece.autoSetPlaced(self.grid, self.current_piece.pos_y + self.piece_speed) {
            self.current_piece.pos_y += self.piece_speed;
        }
    }

    // When the user press a key
    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                /*
                Key::Up => {
                    // rotation
                }
                */
                Key::Down => {
                    self.piece_speed += 2;
                }
                Key::Left => {
                    println!("Before left verif posX-sqsize={}", self.current_piece.pos_x-SQUARE_SIZE as i32);
                    if !self.current_piece.isColliding(self.grid, self.current_piece.matrix, self.current_piece.pos_x-SQUARE_SIZE as i32, self.current_piece.pos_y) {
                        println!("Before turning left posX={}", self.current_piece.pos_x);
                        self.current_piece.pos_x -= SQUARE_SIZE as i32;
                        println!("After turning left posX={}", self.current_piece.pos_x);
                    }
                    /*
                    if self.current_piece.pos_x > 0 { // We don't want that the piece run away from the left of our window
                        self.current_piece.pos_x -= SQUARE_SIZE as i32;
                    }
                     */
                }
                Key::Right => {
                    if !self.current_piece.isColliding(self.grid, self.current_piece.matrix, self.current_piece.pos_x+SQUARE_SIZE as i32, self.current_piece.pos_y) {
                        self.current_piece.pos_x += SQUARE_SIZE as i32;
                    }
                    /*
                    if self.current_piece.pos_x < (PIXEL_GRID_WIDTH-SQUARE_SIZE) as i32 { // We don't want that the piece run away from the right of our window
                        self.current_piece.pos_x += SQUARE_SIZE as i32;
                    }
                     */
                }
                _ => {}
            }
        }
    }

    
    // When the user release the keys
    fn release(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                /*
                Key::Up => {
                    self.right_vel = 0;
                }
                */
                Key::Down => {
                    self.piece_speed = 1;
                }
                _ => {}
            }
        }
    }

}

fn main() {

    // Testing printing
    let game_name = "Tet-rust";
    println!("Welcome to {} !", game_name);

    // Init opengl
    let opengl = OpenGL::V4_5;

    // Defining our window and its parameters
    let mut window: GlutinWindow = WindowSettings::new(game_name, [PIXEL_GRID_WIDTH, PIXEL_GRID_HEIGTH])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Defining our app state values
    let mut app_state = AppState {
        gl: GlGraphics::new(opengl),
        score: 0,
        piece_speed: 1,
        grid: [[0; 10]; 22], // Define our grid to full blank case
        current_piece: Piece{
            color: RED, 
            pos_x: ((PIXEL_GRID_WIDTH)/2) as i32, // Starting piece position into x axis
            pos_y: -4 * SQUARE_SIZE as i32, // Starting piece position into y axis
            placed: false,
            matrix: T_TETRIMINO}
    };

    // Let's init an event listener to react to the user and re-render in function of that
    let mut events = Events::new(EventSettings::new());
    
    
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            app_state.render(&r);
        }
        
        // TO-DO
        if let Some(u) = e.update_args() {
            app_state.update(&u);
        }
        
        // TO-DO
        if let Some(b) = e.press_args() {
            app_state.press(&b);
        }

        // TO-DO
        if let Some(b) = e.release_args() {
            app_state.release(&b);
        }

    }

}
