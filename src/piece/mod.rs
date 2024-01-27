use wasm_bindgen::prelude::*;
use std::{sync::Mutex, ops::Sub};
extern crate once_cell;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::vec2::Vec2;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum PieceColor {

    I=1,
    L=2,
    O=3,
    Z=4,
    T=5,
    J=6,
    S=7
}
impl PieceColor {
    pub fn from_int(int: i8) -> PieceColor {
        match int {
            1 => PieceColor::I,
            2 => PieceColor::L,
            3 => PieceColor::O,
            4 => PieceColor::Z,
            5 => PieceColor::T,
            6 => PieceColor::J,
            7 => PieceColor::S,
            _ => PieceColor::T
        }
    }
    pub fn to_char(&self) -> char{
        match self {
            PieceColor::I => 'I',
            PieceColor::J => 'J',
            PieceColor::L => 'L',
            PieceColor::O => 'O',
            PieceColor::T => 'T',
            PieceColor::Z => 'Z',
            PieceColor::S => 'S'
        }
    }
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RotationState {
    North = 0,
    East = 1,
    South = 2,
    West = 3
}
impl Sub for RotationState { 
    type Output = i64;
    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs
    }
}
impl Add for RotationState { 
    type Output = i64;
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}
impl Add<i8> for RotationState { 
    type Output = i8;
    fn add(self, rhs: i8) -> Self::Output {
        self + rhs
    }
}
impl RotationState {
    pub fn from_int(int: i64) -> RotationState {
        match int {
            0 => RotationState::North,
            1 => RotationState::East,
            2 => RotationState::South,
            3 => RotationState::West,
            _ => RotationState::North
        }
    }
}
#[derive(Clone, Copy)]
pub struct Block {
    color: PieceColor,
    positions: [(i32,i32); 4]
}

const static BLOCKS: [Vec2
fn set_block(color: &PieceColor, value: Block) { 
    BLOCKS.lock().unwrap().insert(*color, value);
}
pub fn get_block(color: PieceColor) -> Block {
   
    if BLOCKS.lock().unwrap().keys().any(|&clr| clr == color) {
        match color {
            PieceColor::T => {
                set_block(&color, Block {
                    color: PieceColor::T,
                    positions: [(0,0),(-1,0),(1,0),(0,1)]
                }) 
            },
            PieceColor::I => {
                set_block(&color, Block {
                    color: PieceColor::I,
                    positions: [(0,0),(-1,0),(1,0),(2,0)]
                }) ;
            },
            PieceColor::L => {
                set_block(&color, Block {
                    color: PieceColor::L,
                    positions: [(0,0),(-1,0),(1,0),(1,1)]
                });

            },
            PieceColor::J => {
                set_block(&color, Block {
                    color: PieceColor::J,
                    positions: [(0,0),(-1,0),(1,0),(-1,1)]
                }) 

            },
            PieceColor::S => {
                set_block(&color, Block {
                    color: PieceColor::S,
                    positions: [(0,0),(-1,0),(0,1),(1,1)]
                });

                
            },
            PieceColor::Z => {
                set_block(&color, Block {
                    color: PieceColor::Z,
                    positions: [(0,0),(1,0),(0,1),(-1,1)]
                });

            },
            PieceColor::O => {
                set_block(&color, Block {
                    color: PieceColor::O,
                    positions: [(0,0),(1,0),(0,1),(1,1)]
                });
            },
        }
    }
    return BLOCKS.lock().unwrap()[&color];
}
#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub rotation: RotationState,
    pub position: (usize, usize)
}

impl Piece {
    pub fn new(color: PieceColor, rotation: RotationState, position: (usize, usize)) -> Piece {
        let piece = Piece {
            color: color,
            rotation: rotation,
            position: position
            
        };
        return piece;
    }
}

pub fn get_pieces() -> [PieceColor; 7] {
    return [PieceColor::T, PieceColor::I, PieceColor::L, PieceColor::J, PieceColor::Z, PieceColor::S, PieceColor::O];
}


    