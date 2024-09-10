use std::{fs::create_dir, path::Display};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    board::{Board, ClearStruct, ClearedLine, TSpinResult, TetBoard},
    kicks::{get_180_kicks, get_kicks},
    piece::{piece_color_from_int, Direction, PieceColor, TetPiece},
    queue::{Queue, QueueNode},
    vec2::Vec2,
};
#[wasm_bindgen]
/**
 * lowest 40 bits represent 10x4 board state
 * example:
 * 0b0000001111_0000000111_0000011111_0000001111
 * on multiple lines this will look like
 * 0b0000001111
 *   0000000111
 *   0000011111
 *   0000000111
 *
 */
pub struct u64_board(u64);
impl u64_board {
    pub fn from_board(board: TetBoard) -> u64_board {
        let matrix = board.get_tile_matrix();
        let mut n = 0;
        for i in 0..4 {
            for j in 0..10 {
                n |= matrix[3 - i][j] as u64;
                n <<= 1;
            }
        }
        u64_board(n)
    }
    pub fn in_bounds(&self, position: Vec2) -> bool {
        position.0 >= 0 && position.0 < 10 && position.1 >= 15 && position.1 < 24
    }

    pub fn as_matrix(&self) -> [[u8; 10]; 4] {
        let mut base = [[0; 10]; 4];
        for i in 0..4 {
            for j in 0..10 {
                base[3 - i][9 - j] = ((self.0 << (i * 10 + j)) & 0b1) as u8;
            }
        }
        base
    }
    pub fn as_array(&self) -> [u8; 40] {
        let mut base = [0; 40];
        for i in 0..40 {
            base[39 - i] = ((self.0 << i) & 0b1) as u8;
        }
        base
    }
    pub fn as_board(&self) -> TetBoard {
        TetBoard::from_4h_array(self.as_array())
    }
}
impl Board for u64_board {
    fn get_tile_array(&self) -> [u8; 240] {
        let mut arr = [0; 240];
        for i in 15..24 {
            for j in 0..10 {
                arr[i * 10 + j] = (self.0 >> (i * 10 + j) & 0b1) as u8;
            }
        }
        arr
    }
    fn get_tile_matrix(&self) -> [[u8; 10]; 24] {
        let mut matrix: [[u8; 10]; 24] = [[0; 10]; 24];
        let mut array = self.get_tile_array();
        for y in 0..24 {
            for x in 0..10 {
                matrix[y][x] = array[y * 10 + x];
            }
        }
        return matrix;
    }
    fn from_int_array(arr: [u8; 240]) -> u64_board {
        let mut new_board = u64_board(0);
        for i in 160..240 {
            new_board.0 |= (arr[i] as u64);
            new_board.0 <<= 1;
        }

        return new_board;
    }
    fn from_4h_array(arr: [u8; 40]) -> u64_board {
        let mut base: u64 = 0;
        for i in 0..4 {
            for j in 0..10 {
                base += arr[(3 - i) * 10 + j] as u64;
                base <<= 1;
            }
        }
        u64_board(base)
    }
    fn tile_occupied(&self, x: isize, y: isize) -> bool {
        self.get_tile(x, y) != 0
    }

    fn clear_tile(&mut self, x: isize, y: isize) {
        self.0 &= !(1 << (x + y * 10));
    }

    fn in_bounds(&self, pos: Vec2) -> bool {
        return pos.0 > -1 && pos.0 < 10 && pos.1 > -1 && pos.1 < 24;
    }
    fn rotate_piece(&mut self, piece: &mut TetPiece, rotation: u8) -> bool {
        let mut test_piece = piece.clone();
        let mod_rot = rotation % 4;
        let old_rot: usize = piece.rotation as usize;
        let new_rot = (piece.rotation + mod_rot as i64) % 4;
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
        } else {
            let kicks = get_kicks(*piece);
            let mut passed_tests = true;
            for i in 0..5 {
                let old_offset = kicks[old_rot][i];
                let new_offset = kicks[new_rot as usize][i];

                let shift: Vec2 = kicks[old_rot][i] - kicks[new_rot as usize][i];
                test_piece.position += shift;

                // print!("===========NEW ROT===========\n");
                // println!("Old offset: {:?}, New offset: {:?}", old_offset, new_offset);
                // println!("Attempting to rotate with offset {:?}", shift);
                // println!("{:?}", Vec2(10,23) - test_piece.position);
                // println!("{}", field::Field::new(*self, test_piece));
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
                for i in 0..11 {
                    piece.position += Vec2(1, 0);
                    if self.does_collide(*piece) {
                        ret = 1;
                        if original == piece.position {
                            ret = 2;
                        }
                        piece.position -= Vec2(1, 0);
                        break;
                    }
                }
            }
            Direction::West => {
                for i in 0..11 {
                    piece.position += Vec2(-1, 0);
                    if self.does_collide(*piece) {
                        ret = 1;
                        if original == piece.position {
                            ret = 2;
                        }
                        piece.position -= Vec2(-1, 0);
                        break;
                    }
                }
            }
            Direction::South => {
                for i in 0..23 {
                    if self.does_collide(*piece) {
                        ret = 1;
                        if original == piece.position {
                            ret = 2;
                        }
                        piece.position += Vec2(0, 1);
                        break;
                    }
                    piece.position -= Vec2(0, 1);
                }
            }
            _ => {}
        }
        ret
    }
    fn apply_gravity(&mut self, piece: &mut TetPiece, force: i32) -> bool {
        piece.position += Vec2(0, 1 * force);
        if self.does_collide(*piece) {
            piece.position -= Vec2(0, 1 * force);
            false
        } else {
            true
        }
    }

    fn get_tile(&self, x: isize, y: isize) -> u8 {
        if self.in_bounds(Vec2(x.try_into().unwrap(), y.try_into().unwrap())) {
            ((self.0 >> (4 + x + y * 10)) & 1) as u8
        } else {
            1
        }
    }

    fn set_tile(&mut self, x: isize, y: isize, new: u8) {
        if self.in_bounds(Vec2(x.try_into().unwrap(), y.try_into().unwrap())) {
            self.0 = (self.0 & !(1 << (4 + x + y * 10))) | ((new as u64) << (4 + x + y * 10));
        }
    }

    fn does_collide(&mut self, piece: TetPiece) -> bool {
        let minos = piece.get_minos();
        if piece.position.1 > 6 {
            return true;
        } else {
            for mino in minos {
                if self.get_tile(mino.0.try_into().unwrap(), mino.1.try_into().unwrap()) == 1 {
                    return true;
                }
            }
        }
        return false;
    }
    fn can_place(&mut self, piece: TetPiece) -> bool {
        if self.does_collide(piece) {
            false
        } else {
            let mut test = piece.clone();
            test.apply_gravity(1);
            self.does_collide(test)
        }
    }

    fn move_left(&mut self, piece: &mut TetPiece, amount: i32) -> bool {
        piece.position += Vec2(-1 * amount, 0);
        if self.does_collide(*piece) {
            piece.position -= Vec2(-1 * amount, 0);
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
        if (!self.can_place(piece)) {
            return false;
        }
        for mino in piece.get_minos() {
            self.set_tile(
                mino.0.try_into().unwrap(),
                mino.1.try_into().unwrap(),
                piece.color() as u8,
            );
        }
        true
    }
    fn unplace(&mut self, piece: TetPiece) -> bool {
        for mino in piece.get_minos() {
            self.clear_tile(
                mino.0.try_into().unwrap(),
                mino.1.try_into().unwrap(),
            );
        }
        true
    }
    fn place_n_clear(&mut self, piece: TetPiece) -> ClearStruct {
        if self.place(piece) {
            let mut ret = self.get_filled_rows();
            ClearStruct::new(true, ret)
        } else {
            ClearStruct::new(false, Vec::new())
        }
    }
    fn get_filled_rows(&self) -> Vec<isize> {
        let truncated = self.0 & 0b1111111111_1111111111_1111111111_1111111111;
        let mut ret = Vec::new();
        let row_mask = 0b1111111111;
        for i in 0..4 {
            let row = (truncated >> (i * 10)) & row_mask;
            if row > 0 {
                ret.push(i);
            }
        }
        ret
    }
    
    fn check_pc(&self) -> bool {
        self.0 == 0
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
        let mut ret = Vec::with_capacity(10);
        if row < 4 {
            for i in row..4 {
                for j in 0..10 {
                    let above = self.get_tile(j, i + 1);
                    ret.push(piece_color_from_int(self.get_tile(j, i)));
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
            if row < 4 {
                for i in (row..4).rev() {
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
        let mut clne = Self(self.0);
        clne.place(piece);
        return clne;
    }

}
impl std::fmt::Display for u64_board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_board())
    }
}
