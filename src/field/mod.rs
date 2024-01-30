
use std::fmt::{self, Write};

use wasm_bindgen::prelude::*;
use crate::{board::Board, piece::{self, Piece, PieceColor}, vec2::Vec2};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Field {
    board: Board,
    active_piece: Piece

}
/**
 * active_piece can simply be any piece, set color to 0 to represent nothing
 */
impl Field {
    pub fn new(board: Board, active_piece: Piece) -> Field {
        Field {board: board, active_piece: active_piece}
        
    }

    pub fn can_place_active_piece(self: &Field) -> bool { 
        false
    }
}
impl fmt::Display for Field { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut piece_minos = self.active_piece.get_minos();
        piece_minos = piece_minos.map(
            | mino | self.board.to_board_coords(mino)
        );
        print!("{:?}", piece_minos);
        for i in 0..24 {
            for j in 0..10 {
                let mut tile = self.board.get_tile(j, i);
                for mino in piece_minos {
                    // println!("{} {}", j, mino.0);
                    if j as i64 == mino.0 && i as i64 == mino.1 {
                        // println!("MATCH~!!!");
                        // print!("{}", self.active_piece.color as i8);
                        tile = self.active_piece.color as i8;
                    }
                }


                
                let tile_color = PieceColor::from_int(tile);
                
                if tile == 8 {
                    f.write_str("X");
                } else {
                    f.write_str(&tile_color.color_str(tile_color.to_char().into()));
                }
            }
            f.write_char('\n');
        }
        return fmt::Result::Ok(());
    }
}