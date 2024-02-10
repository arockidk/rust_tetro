// game implementation
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{field, piece, queue};
#[wasm_bindgen()]
pub struct Tetra {
    pub score: i32,
    field: field::Field,
    queue: queue::Queue,
    hold: piece::Piece
}