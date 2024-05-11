use js_sys::{Array, Uint16Array, Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use std::fmt::{Display, Write};
use crate::colors::get_piece_color;
use crate::field;
use crate::kicks::get_kicks;
use crate::piece::{color_str, piece_color_from_int, piece_color_to_char, Direction, PieceColor};
use crate::{kicks::get_180_kicks, piece::TetPiece};
use crate::vec2::Vec2;
pub trait Board {
    fn get_tile_array(self: &Self) -> [u8; 200];
    fn get_tile_matrix(self: &Self) -> [[u8; 10]; 20];
    fn from_int_array(arr: [u8; 200]) -> Self;
    fn from_4h_array(arr: [u8; 40]) -> Self;
    fn tile_occupied(&self, x: isize, y: isize) -> bool;
    fn in_bounds(&self, position: Vec2) -> bool;
    fn does_collide(&self, piece: TetPiece) -> bool;
    fn rotate_piece(&self , piece: &mut TetPiece, rotation: u8) -> bool;
    fn das_piece(&self, piece: &mut TetPiece, direction: Direction);
    fn apply_gravity(&mut self, piece: &mut TetPiece);
    fn can_place(&self, piece: TetPiece) -> bool;
    fn set_tile(&mut self, x: isize, y: isize, value: u8);
    fn get_tile(&self, x: isize, y: isize) -> u8;
    fn clear_tile(&mut self, x: isize, y: isize);
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct TetBoard {
    tiles: [u8; 200],
    pub height: isize,
    pub width: isize
}

impl Board for TetBoard {
    
    fn get_tile_array(self: &TetBoard) -> [u8; 200] {
        return self.tiles;
    }
    fn get_tile_matrix(self: &TetBoard) -> [[u8; 10]; 20] {
        let mut matrix: [[u8; 10]; 20] = [[0; 10]; 20];
        for y in 0..20 {
            for x in 0..10 {
                matrix[y][x] = self.get_tile(x as isize, 19 - y as isize);
            }
        }
        return matrix;
    }
    /**
     * For ease of use, first elements (at top) corrispond to top most cells
     */
    fn from_int_array(mut arr: [u8; 200]) -> TetBoard {
        for i in 0..20 {
            for j in 0..10 {
                let temp = arr[i * 10 + j];
                arr[i * 10 + j] = arr[(19 - i) * 10 + j];
                arr[(19 - i) * 10 + j] = temp;
            }
        }
        let new_board = TetBoard { 
            height: 20,
            width: 10,
            tiles: arr
        };
        // println!("{}", new_board);
        return new_board;
    }
    
    fn from_4h_array(mut arr: [u8; 40]) -> TetBoard {
        for i in 0..4 {
            for j in 0..10 {
                let temp = arr[i * 10 + j];
                arr[i * 10 + j] = arr[(3 - i) * 10 + j];
                arr[(3 - i) * 10 + j] = temp;
            }
        }
        let mut tiles: [u8; 200] = [0; 200];

        for i in (0..4).rev() { 
            for j in 0..10 {
                tiles[i * 10 + j] = arr[(3 - i) * 10 + j];
            }
        }

        return TetBoard::from_int_array(tiles);
    } 
    fn tile_occupied(&self, x: isize, y: isize) -> bool {
        self.get_tile(x, y) != 0
    }

    fn set_tile(&mut self, x: isize, y: isize, new: u8) {
        if x > -1 && y > -1 {
            let x = x as usize;
            let y = y as usize;
            self.tiles[y * 10 + x] = new;
        }
        
    }
    
    fn get_tile(&self, x: isize, y: isize) -> u8 {
        let pos = Vec2(x.try_into().unwrap(), y.try_into().unwrap());
        // print!("{:?}", pos);
        if x > -1 && x < 10 && y > -1 && y < self.height  {
            
            self.tiles[pos.1 as usize * 10 + pos.0 as usize]
        } else if y >= self.height {
            0
        } else {
            8
        }
        
    }
    
    fn clear_tile(&mut self, x: isize, y: isize) {
        if x > 0 && y > 0 && x < self.width && y < self.height {
            let x = x as usize;
            let y = y as usize;
            self.tiles[y * 10 + x] = 0;
        }
    }    
    fn does_collide(&self, piece: TetPiece) -> bool {
        let mut minos = piece.get_minos();
        // println!("{:?}", piece.position);
        // println!("{:?}", minos.map(|
        //     mino| Vec2(
        //         mino.0 + piece.position.0,
        //         mino.1 + piece.position.1
        //     )
        // ) );
        // println!("NEW DAS") ;
        for mut mino_pos in minos {
          
            if self.tile_occupied(mino_pos.0 as isize, mino_pos.1 as isize) {
                return true;
            }
        }
        return false;

    }
    fn in_bounds(&self, pos: Vec2) -> bool { 
        return pos.0 > -1 && pos.0 < 10 && pos.1 > 0 && pos.1 < 20
    }
    fn rotate_piece(&self , piece: &mut TetPiece, rotation: u8) -> bool {
        let mut test_piece = piece.clone();
        let mod_rot = rotation % 4;
        let old_rot: usize = piece.rotation as usize;
        let new_rot = (piece.rotation + mod_rot as i64 ) % 4;
        test_piece.rotation = Direction::from_int(new_rot.into());
        if mod_rot == 2 {
            // 180 rotation
            let kicks = get_180_kicks(*piece);
            let mut passed_tests = true;
            for i in 0..2 { 
                let shift: Vec2 = kicks[old_rot][i] - kicks[new_rot as usize][i];
                test_piece.position += shift;
                if self.does_collide(test_piece) {
                    test_piece.position -= shift;
                    passed_tests = false;
                } else {
                    piece.position = test_piece.position;
                    piece.rotation = test_piece.rotation;
                    return true;
                }
            }
            
        } else  {
            let kicks = get_kicks(*piece);
            // println!("Starting kicks, start rotation: {}, new rotation: {}", old_rot, new_rot);
            // println!("Actual minos of new rotation vs old: {:?} {:?}", test_piece.get_minos(), piece.get_minos());
            // println!("Applied minos for new rotation\n{}", field::Field::new(*self, Some(test_piece), None));
            let mut passed_tests = true;
            for i in 0..5 { 
                let old_offset = kicks[old_rot][i];
                let new_offset = kicks[new_rot as usize][i];
                
                let shift: Vec2 = kicks[old_rot][i] - kicks[new_rot as usize][i];
                test_piece.position += shift;
                
                // print!("===========NEW ROT===========\n");
                // println!("Old offset: {:?}, New offset: {:?}", old_offset, new_offset);
                // println!("Attempting to rotate with offset {:?}", shift);
                // println!("{:?}", test_piece.position);
                // println!("{}", field::Field::new(*self, Some(test_piece), None));
                if self.does_collide(test_piece) {
                    test_piece.position -= shift;
                    passed_tests = false;
                } else {
                    piece.position = test_piece.position;
                    piece.rotation = test_piece.rotation;
                    return true;
                }
                
            }
            
        }
        return false;
        
    }

    fn das_piece(&self, piece: &mut TetPiece, direction: Direction) { 
        match direction { 
            Direction::East => {
                for i in 0..11 {
                    piece.position += Vec2(1, 0);
                    if self.does_collide(*piece) {
                        piece.position -= Vec2(1, 0);
                        break;
                    }
                }
            }
            Direction::West => {
                for i in 0..11 {
                    piece.position += Vec2(-1, 0);
                    if self.does_collide(*piece) {
                        piece.position -= Vec2(-1, 0);
                        break;
                    }
                }
            }
            Direction::South => {
                for i in 0..self.height {
                    if self.does_collide(*piece) {
                        
                        piece.position += Vec2(0, 1);
                        break;
                    }
                    piece.position -= Vec2(0, 1);
                }
            }
            _ => {}
        }
        
    }
    fn can_place(&self, piece: TetPiece) -> bool {
        if self.does_collide(piece) {
           false 
        } else {
            let mut test = piece.clone();
            test.apply_gravity(1);
            self.does_collide(test)
        }
    }
    fn apply_gravity(&mut self, piece: &mut TetPiece) {
        piece.position -= Vec2(0, 1);
        if self.does_collide(*piece) {
            piece.position += Vec2(0, 1);
        }
    }
}
#[wasm_bindgen]
impl TetBoard {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TetBoard {
        
        let new_board = TetBoard { 
            height: 20,
            width: 10,
            tiles: [0; 200]

        };

        return new_board;
    }

    #[wasm_bindgen(js_name = tileOccupied)]
    pub fn js_tile_occupied(&self, x: isize, y: isize) -> bool {
        self.get_tile(x, y) != 0
    }
    #[wasm_bindgen(js_name = getTile)]
    pub fn js_get_tile(&self, x: isize, y: isize) -> u8 {
        return self.get_tile(x, y);
        
    }
    #[wasm_bindgen(js_name = setTile)]
    pub fn js_set_tile(&mut self, x: isize, y: isize, value: u8) {
        self.set_tile(x, y, value);
    }
    #[wasm_bindgen(js_name = clearTile)]
    pub fn js_clear_tile(&mut self, x: isize, y: isize) {
        self.clear_tile(x, y);
    }
    #[wasm_bindgen(js_name = fromIntArray)]
    pub fn js_from_int_array(arr: Uint8Array) -> TetBoard {
        let arr = arr.to_vec().as_slice().try_into().unwrap();
        return TetBoard::from_int_array(arr);
    }
    #[wasm_bindgen(js_name = from4hArray)]
    pub fn js_from_4h_array(arr: Uint8Array) -> TetBoard {
        let mut tiles: [u8; 200] = [0; 200];
        tiles[..=189].copy_from_slice(&[0; 190]);
        tiles[190..=229].copy_from_slice(arr.to_vec().as_slice().try_into().unwrap());
        tiles[230..].copy_from_slice(&[0; 10]);
        return TetBoard::from_int_array(tiles);
    }
    #[wasm_bindgen(js_name = doesCollide)]
    pub fn js_does_collide(&self, piece: TetPiece) -> bool {
        self.does_collide(piece)

    }
    #[wasm_bindgen(js_name = inBounds)]
    pub fn js_in_bounds(&self, pos: Vec2) -> bool { 
        return pos.0 > -1 && pos.0 < 10 && pos.1 > -1 && pos.1 < 20
    }
    #[wasm_bindgen(js_name = rotatePiece)]
    pub fn js_rotate_piece(&self , piece: &mut TetPiece, rotation: u8) -> bool {
       self.rotate_piece(piece, rotation)
        
    }
    #[wasm_bindgen(js_name = "dasPiece")]
    pub fn js_das_piece(&self, piece: &mut TetPiece, direction: Direction) { 
        self.das_piece(piece, direction)
        
    }
    #[wasm_bindgen(js_name = "canPlace")]
    pub fn js_can_place(&self, piece: TetPiece) -> bool {
        self.can_place(piece)
    }
    #[wasm_bindgen(js_name = getTileArray)]
    pub fn js_get_tile_array(&self) -> Uint8Array {
        Uint8Array::from(self.get_tile_array().as_slice())
    }
    #[wasm_bindgen(js_name = getTileMatrix)]
    pub fn js_get_tile_matrix(&self) -> Array {
        let matrix = self.get_tile_matrix();
        let mut arr = Array::new();
        for i in 0..20 {
            let mut sub_arr = Array::new();
            for j in 0..10 {
                sub_arr.push(&JsValue::from(matrix[i][j]));
            }
            arr.push(&JsValue::from(sub_arr));
        }
        arr
    }
}
impl Display for TetBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let tile = self.tiles[((self.height - i) * self.width + j) as usize];
                let tile_color = piece_color_from_int(tile);
                if tile == 8 {
                    f.write_str("X");
                } else {
                    f.write_str(&color_str(tile_color, String::from(piece_color_to_char(tile_color))));
                }
            }
            f.write_char('\n');
        }
        return std::fmt::Result::Ok(());
    }
}