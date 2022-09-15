// Defining some constant

// SIZING
pub const SQUARE_SIZE: u32 = 40; // Side size in pixels of a square into the grid
pub const BOXES_GRID_WIDTH: u32 = 10; // Width of the grid in square unit
pub const BOXES_GRID_HEIGTH: u32 = 22; // Width of the grid in square unit
pub const PIXEL_GRID_WIDTH: u32 = BOXES_GRID_WIDTH*SQUARE_SIZE; // Width of the grid in pixel unit
pub const PIXEL_GRID_HEIGTH: u32 = BOXES_GRID_HEIGTH*SQUARE_SIZE; // Width of the grid in pixel unit

// COLORS -> r, v, b, opacity
pub const BACKGROUND: [f32; 4] = [0., 0., 0., 1.]; // BLACK color
pub const RED: [f32; 4] = [0.85, 0., 0.02, 1.]; // RED color
pub const YELLOW: [f32; 4] = [0.99, 0.89, 0.15, 1.]; // YELLOW color
pub const LIGHT_BLUE: [f32; 4] = [0.11, 0.87, 0.86, 1.]; // LIGHT_BLUE color
pub const BLUE: [f32; 4] = [0., 0., 0.86, 1.]; // BLUE color
pub const ORANGE: [f32; 4] = [0.85, 0.52, 0.04, 1.]; // ORANGE color
pub const GREEN: [f32; 4] = [0.12, 0.89, 0.02, 1.]; // GREEN color
pub const PURPLE: [f32; 4] = [0.51, 0., 0.86, 1.]; // PURPLE color

// PIECES NORTH MATRIX
pub const O_TETRIMINO: [[u8; 4]; 4] = [
    [0,0,0,0],  //
    [0,1,1,0],  // XX
    [0,1,1,0],  // XX
    [0,0,0,0],  //
];

pub const I_TETRIMINO: [[u8; 4]; 4] = [
    [0,0,0,0],   //
    [1,1,1,1],   //  XXXX
    [0,0,0,0],   //
    [0,0,0,0]    //
];

pub const T_TETRIMINO: [[u8; 4]; 4] = [
    [0,0,0,0],  //
    [1,1,1,0],  // XXX
    [0,1,0,0],  //  X
    [0,0,0,0],  //
];

pub const L_TETRIMINO: [[u8; 4]; 4] = [
    [0,0,0,0],  //
    [0,0,0,1],  //   X
    [1,1,1,1],  // XXX
    [0,0,0,0],  //
];

pub const J_TETRIMINO: [[u8; 4]; 4] = [
    [0,0,0,0],  //
    [1,0,0,0],  // X
    [1,1,1,1],  // XXX
    [0,0,0,0],  //
];

pub const S_TETRIMINO: [[u8; 4]; 4] = [
    [0,0,0,0],  //
    [0,1,1,0],  //  XX
    [1,1,0,0],  // XX
    [0,0,0,0],  //
];

pub const Z_TETRIMINO: [[u8; 4]; 4] = [
    [0,0,0,0],  //
    [1,1,0,0],  // XX
    [0,1,1,0],  //  XX
    [0,0,0,0],  //
];