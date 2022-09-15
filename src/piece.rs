use crate::constant::BOXES_GRID_WIDTH;
use crate::constant::BOXES_GRID_HEIGTH;

pub struct Piece {
    color: [f32; 4],
    pos_x: i32,
    pos_y: i32,
    placed: bool,
    matrix: [[u8; 4]; 4],
    rotationsAmount_Mod4: u8
}

impl Piece {

    /*
    pub fn is_colliding(&self, square_size: u32, pixel_grid_width: u32, pixel_grid_height: u32) -> bool {
        for (line_index, line) in self.piece_matrix.iter().enumerate() {
            for (case_index, case) in line.iter().enumerate() {
                if *case == 1 as i8 { // So here we have a square that is representing a part of the piece
                    // Testing the left side
                    let square_pos_x = self.pos_x+(case_index as u32 * square_size) as i32;
                    self.is_square_colliding_left_side(square_pos_x);
                    self.is_square_colliding_right_side(square_pos_x, pixel_grid_width);
                }
            }
        }

        false
    }
    */

    fn is_square_colliding_left_side(&self, square_pos_x: i32) -> bool {
        square_pos_x < 0
    }

    fn is_square_colliding_right_side(&self, square_pos_x: i32, pixel_grid_width: u32) -> bool {
        square_pos_x >= pixel_grid_width as i32
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
    fn getRightRotation(&self, matrix: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
        return [
            [matrix[3][0], matrix[2][0], matrix[1][0], matrix[0][0]],
            [matrix[3][2], matrix[2][2], matrix[1][1], matrix[0][2]],
            [matrix[3][3], matrix[2][2], matrix[1][2], matrix[0][2]],
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
    
    pub fn tryRightRotation(&mut self, game_grid:[[i8; BOXES_GRID_WIDTH as usize]; BOXES_GRID_HEIGTH as usize]) -> bool {
        if !self.isColliding(game_grid, self.getRightRotation(self.matrix)) {
            // If succeeds
            self.rotationsAmount_Mod4 += 1;
            self.rotationsAmount_Mod4 %= 4;
            return true;
        }
        return false;
    }
    

    fn isColliding(&self, game_grid:[[i8; BOXES_GRID_WIDTH as usize]; BOXES_GRID_HEIGTH as usize], piece_matrix: [[u8; 4]; 4]) -> bool {
        for (line_index, line) in piece_matrix.iter().enumerate() {
            for (case_index, case) in line.iter().enumerate() {
                // if we have a solid part into our matrix
                if *case != 0 && game_grid[(self.pos_y + line_index as i32) as usize][(self.pos_x + case_index as i32) as usize] != 0 
                    || *case != 0 && self.pos_x + case_index as i32 > 0
                    || *case != 0 && (self.pos_x + case_index as i32) < BOXES_GRID_WIDTH as i32 {
                    return true;
                }
            }
        }
        false
    }

    /*
         TO DO
         - a function whoch makes the super-rotations of tetris game to a piece, and push the piece if it is too far from a side
         - a function which verify if any square is collisionning another one into the grid (if other pieces have been placed)
         - a funciton which can know when the current piece is placed or not
    */
}