use std::fmt::Write;

use crate::board::{Board, TetBoard};


pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        let shifted =  (val as u64) << (y * 10 + x);
        self.0 = self.0 & !shifted | shifted;
    }
    pub fn set_bool(&mut self, x: usize, y: usize, val: bool) {
        println!("{}", (y * 10 + 9 - x));
        let shifted =  (val as u64) << (y * 10 + x);
        self.0 = self.0 & !shifted | shifted;
    }
    pub fn get(&self, x: usize, y: usize) -> u64 {
        self.0 & (1 << (y * 10 + 9 - x)) >> (y * 10 + x)
    }
    pub fn repr(&self) -> String {
        let mut str = String::new();
        
        for i in 0..4 {
            for j in 0..10 {
                if self.0 & (1 << ((3-i) * 10 + j)) != 0 {
                    str.write_char('X');
                } else {
                    str.write_char('_');
                }
            }
            str.write_char('\n');
        }

        str
    }
    pub fn place(&self, ) {
        
    } 
}

impl From<TetBoard> for BitBoard {
    fn from(val: TetBoard) -> Self {
        let mut board = BitBoard(0);
        for i in 0..4 {
            for j in 0..10 {
                board.set_bool(j, i, val.get_tile(j as isize, i as isize) != 0);
            }
        }
        board
    }
}
impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}