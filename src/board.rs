use crate::colors::get_piece_color;
use crate::field;
use crate::kicks::get_kicks;
use crate::piece::{color_str, piece_color_from_int, piece_color_to_char, Direction, PieceColor};
use crate::vec2::Vec2;
use crate::{kicks::get_180_kicks, piece::TetPiece};
use fumen::Piece;
use js_sys::{Array, Uint16Array, Uint8Array};
use serde::de::value::{EnumAccessDeserializer, IsizeDeserializer};
use std::clone;
use std::fmt::{Display, Write};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TSpinResult {
    NoSpin,
    MiniSpin,
    TSpin,
}
#[wasm_bindgen()]
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClearStruct(pub bool, Vec<isize>);
impl ClearStruct {
    pub fn new(success: bool, lines: Vec<isize>) -> Self {
        ClearStruct(success, lines)
    }
    pub fn set_lines(&mut self, lines: Vec<isize>) {
        self.1 = lines;
    }
}
#[wasm_bindgen]
impl ClearStruct {
    #[wasm_bindgen(js_name = "getLines")]
    pub fn get_lines(&self) -> Vec<isize> {
        self.1.clone()
    }
}
pub struct ClearedLine(pub isize, pub Vec<PieceColor>);

pub trait Board {
    fn get_tile_array(self: &Self) -> [u8; 240];
    fn get_tile_matrix(self: &Self) -> [[u8; 10]; 24];
    fn from_int_array(arr: [u8; 240]) -> Self;
    fn from_4h_array(arr: [u8; 40]) -> Self;
    fn tile_occupied(&self, x: isize, y: isize) -> bool;
    fn in_bounds(&self, position: Vec2) -> bool;
    fn does_collide(&mut self, piece: TetPiece) -> bool;
    fn rotate_piece(&mut self, piece: &mut TetPiece, rotation: u8) -> bool;
    fn das_piece(&mut self, piece: &mut TetPiece, direction: Direction, force: i32) -> i8;
    fn apply_gravity(&mut self, piece: &mut TetPiece, force: i32) -> bool;
    fn move_left(&mut self, piece: &mut TetPiece, amount: i32) -> bool;
    fn move_right(&mut self, piece: &mut TetPiece, amount: i32) -> bool;
    fn can_place(&mut self, piece: TetPiece) -> bool;
    fn set_tile(&mut self, x: isize, y: isize, value: u8);
    fn get_tile(&self, x: isize, y: isize) -> u8;
    fn clear_tile(&mut self, x: isize, y: isize);
    fn place(&mut self, piece: TetPiece) -> bool;
    fn place_n_clear(&mut self, piece: TetPiece) -> ClearStruct;
    fn get_filled_rows(&self) -> Vec<isize>;
    fn clear_row(&mut self, row: isize) -> ClearedLine;
    fn unplace(&mut self, piece: TetPiece) -> bool;
    fn check_pc(&self) -> bool;
    fn check_t_spin(&self, piece: TetPiece) -> TSpinResult;
    fn refill_rows(&mut self, rows: Vec<ClearedLine>);
    fn place_clone(&self, piece: TetPiece) -> Self;
    fn to_gray(&mut self);
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TetBoard {
    tiles: [u8; 240],
    pub height: isize,
    pub width: isize,
}
impl Default for TetBoard {
    fn default() -> Self {
        Self::new()
    }
}
impl Board for TetBoard {
    fn get_tile_array(self: &TetBoard) -> [u8; 240] {
        return self.tiles;
    }
    fn get_tile_matrix(self: &TetBoard) -> [[u8; 10]; 24] {
        let mut matrix: [[u8; 10]; 24] = [[0; 10]; 24];
        for y in 0..24 {
            for x in 0..10 {
                matrix[y][x] = self.get_tile(x as isize, 23 - y as isize);
            }
        }
        return matrix;
    }
    /**
     * For ease of use, first elements (at top) corrispond to top most cells
     */
    fn from_int_array(mut arr: [u8; 240]) -> TetBoard {
        for i in 0..24 {
            for j in 0..10 {
                let temp = arr[i * 10 + j];
                arr[i * 10 + j] = arr[(23 - i) * 10 + j];
                arr[(23 - i) * 10 + j] = temp;
            }
        }
        let new_board = TetBoard {
            height: 24,
            width: 10,
            tiles: arr,
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
        let mut tiles: [u8; 240] = [0; 240];

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
        if x > -1 && x < 10 && y > -1 && y < self.height {
            self.tiles[pos.1 as usize * 10 + pos.0 as usize]
        } else if x < 0 || x > 9 {
            8
        } else if y >= self.height {
            0
        } else {
            8
        }
    }

    fn clear_tile(&mut self, x: isize, y: isize) {
        if x > -1 && y > -1 && x < self.width && y < self.height {
            let x = x as usize;
            let y = y as usize;
            self.tiles[y * 10 + x] = 0;
        }
    }
    fn does_collide(&mut self, piece: TetPiece) -> bool {
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
        return pos.0 > -1 && pos.0 < 10 && pos.1 > 0 && pos.1 < 24;
    }
    fn rotate_piece(&mut self, piece: &mut TetPiece, rotation: u8) -> bool {
        let mut test_piece = piece.clone();
        let mod_rot = rotation % 4;
        let old_rot: usize = piece.rotation as usize;
        let new_rot = (piece.rotation + mod_rot as i64) % 4;
        test_piece.rotation = Direction::from_int(new_rot.into());
        if (piece.color() == PieceColor::O) {
            piece.rotation = test_piece.rotation;
            return true;
        }
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
        } else {
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

    fn das_piece(&mut self, piece: &mut TetPiece, direction: Direction, force: i32) -> i8 {
        let mut ret = 0;
        let original = piece.position;
        match direction {
            Direction::East => {
                for i in 0..force {
                    piece.position += Vec2(1, 0);
                    if self.does_collide(*piece) {
                        piece.position -= Vec2(1, 0);
                        ret = 1;
                        if original == piece.position {
                            ret = 2;
                        }
                        break;
                    }
                }
            }
            Direction::West => {
                for i in 0..force {
                    piece.position += Vec2(-1, 0);
                    if self.does_collide(*piece) {
                        piece.position -= Vec2(-1, 0);
                        ret = 1;
                        if original == piece.position {
                            ret = 2;
                        }
                        break;
                    }
                }
            }
            Direction::South => {
                for i in 0..force {
                    piece.position -= Vec2(0, 1);
                    if self.does_collide(*piece) {
                        ret = 1;
                        if original == piece.position {
                            ret = 2;
                        }
                        piece.position += Vec2(0, 1);
                        break;
                    }
                }
            }
            _ => {}
        }
        ret
    }
    fn can_place(&mut self, piece: TetPiece) -> bool {
        if self.does_collide(piece) {
            false
        } else {
            let mut test = piece.clone();
            let a = !self.apply_gravity(&mut test, 1);

            a
        }
    }
    fn apply_gravity(&mut self, piece: &mut TetPiece, force: i32) -> bool {
        piece.position -= Vec2(0, 1 * force);
        if self.does_collide(*piece) {
            piece.position += Vec2(0, 1 * force);
            false
        } else {
            true
        }
    }
    fn move_left(&mut self, piece: &mut TetPiece, amount: i32) -> bool {
        piece.position += Vec2(-amount, 0);
        if self.does_collide(*piece) {
            piece.position -= Vec2(-amount, 0);
            false
        } else {
            true
        }
    }
    fn move_right(&mut self, piece: &mut TetPiece, amount: i32) -> bool {
        piece.position += Vec2(amount, 0);
        if self.does_collide(*piece) {
            piece.position -= Vec2(amount, 0);
            false
        } else {
            true
        }
    }

    fn place(&mut self, piece: TetPiece) -> bool {
        let filled = self.get_filled_rows();
        let mut rows = Vec::new();
        for row in filled {
            rows.push(self.clear_row(row));

        }
        if (!self.can_place(piece)) {
            self.refill_rows(rows);
            return false;
        }
        for mino in piece.get_minos() {
            self.set_tile(
                mino.0.try_into().unwrap(),
                mino.1.try_into().unwrap(),
                piece.color() as u8,
            );
        }
        rows.reverse();
        self.refill_rows(rows);
        true
    }
    fn unplace(&mut self, piece: TetPiece) -> bool {
        let filled = self.get_filled_rows();
        let mut rows = Vec::new();
        for row in filled {
            rows.push(self.clear_row(row));

        }
        for mino in piece.get_minos() {
            self.clear_tile(mino.0.try_into().unwrap(), mino.1.try_into().unwrap());
        }
        self.refill_rows(rows);
        true
    }
    fn place_n_clear(&mut self, piece: TetPiece) -> ClearStruct {
        if self.place(piece) {
            let mut ret = self.get_filled_rows();
            for row in &ret {
                self.clear_row(*row);
            }
            ClearStruct::new(true, ret)
        } else {
            ClearStruct::new(false, Vec::new())
        }
    }
    fn get_filled_rows(&self) -> Vec<isize> {
        let mut rows = Vec::new();

        for i in (0..self.height).rev() {
            let mut filled = true;
            for j in 0..10 {
                let tile = self.tiles[(i * 10 + j) as usize];
                if tile == 0 {
                    filled = false;
                    break;
                }
            }
            if filled {
                rows.push(i as isize);
            }
        }
        rows
    }

    fn check_pc(&self) -> bool {
        let mut clone = self.clone();
        for row in self.get_filled_rows() {
            clone.clear_row(row);
        }
        let mut pc = true;
        for tile in self.tiles {
            if tile > 0 {
                pc = false;
                break;
            }
        }
        pc
    }

    fn check_t_spin(&self, piece: TetPiece) -> TSpinResult {
        let mut res = TSpinResult::NoSpin;
        if piece.color() == PieceColor::T {
            let x = piece.position.0 as isize;

            let y = piece.position.1 as isize;
            let mut tl = self.get_tile(x - 1, y + 1);
            let mut tr = self.get_tile(x + 1, y + 1);
            let mut bl = self.get_tile(x - 1, y - 1);
            let mut br = self.get_tile(x + 1, y - 1);
            if tl > 0 {
                tl = 1;
            }
            if tr > 0 {
                tr = 1;
            }
            if bl > 0 {
                bl = 1;
            }
            if br > 0 {
                br = 1;
            }
            if tl + tr + bl + br >= 3 {
                res = TSpinResult::MiniSpin;
            }
            match piece.rotation {
                Direction::North => {
                    if tl == 1 && tr == 1 {
                        res = TSpinResult::TSpin;
                    }
                }
                Direction::East => {
                    if tr == 1 && br == 1 {
                        res = TSpinResult::TSpin;
                    }
                }
                Direction::South => {
                    if bl == 1 && br == 1 {
                        res = TSpinResult::TSpin;
                    }
                }
                Direction::West => {
                    if tl == 1 && bl == 1 {
                        res = TSpinResult::TSpin;
                    }
                }
            }
        }
        res
    }

    fn clear_row(&mut self, row: isize) -> ClearedLine {
        let conv = row as usize;
        let mut ret = Vec::with_capacity(self.width.try_into().unwrap());
        for j in 0..self.width {
            ret.push(piece_color_from_int(self.get_tile(j, row)));
        }
        if row < self.height {
            for i in row..23 {
                for j in 0..10 {
                    let above = self.get_tile(j, i + 1);
                    self.clear_tile(j, i);
                    self.set_tile(j, i, above);
                }
            }
        }
        ClearedLine(row, ret)
    }
    
    fn refill_rows(&mut self, rows: Vec<ClearedLine>) {
        for line in rows {
            let row = line.0;
            if row < self.height {
                for i in (row..23).rev() {
                    for j in 0..10 {
                        self.set_tile(j, i + 1, self.get_tile(j, i));
                        self.clear_tile(j, i);
                    }
                }
            }
            for j in 0..10 {
                self.set_tile(j, row, line.1[j as usize] as u8);
            }
        }
    }
    
    fn place_clone(&self, piece: TetPiece) -> Self {
        let mut clne = self.clone();
        clne.place(piece);
        return clne;
    }
    
    fn to_gray(&mut self) {
        for ref mut tile in self.tiles {
            if *tile != 0 {
                *tile = 8;
            }
        }
    }
}
#[wasm_bindgen]
impl TetBoard {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TetBoard {
        let new_board = TetBoard {
            height: 24,
            width: 10,
            tiles: [0; 240],
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
        if x > -1 && y > -1 {
            // log(&format!(
            //     "tile at ({}, {}): {}",
            //     x,
            //     y,
            //     self.tiles[y as usize * 10 + x as usize]
            // ));
            // log(&format!("set_tile({}, {}, {})", x, y, value));
            // log(&format!("{:?}", self.tiles));
            // log(&format!("{}", self.get_tile(x, y)));
        }
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
        return TetBoard::from_4h_array(arr.to_vec().as_slice().try_into().unwrap());
    }
    #[wasm_bindgen(js_name = doesCollide)]
    pub fn js_does_collide(&mut self, piece: TetPiece) -> bool {
        self.does_collide(piece)
    }
    #[wasm_bindgen(js_name = inBounds)]
    pub fn js_in_bounds(&self, pos: Vec2) -> bool {
        return pos.0 > -1 && pos.0 < 10 && pos.1 > -1 && pos.1 < 24;
    }
    #[wasm_bindgen(js_name = rotatePiece)]
    pub fn js_rotate_piece(&mut self, piece: &mut TetPiece, rotation: u8) -> bool {
        self.rotate_piece(piece, rotation)
    }
    #[wasm_bindgen(js_name = "dasPiece")]
    pub fn js_das_piece(&mut self, piece: &mut TetPiece, direction: Direction, force: i32) -> i8 {
        self.das_piece(piece, direction, force)
    }
    #[wasm_bindgen(js_name = "canPlace")]
    pub fn js_can_place(&mut self, piece: TetPiece) -> bool {
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
        for i in 0..24 {
            let mut sub_arr = Array::new();
            for j in 0..10 {
                sub_arr.push(&JsValue::from(matrix[i][j]));
            }
            arr.push(&JsValue::from(sub_arr));
        }
        arr
    }
    #[wasm_bindgen(js_name = applyGravity)]
    pub fn js_apply_gravity(&mut self, piece: &mut TetPiece, force: i32) -> bool {
        self.apply_gravity(piece, force)
    }
    #[wasm_bindgen(js_name = moveLeft)]
    pub fn js_move_left(&mut self, piece: &mut TetPiece, amount: i32) -> bool {
        self.move_left(piece, amount)
    }
    #[wasm_bindgen(js_name = moveRight)]
    pub fn js_move_right(&mut self, piece: &mut TetPiece, amount: i32) -> bool {
        self.move_right(piece, amount)
    }
    #[wasm_bindgen(js_name = get_filled_rows)]
    pub fn js_get_filled_rows(&self) -> Vec<isize> {
        self.get_filled_rows()
    }
    #[wasm_bindgen(js_name = clearRow)]
    pub fn js_clear_row(&mut self, row: isize) {
        self.clear_row(row);
    }
    #[wasm_bindgen(js_name = unplace)]
    pub fn js_unplace(&mut self, piece: TetPiece) -> bool {
        self.unplace(piece)
    }
    #[wasm_bindgen(js_name = checkPC)]
    pub fn js_check_pc(&self) -> bool {
        self.check_pc()
    }
    #[wasm_bindgen(js_name = checkTSpin)]
    pub fn js_check_t_spin(&self, piece: TetPiece) -> TSpinResult {
        self.check_t_spin(piece)
    }
    #[wasm_bindgen(js_name = place)]
    pub fn js_place(&mut self, piece: TetPiece) -> bool {
        self.place(piece)
    }
    #[wasm_bindgen(js_name = placeNClear)]
    pub fn js_place_n_clear(&mut self, piece: TetPiece) -> ClearStruct {
        self.place_n_clear(piece)
    }
    #[wasm_bindgen(js_name = noColorString)]
    pub fn no_color_string(&self) -> String {
        let mut base = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
                let tile = self.tiles[((self.height - i - 1) * self.width + j) as usize];
                let tile_color = piece_color_from_int(tile);
                if tile == 8 {
                    base.write_str("X");
                } else {
                    base.write_char(piece_color_to_char(tile_color));
                }
            }
            base.write_char('\n');
        }
        base
    }
}

impl Display for TetBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let tile = self.tiles[((self.height - i - 1) * self.width + j) as usize];
                let tile_color = piece_color_from_int(tile);
                if tile == 8 {
                    f.write_str("X");
                } else {
                    f.write_str(&color_str(
                        tile_color,
                        String::from(piece_color_to_char(tile_color)),
                    ));
                }
            }
            f.write_char('\n');
        }
        return std::fmt::Result::Ok(());
    }
}
