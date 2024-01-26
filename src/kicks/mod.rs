use crate::piece::{Piece, PieceColor};

pub type PieceOffset = (i32,i32);
#[derive(Copy, Clone)]
struct KickData {
    NE: [PieceOffset; 5],
    ES: [PieceOffset; 5],
    SW: [PieceOffset; 5],
    WN: [PieceOffset; 5],
    NW: [PieceOffset; 5],
    WS: [PieceOffset; 5],
    SE: [PieceOffset; 5],
    EN: [PieceOffset; 5],
    NS: [PieceOffset; 2],
    EW: [PieceOffset; 2],
    SN: [PieceOffset; 2],
    WE: [PieceOffset; 2]
}


pub static T_Type: KickData = KickData {
    NE: [( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)],
    ES: [( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)],
    SW: [( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1, 2)],
    WN: [( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)],

    NW: [( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)],
    WS: [( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)],
    SE: [( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)],
    EN: [( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)],

    NS: [( 0, 0),( 0, 1)],
    EW: [( 0, 0),( 1, 0)],
    SN: [( 0, 0),( 0,-1)],
    WE: [( 0, 0),(-1, 0)]
}; 

pub static I_Type: KickData = KickData {
    NE: [( 1, 0),(-1, 0),( 2, 0),(-1,-1),( 2, 2)],
    ES: [( 0,-1),(-1,-1),( 2,-1),(-1, 1),( 2,-2)],
    SW: [(-1, 0),( 1, 0),(-2, 0),( 1, 1),(-2,-2)],
    WN: [( 0, 1),( 1, 1),(-2, 1),( 1,-1),(-2, 2)],
    
    NW: [( 0,-1),(-1,-1),( 2,-1),(-1, 1),( 2,-2)],
    WS: [( 1, 0),(-1, 0),( 2, 0),(-1,-1),( 2, 2)],
    SE: [( 0, 1),( 1, 1),(-2, 1),( 1,-1),(-2, 2)],
    EN: [(-1, 0),( 1, 0),(-2, 0),( 1, 1),(-2,-2)],
    
    NS: [( 1,-1),( 1,0)],
    EW: [(-1,-1),(0,-1)],
    SN: [(-1, 1),(-1,0)],
    WE: [( 1, 1),(0, 1)]
}; 
pub fn get_kick_for_piece(piece: Piece) -> KickData { 
    match piece.color {
        PieceColor::I => I_Type,
        t_type => T_Type
        
    }
}