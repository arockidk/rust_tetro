use fumen;
use std::fmt::{self, Write};

use wasm_bindgen::prelude::*;
use crate::{board::Board, piece::{self, Direction, Piece, PieceColor}, vec2::Vec2};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Field {
    pub board: Board,
    pub active_piece: Option<Piece>

}

/**
 * active_piece can simply be any piece, set color to 0 to represent nothing
 */
impl Field {
    pub fn new(board: Board, active_piece: Option<Piece>) -> Field {
        Field {board: board, active_piece: active_piece}
        
    }

    pub fn can_place_active_piece(self: &Field) -> bool { 
        false
    }
    pub fn das_piece(&mut self, direction: Direction){
        match self.active_piece {
            Some(mut p) => {
                self.board.das_piece(&mut p, direction);
            }
            None => ()
        }
        
        // print!("{:?}", self.active_piece.position);
    }
    pub fn rotate_piece(&mut self, rotation: i8) {
        match self.active_piece {
            Some(mut p) => {
                self.board.rotate_piece(&mut p, rotation);
            }
            None => ()
        }
    }
  
}
impl fmt::Display for Field { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut piece_minos: Option<[Vec2; 4]> = None;
        match self.active_piece { 
            Some(piece) => {
                piece_minos = Some(piece.get_raw_minos().map(
                    | mino | Vec2(mino.0 + piece.position.0, (23 - piece.position.1) - mino.1)
                ));
                // println!("Field display, active piece: {:?}", self.active_piece.position);
                // println!("{:?}", piece_minos);
                // println!("{:?}", self.active_piece.get_minos());
            }
            None => ()
        }

        for i in 0..24 {
            // print!("{} {}", 23 - i, i);
            for j in 0..10 {
                let mut tile = self.board.get_tile(j, i);
                match self.active_piece {
                    Some(piece) => {
                        match piece_minos {
                            Some(piece_minos) => {
                                for mino in piece_minos {
                                    // println!("{} {}", j, mino.0);
                                    if j as i64 == mino.0 && (i) as i64 == mino.1 {
                                        // println!("MATCH~!!!");
                                        // print!("{}", self.active_piece.color as i8);
                                        tile = piece.color as i8;
                                    }
                                }
                            }
                            None => ()
                        }

                    }
                    None => ()
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