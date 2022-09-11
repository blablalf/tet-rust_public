// Libs import
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// I inspired myself a lot from https://www.youtube.com/watch?v=-JIlCYbpNnI

use piston::UpdateEvent;
// lets put some shortcuts that could be usefuls
//use std::process; // Will be used later
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs}; // Some will be used later when we will react to the player
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

// Defining some constant

// SIZING
const SQUARE_SIZE: u32 = 40; // Side size in pixels of a square into the grid
const GRID_WIDTH: u32 = 10; // Width of the grid in square unit
const GRID_HEIGTH: u32 = 22; // Width of the grid in square unit

// COLORS (color -> r, v, b, opacity)
const BACKGROUND: [f32; 4] = [0., 0., 0., 1.]; // Black
const RED: [f32; 4] = [0.85, 0., 0.02, 1.]; // RED
const YELLOW: [f32; 4] = [0.99, 0.89, 0.15, 1.]; // YELLOW
const LIGHT_BLUE: [f32; 4] = [0.11, 0.87, 0.86, 1.]; // LIGHT_BLUE
const BLUE: [f32; 4] = [0., 0., 0.86, 1.]; // BLUE color
const ORANGE: [f32; 4] = [0.85, 0.52, 0.04, 1.]; // ORANGE color
const GREEN: [f32; 4] = [0.12, 0.89, 0.02, 1.]; // GREEN color
const PURPLE: [f32; 4] = [0.51, 0., 0.86, 1.]; // PURPLE color


pub struct AppState{
    gl: GlGraphics,
    score: i32,
    piece_speed: i32, // Speed of the descent of the current piece to place
    pos_x: i32, // x axis coordinate of the piece apparitions
    pos_y: i32,// y axis coordinate of the piece apparitions
    grid: [[i8; 10]; 22]
}

impl AppState {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let pos_x = self.pos_x as f64;
        let pos_y = self.pos_y as f64;

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
            rectangle(RED, square, c.transform.trans(pos_x, pos_y), gl); // transform to enlarge our piece shape
        })
    }

    // Updating data
    fn update(&mut self, _args: &UpdateArgs) {
        if self.pos_y <= (GRID_HEIGTH*SQUARE_SIZE-SQUARE_SIZE) as i32 { // We don't want that the piece throw down of our window
            self.pos_y += self.piece_speed;
        }
    }

    // When the user press a key
    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                /*
                Key::Up => {
                    self.right_vel = -1;
                }
                */
                Key::Down => {
                    self.piece_speed += 2;
                }
                Key::Left => {
                    if self.pos_x > 0 { // We don't want that the piece run away from the left of our window
                        self.pos_x -= SQUARE_SIZE as i32;
                    }
                }
                Key::Right => {
                    if self.pos_x < (GRID_WIDTH*SQUARE_SIZE-SQUARE_SIZE) as i32 { // We don't want that the piece run away from the right of our window
                        self.pos_x += SQUARE_SIZE as i32;
                    }
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


    // Just doing some test for fun
    println!("Printing this : {} !", addition(9, 18));

    // Init opengl
    let opengl = OpenGL::V4_5;

    // Defining our window and its parameters
    let mut window: GlutinWindow = WindowSettings::new(game_name, [GRID_WIDTH * SQUARE_SIZE, GRID_HEIGTH * SQUARE_SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Defining our app state values
    let mut app_state = AppState {
        gl: GlGraphics::new(opengl),
        score: 0,
        piece_speed: 1,
        pos_x: ((GRID_WIDTH*SQUARE_SIZE)/2) as i32, // Starting piece position into x axis
        pos_y: -4 * SQUARE_SIZE as i32, // Starting piece position into y axis
        grid: [[0; 10]; 22] // Define our grid to full blank case
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

// Testing a basic function declaration and definition
fn addition(number1: i32, number2: i32) -> i32 {
    number1 + number2
}