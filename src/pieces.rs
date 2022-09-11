const PIECES: [[[i8]]; 7] = [
    [
        [1, 1], //  XX
        [1, 1]  //  XX
    ],
    [
        [0, 1, 0, 0],   //  X
        [0, 1, 0, 0],   //  X
        [0, 1, 0, 0],   //  X
        [0, 1, 0, 0]    //  X
    ],
    [
        [0, 0, 0],   //
        [0, 1, 1],   //   XX
        [1, 1, 0]    //  XX
    ],
    [
        [0, 0, 0],   //
        [1, 1, 0],   // XX
        [0, 1, 1]    //  XX
    ],
    [
        [0, 0, 0],  //  
        [1, 0, 0],  //  X
        [1, 1, 1]   //  XXX
    ],
    [
        [0, 0, 0],  //
        [0, 0, 1],  //    X
        [1, 1, 1]   //  XXX
    ],
    [
        [0, 0, 0],  //
        [0, 1, 0],  //   X
        [1, 1, 1]   //  XXX
    ]
];

pub struct Piece {
    piece_matrix: [[i8]],
    color: [f32; 4],
    pos_x: u8,
    pos_y: u8
}

impl Piece {
    pub fn is_colliding(&self, square_size: u32, pixel_grid_width: u32, pixel_grid_height: u32) -> bool {
        for (line_index, line) in piece_matrix.iter().enumerate() {
            for (case_index, case) in line.iter().enumerate() {
                if case == 1 { // So here we have a square that is representing a part of the piece
                    // Testing the left side
                    let square_pos_x = self.pos_x+case_index*square_size;
                    is_square_colliding_left_side(square_pos_x);
                    is_square_colliding_right_side(square_pos_x, pixel_grid_width);
                }
            }
        }

        false
    }

    fn is_square_colliding_left_side(square_pos_x: u8) {
        if square_pos_x < 0 {
            true
        }
        false
    }

    fn is_square_colliding_right_side(square_pos_x: u8, pixel_grid_width: u8) {
        if square_pos_x >= pixel_grid_width {
            true
        }
        false
    }
 }