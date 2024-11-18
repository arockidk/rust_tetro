use std::fmt;

use crate::{piece::{piece_color_from_str, PieceColor}, queue::{self, Queue}};
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BitQueue(u32, u8);
const DIV_TABLE: [u8; 32] = [
    0,
    0,
    0,
    1,
    1,
    1,
    2,
    2,
    2,
    3,
    3,
    3,
    4,
    4,
    4,
    5,
    5,
    5,
    6,
    6,
    6,
    7,
    7,
    7,
    8,
    8,
    8,
    9,
    9,
    9,
    10,
    10
];
const PIECE_MAP: u32 = 0b0111;
impl BitQueue {
    pub fn new() -> BitQueue {
        BitQueue(0, 0)
    }
    pub fn len(&self) -> u8 {
        self.1 & 0b1111
    }

    pub fn at(&self, idx: u8) -> usize {
        if idx >= self.len() {
            return 0;
        }
        if idx == 11 {
            return ((self.0 >> 30) | (self.1 >> 4) as u32) as usize
        }
        ((self.0 >> idx*3) & PIECE_MAP) as usize
    }       
    pub fn push_back(&mut self, piece: usize) {
        self.0 |= (piece as u32) << (self.len() * 3);
        self.1 += 1;
    }
    pub fn pop_front(&mut self) -> usize {
        if self.len() == 0 {
            0
        } else {
            let piece = self.0 & PIECE_MAP;
            self.0 >>= 3;
            self.1 -= 1;
            piece as usize
        }
    }
}
impl From<Queue> for BitQueue {
    fn from(q: Queue) -> BitQueue {
        let mut b = BitQueue::new();
        for piece in q.iter() {
            b.push_back(piece.piece() as usize);
        }
        b
    }
}
impl From<&str> for BitQueue {
    fn from(s: &str) -> BitQueue {
        let mut b = BitQueue::new();
        for c in s.chars() {
            b.push_back(piece_color_from_str(c.to_string().as_str()) as usize);
        }
        b
    }
}
impl fmt::Display for BitQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = String::new();
        for i in 0..self.len() {
            builder.push_str(&format!("{:?}", PieceColor::from(self.at(i) as u32)));
        }
        f.write_str(builder.as_str())
    }
}