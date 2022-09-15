use crate::constant::BOXES_GRID_WIDTH;
use crate::constant::BOXES_GRID_HEIGTH;

pub struct Piece {
    pub color: [f32; 4],
    pub pos_x: i32,
    pub pos_y: i32,
    pub placed: bool,
    pub matrix: [[u8; 4]; 4]
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

    pub fn is_square_colliding_left_side(&self) -> bool {
        self.pos_x < 0
    }

    pub fn is_square_colliding_right_side(&self, pixel_grid_width: u32) -> bool {
        self.pos_x >= pixel_grid_width as i32
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
    fn performRightRotation(&self, matrix: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
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
        if !self.isColliding(game_grid, self.performRightRotation(self.matrix)) {
            // If succeeds
            self.matrix = self.performRightRotation(self.matrix);
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
                    true;
                }
            }
        }
        false
    }

    fn isPlaced(&self, game_grid:[[i8; BOXES_GRID_WIDTH as usize]; BOXES_GRID_HEIGTH as usize]) -> bool {
        for (line_index, line) in self.matrix.iter().enumerate() {
            for (case_index, case) in line.iter().enumerate() {
                // if we have a solid part into our matrix
                if *case != 0 && self.pos_y == BOXES_GRID_HEIGTH as i32
                    || *case != 0 && game_grid[line_index + self.pos_y as usize + 1][case_index + self.pos_x as usize] != 0 {
                    return true;
                }
            }
        }
        return false; 
    }

    pub fn autoSetPlaced(&mut self, game_grid:[[i8; BOXES_GRID_WIDTH as usize]; BOXES_GRID_HEIGTH as usize]) -> bool {
        if self.isPlaced(game_grid) {
            self.placed = true;
            return true;
        }
        return false;
    }

    pub fn getMatrix(&self) {
        let returnedMatrix = self.matrix;
    }

    /*
         TO DO
         - implement a function to tell if the piece is too far at right or left
         - a function whoch makes the super-rotations of tetris game to a piece, and push the piece if it is too far from a side
         - a function which verify if any square is collisionning another one into the grid (if other pieces have been placed)
         - a funciton which can know when the current piece is placed or not
    */
}