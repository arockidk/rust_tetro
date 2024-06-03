
use std::fmt::{self, format, Write};

use wasm_bindgen::prelude::*;
use crate::{board::{Board, TetBoard}, piece::{self, color_str, piece_color_from_int, piece_color_to_char, Direction, PieceColor, PieceMinos, TetPiece}, vec2::Vec2};

#[wasm_bindgen]
#[derive(Clone, Copy)]

pub struct Field {
    pub board: TetBoard,
    pub active_piece: Option<TetPiece>,
    pub hold: Option<TetPiece>
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[wasm_bindgen()]
pub struct clear_struct(bool, Vec<isize>);
#[wasm_bindgen]
impl Field {
    #[wasm_bindgen(constructor)]
    pub fn new(board: TetBoard, active_piece: Option<TetPiece>, hold: Option<TetPiece>) -> Field {
        Field {board, active_piece, hold}
        
    }
    

    #[wasm_bindgen(js_name = canPlaceActivePiece)]
    pub fn can_place_active_piece(&self) -> bool { 
        match self.active_piece {
            Some(ref p) => {
                let a = self.board.can_place(p.clone());
  
                a
                
            }
            None => false
        }
    }
    #[wasm_bindgen(js_name = applyGravity)]
    pub fn apply_gravity(&mut self, force: i32) -> bool {
        match self.active_piece {
            Some(ref mut p) => self.board.apply_gravity(p, force),
            None => false
        }
    }
    #[wasm_bindgen(js_name = moveLeft)]
    pub fn move_left(&mut self, amount: i32) -> bool {
        match self.active_piece {
            Some(ref mut p) => self.board.move_left(p, amount),
            None => false
        }
    }
    #[wasm_bindgen(js_name = moveRight)]
    pub fn move_right(&mut self, amount: i32) -> bool {
        match self.active_piece {
            Some(ref mut p) => self.board.move_right(p, amount),
            None => false
        }
    }
    #[wasm_bindgen(js_name = dasPiece)]
    pub fn das_piece(&mut self, direction: Direction, force: i32) -> i8 {
        match self.active_piece {
            Some(ref mut p) => {
                self.board.das_piece(p, direction, force)
            }
            None => (3)
        }
        
        // print!("{:?}", self.active_piece.position);
    }
    #[wasm_bindgen(js_name = rotatePiece)]
    pub fn rotate_piece(&mut self, rotation: u8) {
        match self.active_piece {
            Some(ref mut p) => {
                self.board.rotate_piece(p, rotation);
            }
            None => ()
        }
    }
    #[wasm_bindgen(js_name = getTile)]
    pub fn get_tile(&self, x: isize, y: isize) -> u8 {
        self.board.get_tile(x, y)
    }
    #[wasm_bindgen(js_name = setTile)]
    pub fn set_tile(&mut self, x: isize, y: isize, color: u8) {
        self.board.set_tile(x, y, color);
    }
    #[wasm_bindgen(js_name = place_active_piece)] 
    pub fn place_active_piece(&mut self) -> bool {
        match self.active_piece {
            Some(p) => {
                self.board.place(p)
            },
            None => false
        }
    }   
    #[wasm_bindgen(js_name = place_n_clear_active_piece)]
    pub fn place_n_clear_active_piece(&mut self) -> clear_struct {
        let mut ret = clear_struct(false, Vec::new());
        match self.active_piece {
            Some(p) => {
                let res = self.board.place_n_clear(p);
                ret.0 = res.0;
                ret.1 = res.1;
            },
            None => {}  
        }
        ret
    }   
}
impl fmt::Display for Field { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut piece_minos: Option<PieceMinos> = None;
        match self.active_piece { 
            Some(piece) => {
                piece_minos = Some(piece.get_minos());
                // println!("Field display, active piece: {:?}", self.active_piece.position);
                // println!("{:?}", piece_minos);
                // println!("{:?}", self.active_piece.get_minos());
            }
            None => ()
        }

        for i in (0..self.board.height).rev() {
            // print!("{} {}", 23 - i, i);
            for j in 0..self.board.width {
                let mut tile = self.board.get_tile(j, i);
                match self.active_piece {
                    Some(piece) => {
                        match piece_minos {
                            Some(piece_minos) => {
                                for mino in piece_minos {
                                    // println!("{} {}", j, mino.0);
                                    if j as i32 == mino.0 && (i) as i32 == mino.1 {
                                        // println!("MATCH~!!!");
                                        // print!("{}", self.active_piece.color as u8);
                                        tile = piece.color() as u8;
                                    }
                                }
                            }
                            None => ()
                        }

                    }
                    None => ()
                }



                
                let tile_color = piece_color_from_int(tile);
                
                if tile == 8 {
                    f.write_str("X");
                } else {
                   f.write_str(&color_str(tile_color, String::from(piece_color_to_char(tile_color))));
                }
            }
            f.write_char('\n');
        }
        return fmt::Result::Ok(());
    }
}