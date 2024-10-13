
use fumen::Piece;
use js_sys::{Array, Number};
use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi, RefFromWasmAbi};
use core::fmt;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::AddAssign;
use std::vec;
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
    S=7,
    G=8
}
static PIECES: [PieceColor; 7] = [
    PieceColor::I,
    PieceColor::L,
    PieceColor::O,
    PieceColor::Z,
    PieceColor::T,
    PieceColor::J,
    PieceColor::S
];
impl From<u8> for PieceColor {
    fn from(c: u8) -> Self {
        match c {
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
}

pub fn invert_piece_vec(vec: Vec<PieceColor>) -> Vec<PieceColor> {
    let mut ret = Vec::new();
    for piece in PIECES {
        if !vec.contains(&piece) {
            ret.push(piece);
        }
    }
    ret
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
        8 => PieceColor::G,
        _ => PieceColor::T
    }
}
#[wasm_bindgen(js_name = "pieceColorFromStr")]
pub fn piece_color_from_str(str: &str) -> PieceColor {
    match str {
        "B" => PieceColor::B,
        "I" => PieceColor::I,
        "L" => PieceColor::L,
        "O" => PieceColor::O,
        "Z" => PieceColor::Z,
        "T" => PieceColor::T,
        "J" => PieceColor::J,
        "S" => PieceColor::S,
        "G" => PieceColor::G,
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
        PieceColor::S => 'S',
        PieceColor::G => 'X'
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
#[derive(PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize, Debug)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3
}
impl AddAssign<i32> for Direction {
    fn add_assign(&mut self, rhs: i32) {
        *self = Self::from_int(self.to_i64() + rhs as i64);

    }
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
pub fn direction_to_i32(dir: Direction) -> i32 {
    direction_to_i8(dir) as i32        
}
#[wasm_bindgen]
pub fn direction_to_i64(dir: Direction) -> i64 {
    direction_to_i8(dir) as i64        
}
impl Direction {
    pub fn to_i8(&self) -> i8 {
        direction_to_i8(*self)
    }
    pub fn to_i32(&self) -> i32 {
        direction_to_i32(*self)
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
impl From<u8> for Direction {
    fn from(int: u8) -> Self {
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

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
#[wasm_bindgen]
pub struct TetPiece {
    color: PieceColor,
    pub rotation: Direction,
    pub position: Vec2
}
impl fmt::Display for TetPiece { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minos = self.get_raw_minos();
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
                mino.0,
                mino.1
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
    pub fn z() -> TetPiece {
        TetPiece {
            color: PieceColor::Z,
            rotation: Direction::North,
            position: Vec2(0, 0)
        }
    }
    pub fn i() -> TetPiece {
        TetPiece {
            color: PieceColor::I,
            rotation: Direction::North,
            position: Vec2(0, 0)
        }
    }
    pub fn j() -> TetPiece {
        TetPiece {
            color: PieceColor::J,
            rotation: Direction::North,
            position: Vec2(0, 0)
        }
    }
    pub fn l() -> TetPiece {
        TetPiece {
            color: PieceColor::L,
            rotation: Direction::North,
            position: Vec2(0, 0)
        }
    }
    pub fn o() -> TetPiece {
        TetPiece {
            color: PieceColor::O,
            rotation: Direction::North,
            position: Vec2(0, 0)
        }
    }
    pub fn t() -> TetPiece {
        TetPiece {
            color: PieceColor::T,
            rotation: Direction::North,
            position: Vec2(0, 0)
        }
    }
    pub fn s() -> TetPiece {
        TetPiece {
            color: PieceColor::S,
            rotation: Direction::North,
            position: Vec2(0, 0)
        }
    }
    #[wasm_bindgen(js_name = "applyGravity")]
    pub fn apply_gravity(&mut self, force: i32) {
        self.position.1 -= force;
    }
    #[wasm_bindgen(js_name = "clone")]
    pub fn js_clone(&self) -> TetPiece {
        TetPiece::new(self.color, self.rotation, self.position)
    }
    #[wasm_bindgen(js_name = moveLeft)]
    pub fn move_left(&mut self, amount: i32) {
        self.position.0 -= amount;
    }
    #[wasm_bindgen(js_name = moveRight)]
    pub fn move_right(&mut self, amount: i32) {
        self.position.0 += amount;
    }
    #[wasm_bindgen(js_name = getRawMinos)] 
    pub fn js_get_raw_minos(self: &TetPiece) -> Array { 
        self.get_raw_minos().iter().copied().map(JsValue::from).collect()
    }
    #[wasm_bindgen(js_name = getMinos)] 
    pub fn js_get_minos(self: &TetPiece) -> Array { 
        self.get_minos().iter().copied().map(JsValue::from).collect()
    }
    #[wasm_bindgen(getter)]
    pub fn color(&self) -> PieceColor {
        self.color.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_color(&mut self, color: PieceColor) {
        self.color = color;
    }
    #[wasm_bindgen(js_name = minoAbove)] 
    pub fn mino_above(&self, y: i8) -> bool {
        for mino in self.get_minos() {
            if mino.1 > y.into() {
                return true;
            }
        }
        false
    }

}
impl TetPiece {
    pub fn get_raw_minos(&self) -> PieceMinos {
        let mut minos: PieceMinos = BLOCKS[self.color as usize - 1];
        for i in 0..4 {
            let mino = &mut minos[i as usize];
            let temp = mino.0;
            if self.color != PieceColor::O {
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
impl From<(PieceColor, Vec2)> for TetPiece {
    fn from(value: (PieceColor, Vec2)) -> Self {
        TetPiece::new(
            value.0,
            Direction::North, 
            value.1
        )
    }
} 
pub fn get_pieces() -> [PieceColor; 7] {
    return [PieceColor::T, PieceColor::I, PieceColor::L, PieceColor::J, PieceColor::Z, PieceColor::S, PieceColor::O];
}


