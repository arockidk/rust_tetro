// game implementation
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{field, piece, queue};
#[wasm_bindgen()]
pub struct Tetra {
    pub score: i32,
    pub field: field::Field,
    queue: queue::Queue,
    pub hold: piece::TetPiece
}
#[wasm_bindgen]
impl Tetra {
    #[wasm_bindgen(getter)]
    pub fn queue(&self) -> queue::Queue {
        self.queue.clone()
    } 
    #[wasm_bindgen(setter)]
    pub fn set_queue(&mut self, queue: queue::Queue) {
        self.queue = queue;
    }
}