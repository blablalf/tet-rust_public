// Constant value 
use crate::constant::BOXES_GRID_WIDTH;
use crate::constant::BOXES_GRID_HEIGTH;
use crate::constant::PIXEL_GRID_HEIGTH;
use crate::constant::PIXEL_GRID_WIDTH;
use crate::constant::SQUARE_SIZE;

// Colors
use crate::constant::RED;
use crate::constant::YELLOW;
use crate::constant::LIGHT_BLUE;
use crate::constant::BLUE;
use crate::constant::ORANGE;
use crate::constant::GREEN;
use crate::constant::PURPLE;

// Pieces
use crate::constant::T_TETRIMINO;
use crate::constant::O_TETRIMINO;
use crate::constant::I_TETRIMINO;
use crate::constant::L_TETRIMINO;
use crate::constant::J_TETRIMINO;
use crate::constant::S_TETRIMINO;
use crate::constant::Z_TETRIMINO;

// Lib
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Piece {
    pub color: [f32; 4],
    pub pos_x: i32,
    pub pos_y: i32,
    pub placed: bool,
    pub matrix: [[u8; 4]; 4]
}

impl Piece {

    pub fn is_square_colliding_left_side(&self) -> bool {
        self.pos_x < 0
    }

    pub fn is_square_colliding_right_side(&self) -> bool {
        self.pos_x as i32 >= PIXEL_GRID_WIDTH as i32
    }

    /*
    fn getRightRotation22(&self, matrix: [[u8; 2]; 2]) -> [[u8; 2]; 2] {
        return [
            [matrix[0][1], matrix[0][0]],
            [matrix[1][1], matrix[0][1]]
        ];
    }

    fn getRightRotation33(&self, matrix: [[u8; 3]; 3]) -> [[u8; 3]; 3] {
        return [
            [matrix[2][0], matrix[1][0], matrix[0][0]],
            [matrix[2][1], matrix[1][1], matrix[0][1]],
            [matrix[2][2], matrix[1][2], matrix[0][2]],
        ];
    }
    */

    // Dynamic formulae : x=size-y-1; y=x
    fn perform_right_rotation(&self, matrix: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
        return [
            [matrix[3][0], matrix[2][0], matrix[1][0], matrix[0][0]],
            [matrix[3][1], matrix[2][1], matrix[1][1], matrix[0][1]],
            [matrix[3][2], matrix[2][2], matrix[1][2], matrix[0][2]],
            [matrix[3][3], matrix[2][3], matrix[1][3], matrix[0][3]],
        ];
    }

    /*
    // Dynamic formulae : x=y; y=size-x-1
    fn getLeftRotation(&self, matrix: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
        return [
            [matrix[0][3], matrix[1][3], matrix[2][3], matrix[3][3]],
            [matrix[0][2], matrix[1][2], matrix[2][2], matrix[3][2]],
            [matrix[0][1], matrix[1][1], matrix[1][2], matrix[3][1]],
            [matrix[0][0], matrix[1][0], matrix[2][0], matrix[3][0]],
        ];
    }
    */
    
    pub fn try_right_rotation(&mut self, game_grid:[[u8; BOXES_GRID_WIDTH as usize]; BOXES_GRID_HEIGTH as usize]) -> bool {
        if !self.placed && !self.is_colliding(game_grid, self.perform_right_rotation(self.matrix), self.pos_x, self.pos_y) {
            // If succeeds
            self.matrix = self.perform_right_rotation(self.matrix);
            return true;
        }
        return false;
    }
    

    pub fn is_colliding(&self, game_grid:[[u8; BOXES_GRID_WIDTH as usize]; BOXES_GRID_HEIGTH as usize], piece_matrix: [[u8; 4]; 4], pos_x: i32, pos_y: i32) -> bool {
        if pos_y >= 0 {
            for (line_index, line) in piece_matrix.iter().enumerate() {
                for (case_index, case) in line.iter().enumerate() {
                    // if we have a solid part into our matrix
                    if *case != 0 && (pos_x + (case_index*SQUARE_SIZE as usize) as i32) < 0 // Through the left
                        || *case != 0 && (pos_x + (case_index*SQUARE_SIZE as usize) as i32) >= PIXEL_GRID_WIDTH as i32
                        || *case != 0 && game_grid[(pos_y/SQUARE_SIZE as i32 + line_index as i32) as usize][(pos_x/SQUARE_SIZE as i32 + case_index as i32) as usize] != 0 {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn is_placed(&self, game_grid:[[u8; BOXES_GRID_WIDTH as usize]; BOXES_GRID_HEIGTH as usize], pos_y: i32) -> bool {
        for (line_index, line) in self.matrix.iter().enumerate() {
            for (case_index, case) in line.iter().enumerate() {
                // if we have a solid part into our matrix
                let temp_pos_y: i32 = if pos_y < 0 {0} else {pos_y};
                if (*case != 0 && (pos_y + (SQUARE_SIZE*line_index as u32) as i32 >= (PIXEL_GRID_HEIGTH-SQUARE_SIZE) as i32))
                    || *case != 0 && game_grid[line_index + 1 + (temp_pos_y/SQUARE_SIZE as i32) as usize][(case_index as i32 + self.pos_x/SQUARE_SIZE as i32) as usize] != 0 {
                    return true;
                }
            }
        }
        return false; 
    }

    pub fn auto_set_placed(&mut self, game_grid:[[u8; BOXES_GRID_WIDTH as usize]; BOXES_GRID_HEIGTH as usize], pos_y: i32) -> bool {
        if self.is_placed(game_grid, pos_y) {
            self.placed = true;
            if self.pos_y%SQUARE_SIZE as i32 != 0 {
                self.pos_y += SQUARE_SIZE as i32 - self.pos_y%SQUARE_SIZE as i32;
            }
            return true;
        }
        return false;
    }

    pub fn get_matrix(&self) -> [[u8; 4]; 4] {
        return self.matrix;
    }

    pub fn new() -> Self {
        // Init randomization
        let mut rng = rand::thread_rng();

        // Next tetrimino type
        let tetrimino;

        // Getting a random piece and init tetrimino
        match rng.gen_range(0..6) {
            0 => {
                tetrimino = T_TETRIMINO;
            }
            1 => {
                tetrimino = O_TETRIMINO;
            }
            2 => {
                tetrimino = I_TETRIMINO;
            }
            3 => {
                tetrimino = L_TETRIMINO;
            }
            4 => {
                tetrimino = J_TETRIMINO;
            }
            5 => {
                tetrimino = S_TETRIMINO;
            }
            6 => {
                tetrimino = Z_TETRIMINO;
            }
            _ => {
                tetrimino = T_TETRIMINO;
            }
        }

        let color;

        // Getting a random piece and init tetrimino
        match rng.gen_range(0..6) {
            0 => {
                color = RED;
            }
            1 => {
                color = YELLOW;
            }
            2 => {
                color = LIGHT_BLUE;
            }
            3 => {
                color = BLUE;
            }
            4 => {
                color = ORANGE;
            }
            5 => {
                color = GREEN;
            }
            6 => {
                color = PURPLE;
            }
            _ => {
                color = RED;
            }
        }

        return Self {
            color: color, 
            pos_x: ((PIXEL_GRID_WIDTH)/2) as i32, // Starting piece position into x axis
            pos_y: -4 * SQUARE_SIZE as i32, // Starting piece position into y axis
            placed: false,
            matrix: tetrimino};
    }

    /*
         TO DO
         - implement a function to tell if the piece is too far at right or left
         - a function whoch makes the super-rotations of tetris game to a piece, and push the piece if it is too far from a side
         - a function which verify if any square is collisionning another one into the grid (if other pieces have been placed)
         - a funciton which can know when the current piece is placed or not
    */
}