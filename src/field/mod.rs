use wasm_bindgen::prelude::*;
use crate::piece::{self, RotationState};
use crate::kicks::{self, get_kick_for_piece, get_kicks};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Field {
    tiles: [i8; 240],
    active_piece: Option<piece::Piece>

}

impl Field {

    pub fn get_active_piece(self: &Field) -> Option<piece::Piece> {
        return self.active_piece;
    }
    pub fn can_place_active_piece(self: &Field) -> bool { 
        false
    }

    }

}