enum PieceType {
    /*
        ##
        ##
     */
    o_tetrimino ([[u8; 2]; 2]),

    /*
        #
        #
        #
        #
    */
    i_tetrimino ([[u8; 4]; 4]),

    /*
         #
        ###
    */
    t_tetrimino ([[u8; 3]; 3]),

    /*
        #
        ###
    */
    l_tetrimino ([[u8; 3]; 3]),

    /*
          #
        ###
    */
    j_tetrimino ([[u8; 3]; 3]),

    /*
         ##
        ##
    */
    s_tetrimino ([[u8; 3]; 3]),

    /*
        ##
         ##
    */
    z_tetrimino ([[u8; 3]; 3]),

    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

const O_TETRIMINO:PieceType = PieceType::o_tetrimino([
    [1, 1], //  XX
    [1, 1]  //  XX
]);

const I_TETRIMINO:PieceType = PieceType::i_tetrimino([
    [0, 0, 0, 0],   //  X
    [1, 1, 1, 1],   //  X
    [0, 0, 0, 0],   //  X
    [0, 0, 0, 0]    //  X
]);

const T_TETRIMINO:PieceType = PieceType::t_tetrimino([
    [0, 0, 0],  //
    [0, 1, 0],  //   X
    [1, 1, 1]   //  XXX
]);

const L_TETRIMINO:PieceType = PieceType::l_tetrimino([
    [0, 0, 1],  //    X
    [1, 1, 1],  //  XXX
    [0, 0, 0]   //
]);

const J_TETRIMINO:PieceType = PieceType::j_tetrimino([
    [1, 0, 0],  //  X
    [1, 1, 1],  //  XXX
    [0, 0, 0]   //
]);

const S_TETRIMINO:PieceType = PieceType::s_tetrimino([
    [0, 1, 1],   //   XX
    [1, 1, 0],    //  XX
    [0, 0, 0]   //
]);

const Z_TETRIMINO:PieceType = PieceType::z_tetrimino([
    [1, 1, 0],   // XX
    [0, 1, 1],    //  XX
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

    /*
         TO DO
         - a function whoch makes the super-rotations of tetris game to a piece, and push the piece if it is too far from a side
         - a function which verify if any square is collisionning another one into the grid (if other pieces have been placed)
         - a funciton which can know when the current piece is placed or not
    */
 }