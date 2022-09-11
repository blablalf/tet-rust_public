// Libs import
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// I inspired myself a lot from https://www.youtube.com/watch?v=-JIlCYbpNnI

// lets put some shortcuts that could be usefuls
//use std::process; // Will be used later
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs}; // Some will be used later when we will react to the player
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

// Defining some constant
const SQUARE_SIZE: u32 = 40; // Side size in pixels of a square into the grid
const GRID_WIDTH: u32 = 10; // Width of the grid in square unit
const GRID_LENGTH: u32 = 22; // Width of the grid in square unit


pub struct AppState{
    gl: GlGraphics,
    score: i32,
    piece_x_vel: i32, // we will consider this like the speed of our piece into the x axis
    piece_y_vel: i32, // same for y axis
    start_pos_x: i32, // x axis coordinate of the piece apparitions
    start_pos_y: i32// y axis coordinate of the piece apparitions
}

impl AppState {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; // Black color -> r, v, b, opacity
        const FOREGROUND: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; // Red color -> r, v, b, opacity

        let start_pos_x = self.start_pos_x as f64;
        let start_pos_y = self.start_pos_y as f64;

        // We wont draw it currently -> (TO DO)
        let piece = rectangle::square(0.0, 0.0, SQUARE_SIZE as f64); // x -> x starting position | y -> y starting position
        //let piece_x = self.piece_x as f64;
        //let piece_y = self.piece_y as f64;

        // function included inside our opengl lib, when we will draw our window and everything, let's draw 
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(FOREGROUND, piece, c.transform.trans(start_pos_x, start_pos_y), gl); // transform to enlarge our piece shape
        })
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
    let mut window: GlutinWindow = WindowSettings::new(game_name, [GRID_WIDTH * SQUARE_SIZE, GRID_LENGTH * SQUARE_SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Defining our app state values
    let mut app_state = AppState {
        gl: GlGraphics::new(opengl),
        score: 0,
        piece_x_vel: 0,
        piece_y_vel: 0,
        start_pos_x: 20, //
        start_pos_y: 20
    };

    // Let's init an event listener to react to the user and re-render in function of that
    let mut events = Events::new(EventSettings::new());
    
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app_state.render(&r);
        }
        
        // TO-DO
        /* 
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

        */
    }

}

// Testing a basic function declaration and definition
fn addition(number1: i32, number2: i32) -> i32 {
    number1 + number2
}