pub enum PieceColor {
    T,
    I,
    L,
    J,
    S,
    Z,
    O
}

struct Piece {
    color: PieceColor,
    positions: [(usize,usize); 4]
}

impl Piece {
    pub fn new(color: PieceColor, positions: [(usize,usize); 4]) -> Piece{
        let piece = Piece {
            color: color,
            positions: positions
        };
        piece
    }
}


    