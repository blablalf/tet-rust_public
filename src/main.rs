// Libs import
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// lets put some processes
use std::process;
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct AppState{
    gl: GlGraphics,
    score: i32,
    piece_x_vel: i32,
    piece_y_vel: i32,
    piece_x: i32,
    piece_y: i32,

}

fn main() {
    let game_name = "Tetrus";
    println!("Welcome to {} !", game_name);


    // Just doing some test for fun
    println!("Printing this : {} !", addition(9, 18));

}

fn addition(number1: i32, number2: i32) -> i32 {
    number1 + number2
}