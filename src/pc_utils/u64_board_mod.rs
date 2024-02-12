use std::path::Display;

use crate::{board::Board, piece::{piece_color_from_int, Piece, PieceColor}, vec2::Vec2};

pub struct u64_board(u64);

impl u64_board {
    
    pub fn new() -> u64_board {
        u64_board(0)
    }
    pub fn in_4h_bounds(&self, position: Vec2) -> bool {
        position.0 >= 0 && position.0 < 10 && position.1 >= 0 && position.1 < 4
    }
    pub fn in_6h_bounds(&self, position: Vec2) -> bool {
        position.0 >= 0 && position.0 < 10 && position.1 >= 0 && position.1 < 6
    }
    
    pub fn get_piece_color(&self) -> PieceColor {
        piece_color_from_int((self.0 & 15) as u8)
    }
    pub fn set_piece_color(&mut self, color: PieceColor) {
        self.0 = (self.0 & !15) | (color as u64);
    }
    pub fn get_tile(&self, x: isize, y: isize) -> bool {
        if self.in_6h_bounds(Vec2(x.try_into().unwrap(), y.try_into().unwrap())) {
            ((self.0 >> (4 + x + y * 10)) & 1) != 0
        } else {
            true
        }
        
    }

    pub fn set_tile(&mut self, x: isize, y: isize, new: bool) {
        self.0 = (self.0 & !(1 << (4 + x + y * 10))) | ((new as u64) << (4 + x + y * 10));
    }
    pub fn as_array(&self) -> [u8; 240] {
        let mut base = [0; 240];
        for i in 0..60 {
            base[i] = ((self.0 >> i >> 4) & 1) as u8;
        }
        base
    }
    pub fn as_board(&self) -> Board {
        let mut new_board = Board::new();
        for i in 0..60 {
            new_board.set_tile(i % 10, 22 - (i / 10), (((self.0 >> i >> 4) & 1) * 8).try_into().unwrap());
        }
        new_board
    }
    pub fn does_collide(&self, piece: Piece) -> bool { 
        let minos = piece.get_minos();
        if piece.position.1 > 6 {
            return true;
        } else {
            for mino in minos {
                if self.get_tile(mino.0.try_into().unwrap(), mino.1.try_into().unwrap()) {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn can_place(&self, piece: Piece) -> bool {
        if self.does_collide(piece) {
           false 
        } else {
            let mut test = piece.clone();
            test.apply_gravity(1);
            self.does_collide(test)
        }
    }
}
impl std::fmt::Display for u64_board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_board())
    }
}