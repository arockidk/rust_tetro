use wasm_bindgen::prelude::wasm_bindgen;
use std::fmt::{Display, Write};
use crate::colors::get_piece_color;
use crate::kicks::get_kicks;
use crate::piece::{PieceColor, Direction};
use crate::{kicks::get_180_kicks, piece::Piece};
use crate::vec2::Vec2;
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Board {
    tiles: [i8; 240]
}

impl Board {
    pub fn new() -> Board {
        
        let new_board = Board { 
           
            tiles: [0; 240]

        };

        return new_board;
    }
    pub fn to_board_coords(&self, original: Vec2) -> Vec2 {
        return Vec2(original.0, -original.1 + 23);
    }
    pub fn tile_occupied(self: &Board, x: usize, y: usize) -> bool {
        return self.get_tile(x, y) != 0;
    }
    pub fn get_tile(self: &Board, x: usize, y: usize) -> i8 {
        let pos = Vec2(x.try_into().unwrap(), y.try_into().unwrap());
        if self.in_bounds(pos) {
            return self.tiles[y * 10 + x];
        } else {
            return 8 
        }
        
    }
    pub fn set_tile(self: &mut Board, x: usize, y: usize, new: i8) {
        self.tiles[y * 10 + x] = new;
    }
    pub fn get_tile_array(self: &Board) -> [i8; 240] {
        return self.tiles;
    }
    pub fn from_int_array(arr: [i8; 240]) -> Board {
        let new_board = Board { 
        
            tiles: arr
        };

        return new_board;
    }
    pub fn from_4h_array(arr: [i8; 40]) -> Board {
        let mut tiles: [i8; 240] = [0; 240];
        tiles[..=189].copy_from_slice(&[0; 190]);
        tiles[190..=229].copy_from_slice(&arr);
        tiles[230..].copy_from_slice(&[0; 10]);
        return Board::from_int_array(tiles);
    }
    pub fn does_collide(self: &Board, piece: &Piece) -> bool {
        let minos = piece.get_minos();
        for mut mino_pos in minos {
            mino_pos.1 = 23 - mino_pos.1;
            if !self.in_bounds(mino_pos) {
                return true;
            }
            if (
                self.tile_occupied(
                mino_pos.0.try_into().unwrap(), 
                mino_pos.1.try_into().unwrap()
                )
            ) {
                return true; 
            }
        }
        return false;

    }
    pub fn in_bounds(&self, pos: Vec2) -> bool { 
        return pos.0 > -1 && pos.0 < 10 && pos.1 > -1 && pos.1 < 23
    }
    pub fn rotate_piece(self: &mut Board , piece: &mut Piece, rotation: i8) -> bool {
        let mut test_piece = piece.clone();
        let mod_rot = rotation % 4;
        let old_rot: usize = piece.rotation as usize;
        let new_rot = (piece.rotation + mod_rot ) % 4;
        test_piece.rotation = Direction::from_int(new_rot.into());
        if mod_rot == 2 {
            // 180 rotation
            let kicks = get_180_kicks(*piece);
            let mut passed_tests = true;
            for i in 0..kicks.len() { 
                let shift: Vec2 = kicks[old_rot][i] - kicks[new_rot as usize][i];
                test_piece.position += shift;
                if self.does_collide(&test_piece) {
                    test_piece.position -= shift;
                    passed_tests = false;
                } else {
                    piece.position = test_piece.position;
                    piece.rotation = test_piece.rotation;
                    return true;
                }
            }
            
        } else  {
            let kicks = get_kicks(*piece);
            
            let mut passed_tests = true;
            for i in 0..5 { 
                let shift: Vec2 = kicks[old_rot][i] - kicks[new_rot as usize][i];
                test_piece.position += shift;

                if self.does_collide(&test_piece) {
                    test_piece.position -= shift;
                    passed_tests = false;
                } else {
                    piece.position = test_piece.position;
                    piece.rotation = test_piece.rotation;
                    return true;
                }
                
            }
            
        }
        return false;
        
    }
    pub fn das_piece(&self, piece: &mut Piece, direction: Direction) { 
        match direction { 
            Direction::East => {
                for i in 0..11 {
                    piece.position += Vec2(1, 0);
                    if self.does_collide(&piece) {
                        piece.position -= Vec2(1, 0);
                        break;
                    }
                }
            }
            Direction::West => {
                for i in 0..11 {
                    piece.position += Vec2(-1, 0);
                    if self.does_collide(&piece) {
                        piece.position -= Vec2(-1, 0);
                        break;
                    }
                }
            }
            Direction::South => {
                for i in 0..23 {
                    piece.position -= Vec2(0, 1);
                    if self.does_collide(&piece) {
                        piece.position += Vec2(0, 1);
                        break;
                    }
                }
            }
            _ => {}
        }
        
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..24 {
            for j in 0..10 {
                let tile = self.tiles[i * 10 + j];
                let tile_color = PieceColor::from_int(tile);
                if tile == 8 {
                    f.write_str("X");
                } else {
                    f.write_str(&tile_color.color_str(tile_color.to_char().into()));
                }
            }
            f.write_char('\n');
        }
        return std::fmt::Result::Ok(());
    }
}