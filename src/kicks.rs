use crate::piece::{TetPiece, PieceColor};
use crate::vec2::Vec2;

pub type KickData = [[Vec2; 5]; 4];
pub type KickData180 = [[Vec2; 2]; 4];
// kicks taken from jan_ewan's stackfish because i cant find kicktables for jstris 180
// Kicks, comprised of 2-tuples
const KICKS: [[Vec2; 5]; 4] = [
    [Vec2(0,0); 5],
    [Vec2( 0, 0), Vec2( 1, 0), Vec2( 1,-1), Vec2( 0, 2), Vec2( 1, 2)],
    
    [Vec2(0,0); 5 ],
    [Vec2( 0, 0), Vec2(-1, 0), Vec2(-1,-1), Vec2( 0, 2), Vec2(-1, 2)],
];

const I_KICKS: [[Vec2; 5]; 4] = [
    [Vec2( 0, 0), Vec2(-1, 0), Vec2( 2, 0), Vec2(-1, 0), Vec2( 2, 0)],
    [Vec2(-1, 0), Vec2( 0, 0), Vec2( 0, 0),  Vec2(0, 1), Vec2( 0,-2)],
    [Vec2(-1, 1), Vec2( 1, 1), Vec2(-2, 1), Vec2( 1, 0), Vec2(-2, 0)],
    [Vec2( 0, 1), Vec2( 0, 1), Vec2( 0, 1), Vec2( 0,-1), Vec2( 0, 2)],
];

const O_KICKS: [[Vec2; 5]; 4] = [
    [Vec2( 0, 0 ), Vec2( 0, 0 ), Vec2( 0, 0 ), Vec2( 0, 0 ), Vec2( 0, 0 )],
    [Vec2( 0,-1 ), Vec2( 0, 0 ), Vec2( 0, 0 ), Vec2( 0, 0 ), Vec2( 0, 0 )],
    [Vec2(-1, 0 ), Vec2( 0, 0 ), Vec2( 0, 0 ), Vec2( 0, 0 ), Vec2( 0, 0 )],
    [Vec2(-1,-1 ), Vec2( 0, 0 ), Vec2( 0, 0 ), Vec2( 0, 0 ), Vec2( 0, 0 )]
];

const KICKS_180: [[Vec2; 2]; 4] = [
    [Vec2( 0, 0), Vec2( 0, 1)],
    [Vec2( 0, 0), Vec2( 1, 0)],
    [Vec2( 0, 0); 2],
    [Vec2( 0, 0); 2],
];

const I_KICKS_180: [[Vec2; 2]; 4] = [
    [Vec2( 1,-1), Vec2( 1, 0)],
    [Vec2(-1,-1), Vec2( 0, -1)],
    [Vec2( 0, 0); 2],
    [Vec2( 0, 0); 2],
];

const O_KICKS_180: [[Vec2; 2]; 4] = [
    [Vec2( 1, 1), Vec2( 0, 0 )],
    [Vec2( 1,-1), Vec2( 0, 0 )],
    [Vec2( 0, 0), Vec2( 0, 0 )],
    [Vec2( 0, 0), Vec2( 0, 0 )],
];

pub fn get_kicks(piece: TetPiece) -> KickData {
    match piece.color {
        PieceColor::I => I_KICKS,
        PieceColor::O => O_KICKS,
        _default => KICKS
        
    }
}
pub fn get_180_kicks(piece: TetPiece) -> KickData180 { 
    match piece.color { 
        PieceColor::I => I_KICKS_180,
        PieceColor::O => O_KICKS_180,
        _default => KICKS_180
    }
}