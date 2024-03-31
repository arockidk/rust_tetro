
use fumen::Piece;
use js_sys::Number;
use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi, RefFromWasmAbi};
use core::fmt;
use std::{fmt::{format, Write}, ops::{Add, Sub}};
use crate::{colors::{get_blank, get_piece_color}, vec2::Vec2};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialOrd, Ord)]
#[wasm_bindgen]
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
pub fn is_piece_color(c: char) -> bool {
    c == 'I' || c == 'L' || c == 'O' || c == 'Z' || c == 'T' || c == 'J' || c == 'S'
}

#[wasm_bindgen(js_name = "pieceColorFromInt")]
pub fn piece_color_from_int(int: u8) -> PieceColor {
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
#[wasm_bindgen(js_name = "pieceColorToChar")]
pub fn piece_color_to_char(color: PieceColor) -> char {
    match color {
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
pub fn color_str(color: PieceColor, str: String) -> String {
    format!("{}{}{}", get_piece_color(color), str, get_blank())
}   
impl From<char> for PieceColor {
    fn from(c: char) -> Self {
        match c {
            'I' => PieceColor::I,
            'J' => PieceColor::J,
            'L' => PieceColor::L,
            'O' => PieceColor::O,
            'T' => PieceColor::T,
            'Z' => PieceColor::Z,
            'S' => PieceColor::S,
            _ => PieceColor::T
        }
    }   
}
pub fn piece_color_from_char(c: char) -> PieceColor {
    match c { 
        'I' => PieceColor::I,
        'J' => PieceColor::J,
        'L' => PieceColor::L,
        'O' => PieceColor::O,
        'T' => PieceColor::T,
        'Z' => PieceColor::Z,
        'S' => PieceColor::S,
        _ => PieceColor::T
        
    }
}
#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3
}


#[wasm_bindgen]

pub fn direction_to_i8(dir: Direction) -> i8 {
    match dir {
        Direction::North => 0,
        Direction::East => 1,
        Direction::South => 2,
        Direction::West => 3
    }
}
#[wasm_bindgen]
pub fn direction_to_i64(dir: Direction) -> i64 {
    direction_to_i8(dir) as i64        
}
impl Direction {
    pub fn to_i8(&self) -> i8 {
        direction_to_i8(*self)
    }
    pub fn to_i64(&self) -> i64 {
        direction_to_i64(*self)
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
impl Add<i64> for Direction { 
    type Output = i64;
    fn add(self, rhs: i64) -> Self::Output {
        self.to_i64() + rhs
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



static BLOCKS: [PieceMinos; 7] = [
    // I
    [

        Vec2(-1, 0), Vec2( 0, 0), Vec2( 1, 0), Vec2( 2, 0)


    ],
    // L
    [
                                  Vec2( 1, 1),
        Vec2(-1, 0), Vec2( 0, 0), Vec2( 1, 0), 
        
    ],
    // O
    [
                     Vec2( 0, 1), Vec2( 1, 1),
                     Vec2( 0, 0), Vec2( 1, 0),  


    ],
    // Z
    [
        Vec2(-1, 1), Vec2( 0, 1),
                     Vec2( 0, 0), Vec2( 1, 0), 
        
    ],
    // T
    [
                     Vec2( 0, 1),
        Vec2(-1, 0), Vec2( 0, 0), Vec2( 1, 0), 
        
    ],
    // J
    [
        Vec2(-1, 1),
        Vec2(-1, 0), Vec2( 0, 0), Vec2( 1, 0), 
        
    ],
    
    // S
    [
                     Vec2( 0, 1), Vec2( 1, 1), 
        Vec2(-1, 0), Vec2( 0, 0),
                                            
    ],
    

];

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[wasm_bindgen]
pub struct TetPiece {
    pub color: PieceColor,
    pub rotation: Direction,
    pub position: Vec2
}
impl fmt::Display for TetPiece { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minos = self.get_minos();
        let mut str_matrix: [[&str; 4]; 4] = [
            [".", ".", ".", "."],
            [".", ".", ".", "."],
            [".", ".", ".", "."],
            [".", ".", ".", "."]
        ];
        let coloured = &color_str(
            self.color,
            String::from(piece_color_to_char(self.color))
        );
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
pub type PieceMinos = [Vec2; 4];

#[wasm_bindgen]
impl TetPiece {
    #[wasm_bindgen(constructor)]
    pub fn new(color: PieceColor, rotation: Direction, position: Vec2) -> TetPiece {
        let piece = TetPiece {
            color,
            rotation,
            position
            
        };
        return piece;
    }
    
    #[wasm_bindgen(js_name = "applyGravity")]
    pub fn apply_gravity(&mut self, force: i64) {
        self.position.1 -= force;
    }
}
impl TetPiece {
    pub fn get_raw_minos(&self) -> PieceMinos {
        let mut minos: PieceMinos = BLOCKS[self.color as usize - 1];
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
    pub fn get_minos(self: &TetPiece) -> PieceMinos {
        let mut minos: PieceMinos = self.get_raw_minos();
        // println!("{:?}", minos);
        for mino in &mut minos {
            mino.0 += self.position.0;
            mino.1 += self.position.1;
        } 
        return minos;
    }
}

pub fn get_pieces() -> [PieceColor; 7] {
    return [PieceColor::T, PieceColor::I, PieceColor::L, PieceColor::J, PieceColor::Z, PieceColor::S, PieceColor::O];
}


    