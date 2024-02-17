use js_sys::{Uint16Array, Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;
use std::fmt::{Display, Write};
use crate::colors::get_piece_color;
use crate::field;
use crate::kicks::get_kicks;
use crate::piece::{color_str, piece_color_from_int, piece_color_to_char, Direction, PieceColor};
use crate::{kicks::get_180_kicks, piece::TetPiece};
use crate::vec2::Vec2;
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Board {
    tiles: [u8; 240]
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
    pub fn get_tile(self: &Board, x: usize, y: usize) -> u8 {
        let pos = Vec2(x.try_into().unwrap(), y.try_into().unwrap());
        // print!("{:?}", pos);
        if self.in_bounds(pos) {
            return self.tiles[y * 10 + x];
        } else {
            return 8 
        }
        
    }
    pub fn set_tile(self: &mut Board, x: usize, y: usize, new: u8) {
        self.tiles[y * 10 + x] = new;
    }
    pub fn get_tile_array(self: &Board) -> [u8; 240] {
        return self.tiles;
    }
    pub fn get_tile_matrix(self: &Board) -> [[u8; 10]; 24] {
        let mut matrix: [[u8; 10]; 24] = [[0; 10]; 24];
        for y in 0..24 {
            for x in 0..10 {
                matrix[y][x] = self.get_tile(x, y);
            }
        }
        return matrix;
    }
    pub fn from_int_array(arr: [u8; 240]) -> Board {
        let new_board = Board { 
        
            tiles: arr
        };

        return new_board;
    }
    pub fn from_4h_array(arr: [u8; 40]) -> Board {
        let mut tiles: [u8; 240] = [0; 240];
        tiles[..=189].copy_from_slice(&[0; 190]);
        tiles[190..=229].copy_from_slice(&arr);
        tiles[230..].copy_from_slice(&[0; 10]);
        return Board::from_int_array(tiles);
    }

}
#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(js_name = fromIntArray)]
    pub fn from_int_array_js(arr: Uint8Array) -> Board {
        let arr = arr.to_vec().as_slice().try_into().unwrap();
        let new_board = Board { 
        
            tiles: arr
        };

        return new_board;
    }
    #[wasm_bindgen(js_name = from4hArray)]
    pub fn from_4h_array_js(arr: Uint8Array) -> Board {
        let mut tiles: [u8; 240] = [0; 240];
        tiles[..=189].copy_from_slice(&[0; 190]);
        tiles[190..=229].copy_from_slice(arr.to_vec().as_slice().try_into().unwrap());
        tiles[230..].copy_from_slice(&[0; 10]);
        return Board::from_int_array(tiles);
    }
    pub fn does_collide(&self, piece: TetPiece) -> bool {
        let mut minos = piece.get_raw_minos();
        // println!("{:?}", piece.position);
        // println!("{:?}", minos.map(|
        //     mino| Vec2(
        //         mino.0 + piece.position.0,
        //         mino.1 + piece.position.1
        //     )
        // ) );
        minos = minos.map(
            | mino | Vec2(mino.0 + piece.position.0, (23 - piece.position.1) - mino.1)
        );
        // println!("NEW DAS") ;
        for mut mino_pos in minos {
          
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
        return pos.0 > -1 && pos.0 < 10 && pos.1 > -1 && pos.1 < 24
    }

    pub fn rotate_piece(&self , piece: &mut TetPiece, rotation: u8) -> bool {
        let mut test_piece = piece.clone();
        let mod_rot = rotation % 4;
        let old_rot: usize = piece.rotation as usize;
        let new_rot = (piece.rotation + mod_rot as i64 ) % 4;
        test_piece.rotation = Direction::from_int(new_rot.into());
        if mod_rot == 2 {
            // 180 rotation
            let kicks = get_180_kicks(*piece);
            let mut passed_tests = true;
            for i in 0..2 { 
                let shift: Vec2 = kicks[old_rot][i] - kicks[new_rot as usize][i];
                test_piece.position += shift;
                if self.does_collide(test_piece) {
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
            // println!("Starting kicks, start rotation: {}, new rotation: {}", old_rot, new_rot);
            // println!("Raw minos of new rotation vs old: {:?} {:?}", test_piece.get_raw_minos(), piece.get_raw_minos());
            // println!("Actual minos of new rotation vs old: {:?} {:?}", test_piece.get_minos(), piece.get_minos());
            // println!("Applied minos for new rotation\n{}", field::Field::new(*self, test_piece));
            let mut passed_tests = true;
            for i in 0..5 { 
                let old_offset = kicks[old_rot][i];
                let new_offset = kicks[new_rot as usize][i];
                
                let shift: Vec2 = kicks[old_rot][i] - kicks[new_rot as usize][i];
                test_piece.position += shift;
                
                // print!("===========NEW ROT===========\n");
                // println!("Old offset: {:?}, New offset: {:?}", old_offset, new_offset);
                // println!("Attempting to rotate with offset {:?}", shift);
                // println!("{:?}", Vec2(10,23) - test_piece.position);
                // println!("{}", field::Field::new(*self, test_piece));
                if self.does_collide(test_piece) {
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
  
    pub fn das_piece(&self, piece: &mut TetPiece, direction: Direction) { 
        match direction { 
            Direction::East => {
                for i in 0..11 {
                    piece.position += Vec2(1, 0);
                    if self.does_collide(*piece) {
                        piece.position -= Vec2(1, 0);
                        break;
                    }
                }
            }
            Direction::West => {
                for i in 0..11 {
                    piece.position += Vec2(-1, 0);
                    if self.does_collide(*piece) {
                        piece.position -= Vec2(-1, 0);
                        break;
                    }
                }
            }
            Direction::South => {
                for i in 0..23 {
                    if self.does_collide(*piece) {
                        
                        piece.position += Vec2(0, 1);
                        break;
                    }
                    piece.position -= Vec2(0, 1);
                }
            }
            _ => {}
        }
        
    }
    #[wasm_bindgen(js_name = "canPlace")]
    pub fn can_place(&self, piece: TetPiece) -> bool {
        if self.does_collide(piece) {
           false 
        } else {
            let mut test = piece.clone();
            test.apply_gravity(1);
            self.does_collide(test)
        }
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..24 {
            for j in 0..10 {
                let tile = self.tiles[i * 10 + j];
                let tile_color = piece_color_from_int(tile);
                if tile == 8 {
                    f.write_str("X");
                } else {
                    f.write_str(&color_str(tile_color, String::from(piece_color_to_char(tile_color))));
                }
            }
            f.write_char('\n');
        }
        return std::fmt::Result::Ok(());
    }
}