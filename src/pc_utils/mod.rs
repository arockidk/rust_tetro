mod u64_board_mod;
use std::any::Any;

use crate::queue;
use crate::piece::PieceColor;
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

use crate::queue::Queue;