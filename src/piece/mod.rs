use wasm_bindgen::prelude::*;
use std::{ops::Sub};
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



static BLOCKS: [[Vec2; 4]; 7] = [
    // I
    [

        Vec2(-1, 0), Vec2( 0, 0), Vec2( 1, 0), Vec2( 2, 0)


    ],
    // L
    [
                                  Vec2( 1,-1),
        Vec2(-1, 0), Vec2( 0, 0), Vec2( 1, 0), 
        
    ],
    // O
    [
                     Vec2( 0,-1), Vec2( 1,-1),
                     Vec2( 0, 0), Vec2( 1, 0),  


    ],
    // Z
    [
        Vec2(-1,-1), Vec2( 0,-1),
                     Vec2( 0, 0), Vec2( 1, 0), 
        
    ],
    // T
    [
                     Vec2( 0,-1),
        Vec2(-1, 0), Vec2( 0, 0), Vec2( 1, 0), 
        
    ],
    // J
    [
        Vec2(-1,-1),
        Vec2(-1, 0), Vec2( 0, 0), Vec2( 1, 0), 
        
    ],
    
    // S
    [
                     Vec2( 0,-1), Vec2( 1,-1), 
        Vec2(-1, 0), Vec2( 0, 0),
                                            
    ],
    

];

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
    pub fn get_minos(&self) {
        let base: [Vec2; 4] = BLOCKS[self.color - 1];
        let rotated: [Vec2; 4] = [(0,0); ];
        // apply rotation
        match self.rotation { 
            RotationState::East => {
                for mino in base {
                }

            }
            RotationState::South => {
                
            }
            RotationState::West => {
                
            }
        }
}

pub fn get_pieces() -> [PieceColor; 7] {
    return [PieceColor::T, PieceColor::I, PieceColor::L, PieceColor::J, PieceColor::Z, PieceColor::S, PieceColor::O];
}


    