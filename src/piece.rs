
use wasm_bindgen::prelude::*;
use core::fmt;
use std::{fmt::{format, Write}, ops::{Add, Sub}};
use crate::{colors::{get_blank, get_piece_color}, vec2::Vec2};

#[wasm_bindgen]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum PieceColor {
    B=0,
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
            0 => PieceColor::B,
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
    pub fn to_char(&self) -> char {
        match self {
            PieceColor::B => '.',
            PieceColor::I => 'I',
            PieceColor::J => 'J',
            PieceColor::L => 'L',
            PieceColor::O => 'O',
            PieceColor::T => 'T',
            PieceColor::Z => 'Z',
            PieceColor::S => 'S'
        }
    }
    pub fn color_str(&self, str: String) -> String {
        format!("{}{}{}", get_piece_color(*self), str, get_blank())
    }   
}
#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3
}

impl Direction {
    pub fn to_i8(&self) -> i8 {
        match self {
         Direction::North => 0,
         Direction::East => 1,
         Direction::South => 2,
         Direction::West => 3
        }
    }
    pub fn to_i64(&self) -> i64 {
        self.to_i8() as i64        
    }
}
impl Sub for Direction { 
    type Output = i64;
    fn sub(self, rhs: Self) -> Self::Output {
        self.to_i64() - rhs.to_i64()
    }
}
impl Add for Direction { 
    type Output = i64;
    fn add(self, rhs: Self) -> Self::Output {
        self.to_i64() + rhs.to_i64()
    }
}
impl Add<i8> for Direction { 
    type Output = i8;
    fn add(self, rhs: i8) -> Self::Output {
        self.to_i8() + rhs
    }
}
impl Direction {
    pub fn from_int(int: i64) -> Direction {
        match int {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => Direction::North
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
#[wasm_bindgen]
pub struct Piece {
    pub color: PieceColor,
    pub rotation: Direction,
    pub position: Vec2
}
impl fmt::Display for Piece { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minos = self.get_minos();
        let mut str_matrix: [[&str; 4]; 4] = [
            [".", ".", ".", "."],
            [".", ".", ".", "."],
            [".", ".", ".", "."],
            [".", ".", ".", "."]
        ];
        let coloured = self.color.color_str(self.color.to_char().into());
        for mino in minos {
            let pos = Vec2(
                (mino.0 - self.position.0 + 1),
                (mino.1 - self.position.1 + 2)
            );
            
            str_matrix[pos.1 as usize][pos.0 as usize] = coloured.as_str();
            
            // println!("{} {} {} {}", mino.0, self.position.0, mino.1, self.position.1);
            
        }
        for row in str_matrix {
            for char in row {
                f.write_str(char);
            }
            f.write_char('\n');
        }
        return fmt::Result::Ok(());
    }
}
impl Piece {
    pub fn new(color: PieceColor, rotation: Direction, position: Vec2) -> Piece {
        let piece = Piece {
            color: color,
            rotation: rotation,
            position: position
            
        };
        return piece;
    }
    pub fn get_raw_minos(&self) -> [Vec2; 4] {
        let mut minos: [Vec2; 4] = BLOCKS[self.color as usize - 1];
        for i in 0..4 {
            let mino = &mut minos[i as usize];
            let temp = mino.0;
            match self.rotation { 
             Direction::North => {}
             Direction::East => {
                    mino.0 = mino.1;
                    mino.1 = -temp;
                }
             Direction::South => {
                    mino.0 *= -1;
                    mino.1 *= -1;
                }
             Direction::West => {
                    mino.0 = -mino.1;
                    mino.1 = temp;
                }
            } 
        }
        return minos;
    }
    pub fn get_minos(self: &Piece) -> [Vec2; 4] {
        let mut minos = self.get_raw_minos();
        // println!("{:?}", minos);
        for mino in &mut minos {
            mino.0 += self.position.0;
            mino.1 += self.position.1;
        } 
        return minos;
    }
    pub fn apply_gravity(&mut self, force: i64) {
        self.position.1 -= force;
    }
}

pub fn get_pieces() -> [PieceColor; 7] {
    return [PieceColor::T, PieceColor::I, PieceColor::L, PieceColor::J, PieceColor::Z, PieceColor::S, PieceColor::O];
}


    