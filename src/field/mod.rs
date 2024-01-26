use wasm_bindgen::prelude::*;
use crate::piece::{self, RotationState};


#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Field {
    tiles: [i8; 240],
    active_piece: Option<piece::Piece>

}

impl Field {
    pub fn new() -> Field {
        
        let new_field = Field { 
           
            tiles: [0; 240],
            active_piece: None

        };

        return new_field;
    }

    pub fn tile_occupied(self: &Field, x: usize, y: usize) -> bool {
        return self.tiles[y * 10 + x] != 0;
    }
    pub fn get_tile(self: &Field, x: usize, y: usize) -> i8 {
        return self.tiles[y * 10 + x];
    }
    pub fn set_tile(self: &mut Field, x: usize, y: usize, new: i8) {
        self.tiles[y * 10 + x] = new;
    }
    pub fn get_tile_array(self: &Field) -> [i8; 240] {
        return self.tiles;
    }
    pub fn from_int_array(arr: [i8; 240]) -> Field {
        let new_field = Field { 
        
            tiles: arr,
            active_piece: None
        };

        return new_field;
    }
    pub fn from_4h_array(arr: [i8; 40]) -> Field {
        let mut tiles: [i8; 240] = [0; 240];
        tiles[..=189].copy_from_slice(&[0; 190]);
        tiles[190..=229].copy_from_slice(&arr);
        tiles[230..].copy_from_slice(&[0; 10]);
        return Field::from_int_array(tiles);
    }
    pub fn get_active_piece(self: &Field) -> Option<piece::Piece> {
        return self.active_piece;
    }
    pub fn can_place_active_piece(self: &Field) -> bool { 
        
    }
    pub fn rotate_active_piece(self: &mut Field, state: RotationState) {
        match self.active_piece { 
            Some(mut piece) => {
                let direction = (piece.rotation - state) % 4;
                match direction { 
                    0 => {
                        // NONE
                    }
                    1 => {
                        // CW
                        match piece.rotation {
                            RotationState::North => {
                                
                            },
                            RotationState::East => todo!(),
                            RotationState::South => todo!(),
                            RotationState::West => todo!(),
                        }
                    }
                    2 => {
                        // 180
                    }
                    3 => {
                        // CCW
                    }
                }
            }
            None => {

            }
            
        }
    }

}