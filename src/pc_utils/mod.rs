mod u64_board_mod;
use std::any::Any;

use crate::board::TetBoard;
use crate::{piece, queue};
use crate::piece::{TetPiece, PieceColor};
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
pub use u64_board_mod::u64_board;
impl TetBoard {
    pub fn get_piece_placements(&self, mut piece: TetPiece, height: u8) {
        piece.position.1 = (height - 1) as i64;
    }
}
use crate::queue::Queue;