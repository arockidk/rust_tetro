use crate::piece::{self, PieceColor};

static COLORS: [&str; 8] = [
    "[90m",
    "[36m",
    "[38;2;255;165;0m",
    "[33m",
    "[31m",
    "[35m",
    "[34m",
    "[32m"
];

pub fn get_piece_color(piece_color: PieceColor) -> &'static str {
    COLORS[piece_color as usize]
}
pub fn get_blank() -> &'static str {
    "[0m"
}