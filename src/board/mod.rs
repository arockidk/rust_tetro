use crate::{kicks::get_180_kicks, piece::Piece};
use crate::vec2::Vec2;
pub struct Board {
    tiles: [i8; 240]
}
impl Board {
    pub fn new() -> Board {
        
        let new_board = Board { 
           
            tiles: [0; 240]

        };

        return new_board;
    }

    pub fn tile_occupied(self: &Board, x: usize, y: usize) -> bool {
        return self.tiles[y * 10 + x] != 0;
    }
    pub fn get_tile(self: &Board, x: usize, y: usize) -> i8 {
        return self.tiles[y * 10 + x];
    }
    pub fn set_tile(self: &mut Board, x: usize, y: usize, new: i8) {
        self.tiles[y * 10 + x] = new;
    }
    pub fn get_tile_array(self: &Board) -> [i8; 240] {
        return self.tiles;
    }
    pub fn from_int_array(arr: [i8; 240]) -> Board {
        let new_board = Board { 
        
            tiles: arr
        };

        return new_board;
    }
    pub fn from_4h_array(arr: [i8; 40]) -> Board {
        let mut tiles: [i8; 240] = [0; 240];
        tiles[..=189].copy_from_slice(&[0; 190]);
        tiles[190..=229].copy_from_slice(&arr);
        tiles[230..].copy_from_slice(&[0; 10]);
        return Board::from_int_array(tiles);
    }
    pub fn does_collide(self: &Board, piece: &Piece) -> bool {
        
    }
    pub fn rotate_piece(self: &mut Field, piece: &mut Piece, rotation: i8) {
        let test_piece = piece.clone();
        let mod_rot = rotation % 4;
        let old_rot: i8 = piece.rotation;
        let new_rot: i8 = (piece.rotation + mod_rot ) % 4;
        test_piece.rotation = new_rot;
        if (mod_rot == 2) {
            // 180 rotation
            let kicks = get_180_kicks(piece);
            for i in 0..2 { 
                let shift: Vec2 = kicks[old_rot][i] - kicks[new_rot][i];
            }
        }
        
    }
}