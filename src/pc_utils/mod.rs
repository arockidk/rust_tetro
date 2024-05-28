mod u64board;
use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;

use crate::board::{Board, TetBoard};
use crate::field::Field;
use crate::gameplay::Action;
use crate::vec2::Vec2;
use crate::{piece, queue};
use crate::piece::{Direction, PieceColor, TetPiece};
impl Queue {
    /**
     * Get columnar parity without T pieces.
     */
    pub fn get_columnar_parity(&self) -> u8 {
        let mut count = 0;
        for node in self.iter() {
            match node.node_type {
                queue::QueueNodeType::Piece => {
                    match node.piece() {
                        PieceColor::L => {
                            count += 1;
                        }
                        PieceColor::J => {
                            count += 1;
                        }
                        _ => {}
                    }
                }
                _ => {}
            } 
        }
        count
    } 
    pub fn t_count(&self) -> u8 {
        let mut count = 0;
        for node in self.iter() {
            match node.node_type {
                queue::QueueNodeType::Piece => {
                    match node.piece() {
                        PieceColor::T => {
                            count += 1;
                        }
                        _ => {}
                    }
                }
                _ => {}
            } 
        }
        count
    }
}
pub use u64board::u64_board;
#[derive(Clone, Copy, Debug)]
pub struct PiecePos(u16);
impl PiecePos {
    pub fn set_pos(&mut self, val: Vec2) {
        let (x, y) = (val.0, val.1);

    }
    pub fn set_x(&mut self, val: i32) {
        self.0 &= (0b00000111111);
        self.0 |= ((val & 0b11111) << 6) as u16;
    }
    pub fn set_y(&mut self, val: i32) {
        self.0 &= (0b11111000011);
        self.0 |= ((val & 0b1111) << 2) as u16;
    }
    pub fn set_rot(&mut self, val: i32) {
        self.0 &= (0b11111111100);
        self.0 |= (val & 0b11) as u16;
    }
}
impl From<TetPiece> for PiecePos {
    fn from(value: TetPiece) -> Self {
        PiecePos(
            ((((value.position.0 & 0b11111 << 6) |
            (value.position.1 & 0b1111) << 2) | 
            (value.rotation.to_i32() & 0b11))).try_into().unwrap()
        )
    }
    
}
impl Into<Vec2> for PiecePos {
    fn into(self) -> Vec2 {
        Vec2(((self.0 >> 6) & 0b11111).into(), ((self.0 >> 2) & 0b1111).into())
    }
}
impl Into<Direction> for PiecePos {
    fn into(self) -> Direction {
        Direction::from_int((self.0 & 0b11).into())
    }
}
pub struct PieceNode {
    val: PiecePos,
    actions: HashMap<u8, Box<PieceNode>>

}
impl u64_board {
    pub fn get_piece_placements(&self, mut piece: TetPiece, height: u8) -> Vec<PiecePos> {
        let mut placements = Vec::new();
        piece.position.1 = (height) as i32;
        piece.position.0 = 1;
        self.das_piece(&mut piece, Direction::South, 1000);
        println!("{}", Field::new(self.as_board(), Some(piece), None));
        
        placements

    }
}
impl TetPiece {
    pub fn set_piece_pos(&mut self, pos: PiecePos) {
        self.position = pos.into();
        self.rotation = pos.into();
    }
}

impl TetBoard {
    pub fn get_piece_placements(&self, mut piece: TetPiece) -> Vec<PiecePos> {
        let mut placements = Vec::new();
        let mut todo = Vec::new();
        let mut seen = [false; 32 * 16 * 4];
        let mut start_pos = PiecePos::from(piece);
        piece.set_piece_pos(start_pos);
        while !self.can_place(piece) {
            piece.rotation += 1;
            if piece.rotation.to_i8() > 3 {
                piece.rotation = Direction::North;
                piece.position.0 += 1;
                if piece.position.0 > 9 {
                    piece.position.1 += 1;
                    piece.position.0 = 0;
                }
            }

        }
        piece.set_piece_pos(start_pos);
        piece.move_left(1);
        if !self.does_collide(piece) {
            let pos = PiecePos::from(piece);
            seen[pos.0 as usize] = true;
            todo.push(pos);
            if self.can_place(piece) { 
                placements.push(pos);
            }
        } 
        piece.set_piece_pos(start_pos);
        piece.move_right(1);
        if !self.does_collide(piece) {
            let pos = PiecePos::from(piece);
            seen[pos.0 as usize] = true;
            todo.push(pos);
            if self.can_place(piece) { 
                placements.push(pos);
            }
        } 
        piece.set_piece_pos(start_pos);
        piece.apply_gravity(1);
        if !self.does_collide(piece) {
            let pos = PiecePos::from(piece);
            seen[pos.0 as usize] = true;
            todo.push(pos);
            if self.can_place(piece) { 
                placements.push(pos);
            }
        } 
        piece.set_piece_pos(start_pos);
        self.rotate_piece(&mut piece, 1);
        if !self.does_collide(piece) {
            let pos = PiecePos::from(piece);
            seen[pos.0 as usize] = true;
            todo.push(pos);
            if self.can_place(piece) { 
                placements.push(pos);
            }
        }
        piece.set_piece_pos(start_pos);
        self.rotate_piece(&mut piece, 3);
        if !self.does_collide(piece) {
            let pos = PiecePos::from(piece);
            seen[pos.0 as usize] = true;  todo.push(pos);
            if self.can_place(piece) { 
                placements.push(pos);
            }
        } 
        while todo.len() > 0 {
            if let Some(mut start_pos) = todo.pop() {
                piece.set_piece_pos(start_pos);
                piece.move_left(1);
                if !self.does_collide(piece) {
                    let pos = PiecePos::from(piece);
                    if !(seen[pos.0 as usize]) {
                        seen[pos.0 as usize] = true;
                        todo.push(pos);
                        if self.can_place(piece) { 
                            placements.push(pos);
                        }
                    }

                } 
                piece.set_piece_pos(start_pos);
                piece.move_right(1);
                if !self.does_collide(piece) {
                    let pos = PiecePos::from(piece);
                    if !(seen[pos.0 as usize]) {
                        seen[pos.0 as usize] = true;
                        todo.push(pos);
                        if self.can_place(piece) { 
                            placements.push(pos);
                        }
                    }
                } 
                piece.set_piece_pos(start_pos);
                piece.apply_gravity(1);
                if !self.does_collide(piece) {
                    let pos = PiecePos::from(piece);
                    if !(seen[pos.0 as usize]) {
                        seen[pos.0 as usize] = true;
                        todo.push(pos);
                        if self.can_place(piece) { 
                            placements.push(pos);
                        }
                    }
                } 
                piece.set_piece_pos(start_pos);
                self.rotate_piece(&mut piece, 1);
                if !self.does_collide(piece) {
                    let pos = PiecePos::from(piece);
                    if !(seen[pos.0 as usize]) {
                        seen[pos.0 as usize] = true;
                        todo.push(pos);
                        if self.can_place(piece) { 
                            placements.push(pos);
                        }
                    }
                }
                piece.set_piece_pos(start_pos);
                self.rotate_piece(&mut piece, 3);
                if !self.does_collide(piece) {
                    let pos = PiecePos::from(piece);
                    seen[pos.0 as usize] = true;  todo.push(pos);
                    if self.can_place(piece) { 
                        placements.push(pos);
                    }
                } 
            }
            
        }
        placements 
    }
}
use crate::queue::Queue;
