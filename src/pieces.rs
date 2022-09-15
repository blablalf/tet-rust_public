use std::any;
use crate::constant::GRID_WIDTH;
use crate::constant::GRID_HEIGTH;

enum PieceType {
    /*
        ##
        ##
     */
    O_tetrimino ([[u8; 2]; 2]),

    /*
        #
        #
        #
        #
    */
    I_tetrimino ([[u8; 4]; 4]),

    /*
         #
        ###
    */
    T_tetrimino ([[u8; 3]; 3]),

    /*
        #
        ###
    */
    L_tetrimino ([[u8; 3]; 3]),

    /*
          #
        ###
    */
    J_tetrimino ([[u8; 3]; 3]),

    /*
         ##
        ##
    */
    S_tetrimino ([[u8; 3]; 3]),

    /*
        ##
         ##
    */
    Z_tetrimino ([[u8; 3]; 3]),

    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

const O_TETRIMINO:PieceType = PieceType::O_tetrimino([
    [1, 1], //  XX
    [1, 1]  //  XX
]);

const I_TETRIMINO:PieceType = PieceType::I_tetrimino([
    [0, 0, 0, 0],   //
    [1, 1, 1, 1],   //  XXXX
    [0, 0, 0, 0],   //
    [0, 0, 0, 0]    //
]);

const T_TETRIMINO:PieceType = PieceType::T_tetrimino([
    [0, 0, 0],  //
    [0, 1, 0],  //   X
    [1, 1, 1]   //  XXX
]);

const L_TETRIMINO:PieceType = PieceType::L_tetrimino([
    [0, 0, 1],  //    X
    [1, 1, 1],  //  XXX
    [0, 0, 0]   //
]);

const J_TETRIMINO:PieceType = PieceType::J_tetrimino([
    [1, 0, 0],  //  X
    [1, 1, 1],  //  XXX
    [0, 0, 0]   //
]);

const S_TETRIMINO:PieceType = PieceType::S_tetrimino([
    [0, 1, 1],  //   XX
    [1, 1, 0],  //  XX
    [0, 0, 0]   //
]);

const Z_TETRIMINO:PieceType = PieceType::Z_tetrimino([
    [1, 1, 0],  // XX
    [0, 1, 1],  //  XX
    [0, 0, 0]   //
]);

pub struct Piece {
    piece_matrix: Vec<Vec<i8>>,
    color: [f32; 4],
    pos_x: i32,
    pos_y: i32,
    placed: bool,
    pieces: Vec<Vec<Vec<i8>>>,
    pieceType: PieceType
}

impl Piece {
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

    fn is_square_colliding_left_side(&self, square_pos_x: i32) -> bool {
        square_pos_x < 0
    }

    fn is_square_colliding_right_side(&self, square_pos_x: i32, pixel_grid_width: u32) -> bool {
        square_pos_x >= pixel_grid_width as i32
    }

    fn getPieceMatrix(piece_type: PieceType) -> PieceType {
        match piece_type {
            PieceType::O_tetrimino(_) => return O_TETRIMINO,
            PieceType::I_tetrimino(_) => return I_TETRIMINO,
            PieceType::T_tetrimino(_) => return T_TETRIMINO,
            PieceType::L_tetrimino(_) => return L_TETRIMINO,
            PieceType::J_tetrimino(_) => return J_TETRIMINO,
            PieceType::S_tetrimino(_) => return S_TETRIMINO,
            PieceType::Z_tetrimino(_) => return Z_TETRIMINO,
            _ => return O_TETRIMINO
        };
    }

    fn getPieceMatrixWithNumber(piece_type_index: u8) -> PieceType {
        match piece_type_index {
            0 => return O_TETRIMINO,
            1 => return I_TETRIMINO,
            2 => return T_TETRIMINO,
            3 => return L_TETRIMINO,
            4 => return J_TETRIMINO,
            5 => return S_TETRIMINO,
            6 => return Z_TETRIMINO,
            _ => return O_TETRIMINO
        };
    }

    fn getRightRotation22(matrix: [[u8; 2]; 2]) -> [[u8; 2]; 2] {
        return [
            [matrix[0][1], matrix[0][0]],
            [matrix[1][1], matrix[0][1]]
        ];
    }

    fn getRightRotation33(matrix: [[u8; 3]; 3]) -> [[u8; 3]; 3] {
        return [
            [matrix[2][0], matrix[1][0], matrix[0][0]],
            [matrix[2][1], matrix[1][1], matrix[0][1]],
            [matrix[2][2], matrix[1][2], matrix[0][2]],
        ];
    }

    fn getRightRotation44(matrix: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
        return [
            [matrix[3][0], matrix[2][0], matrix[1][0], matrix[0][0]],
            [matrix[3][2], matrix[2][2], matrix[1][1], matrix[0][2]],
            [matrix[3][3], matrix[2][2], matrix[1][2], matrix[0][2]],
            [matrix[3][3], matrix[2][3], matrix[1][3], matrix[0][3]],
        ];
    }

    fn isColliding22(piece_matrix: [[u8; 2]; 2], game_grid:[[i8; GRID_WIDTH as usize]; GRID_HEIGTH as usize]) -> bool {
        false
    }

    /*
         TO DO
         - a function whoch makes the super-rotations of tetris game to a piece, and push the piece if it is too far from a side
         - a function which verify if any square is collisionning another one into the grid (if other pieces have been placed)
         - a funciton which can know when the current piece is placed or not
    */
}