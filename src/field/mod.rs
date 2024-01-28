use wasm_bindgen::prelude::*;
use crate::piece::{Piece};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Field {
    tiles: [i8; 240],
    active_piece: Option<Piece>

}

impl Field {

    pub fn get_active_piece(self: &Field) -> Option<Piece> {
        return self.active_piece;
    }
    pub fn can_place_active_piece(self: &Field) -> bool { 
        false
    }
}