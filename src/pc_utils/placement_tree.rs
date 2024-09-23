use std::collections::HashMap;

use crate::board::TetBoard;

use super::PiecePos;

pub struct PlacementTree {
    head: PlacementNode
}
pub struct PlacementNode {
    pub board: TetBoard,
    t: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    i: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    l: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    j: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    o: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    s: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    z: Option<HashMap<PiecePos, Box<PlacementNode>>>,

}
