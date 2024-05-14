mod u64board;
use std::any::Any;

use crate::board::{Board, TetBoard};
use crate::field::Field;
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
pub struct PiecePos(u8, u8, Direction);
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
use crate::queue::Queue;