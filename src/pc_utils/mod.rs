mod u64board;
use core::fmt;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::board::{Board, TetBoard};
use crate::field::Field;
use crate::gameplay::Action;
use crate::vec2::Vec2;
use crate::{piece, queue};
use crate::piece::{Direction, PieceColor, TetPiece};
impl Queue {
    /**
     * Get columnar parity without T pieces.
     */
    pub fn get_columnar_parity(&self) -> u8 {
        let mut count = 0;
        for node in self.iter() {
            match node.node_type {
                queue::QueueNodeType::Piece => {
                    match node.piece() {
                        PieceColor::L => {
                            count += 1;
                        }
                        PieceColor::J => {
                            count += 1;
                        }
                        _ => {}
                    }
                }
                _ => {}
            } 
        }
        count
    } 
    pub fn t_count(&self) -> u8 {
        let mut count = 0;
        for node in self.iter() {
            match node.node_type {
                queue::QueueNodeType::Piece => {
                    match node.piece() {
                        PieceColor::T => {
                            count += 1;
                        }
                        _ => {}
                    }
                }
                _ => {}
            } 
        }
        count
    }
}
pub use u64board::u64_board;
#[derive(Clone, Copy)]
pub struct PiecePos(u16);
impl PiecePos {
    pub fn set_pos(&mut self, val: Vec2) {
        let (x, y) = (val.0, val.1);
        self.set_x(x);
        self.set_y(y);
    }
    pub fn set_x(&mut self, val: i32) {
        self.0 &= (0b00001111111);
        self.0 |= ((val & 0b1111) << 7) as u16;
    }
    pub fn set_y(&mut self, val: i32) {
        self.0 &= (0b11110000011);
        self.0 |= ((val & 0b11111) << 2) as u16;
    }
    pub fn set_rot(&mut self, val: i32) {
        self.0 &= (0b11111111100);
        self.0 |= (val & 0b11) as u16;
    }
    pub fn get_pos(&self) -> Vec2 {
        self.clone().into()
    }
    pub fn get_rot(&self) -> Direction {
        self.clone().into()
    }
}
impl fmt::Display for PiecePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.write_str(format!("{:?} {:?}", self.get_pos(), self.get_rot()).as_str())
    }
}
impl Debug for PiecePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}
impl From<TetPiece> for PiecePos {
    fn from(value: TetPiece) -> Self {
        let (mut x, mut y, mut rot) = (0, 0, 0);
        x = value.position.0 & 0b1111;
        y = value.position.1 & 0b11111;
        rot = value.rotation.to_i32() & 0b11;
        //println!("x: {}, y: {}, rot: {}", x, y, rot);
        x <<= 7;
        y <<= 2;
        let mut ret = PiecePos(
            (x | y | rot).try_into().unwrap(),
        );
        ret
    }
}
impl Into<Vec2> for PiecePos {
    fn into(self) -> Vec2 {
        Vec2(((self.0 >> 7) & 0b1111).into(), ((self.0 >> 2) & 0b11111).into())
    }
}
impl Into<Direction> for PiecePos {
    fn into(self) -> Direction {
        Direction::from_int((self.0 & 0b11).into())
    }
}
pub struct PieceNode {
    val: PiecePos,
    actions: HashMap<u8, Box<PieceNode>>

}
impl u64_board {
    pub fn get_piece_placements(&mut self, mut piece: TetPiece, height: u8) -> Vec<PiecePos> {
        let mut placements = Vec::new();
        piece.position.1 = (height) as i32;
        piece.position.0 = 1;
        self.das_piece(&mut piece, Direction::South, 1000);
        println!("{}", Field::new(self.as_board(), Some(piece), None));

        placements

    }
}
impl TetPiece {
    pub fn set_piece_pos(&mut self, pos: PiecePos) {
        self.position = pos.into();
        self.rotation = pos.into();
    }
}
#[derive(Clone, Copy)]
pub struct PredData {
    pub lines_cleared: u8,
    pub piece: Option<TetPiece>
}
impl TetBoard {
    pub fn check_piece_placement(&mut self, piece: &TetPiece, seen: &mut [bool; 16 * 32 * 4], todo: &mut Vec<PiecePos>, placements: &mut Vec<PiecePos>) {
        if !self.does_collide(*piece) {
            let pos = PiecePos::from(*piece);
            if seen[pos.0 as usize] {
                return
            }
            match piece.color() {
                PieceColor::Z | PieceColor::S => {
                    // println!("{:?} {} {} {} {}", piece, pos.0 & !0b11, pos.0 & 0b11, ((pos.0 + 2) % 4), ((pos.0 & !0b11) + ((pos.0 + 2) % 4)));
                    let mut idx = pos.0;
                            let offset = ((pos.0 + 2) % 4);
                    idx &= !0b11;
                    idx += offset;
                    match pos.0 % 4 {
                        0 => {
                            idx += 1 << 2;
                        },
                        1 => {
                            idx += 1 << 7;
                        },
                        2 => {
                            idx -= 1 << 2;
                        },
                        3 => {
                            idx -= 1 << 7;
                        },
                        _ => {}
                    }
                    if seen[idx as usize] {
                        return
                    }
                },
                PieceColor::I => {
                    let mut idx = pos.0;
                    let offset = ((pos.0 + 2) % 4);
                    idx &= !0b11;
                    idx += offset;
                    let mut out_of_bounds = false;
                    match pos.0 % 4 {
                        0 => {
                            idx +=      0b00010000000;
                            if ((idx &   0b11110000000) >> 7) > 9 {
                                out_of_bounds = true;
                            }
                        },
                        1 => {

                            if (idx &   0b00001111100) == 0 {
                                out_of_bounds = true
                            } else {
                                idx -= 1 << 2;
                            }
                        },
                        2 => {
                            if (idx &   0b11110000000) == 0 {
                                out_of_bounds = true
                            } else {
                                idx -= 1 << 7;
                            }
                        },
                        3 => {
                            idx +=       0b00000000100;
                            if ((idx &   0b00001111100) >> 2) > 19 {
                                out_of_bounds = true;
                            }
                        },
                        _ => {}
                    }
                    if !out_of_bounds {
                        if seen[idx as usize] {
                            return
                        }
                    }
                }
                PieceColor::O => {
                    if seen[((pos.0 & !0b11) + ((pos.0 + 1) % 4)) as usize] {
                        return;
                    }
                    if seen[((pos.0 & !0b11) + ((pos.0 + 2) % 4)) as usize] {
                        return;
                    }
                    if seen[((pos.0 & !0b11) + ((pos.0 + 3) % 4)) as usize] {
                        return;
                    }
                },
                _ => ()
            }
            seen[pos.0 as usize] = true;
            todo.push(pos);
            if self.can_place(*piece) { 
                placements.push(pos);
            }
        }
    }
    pub fn check_piece_placement_pred(&mut self, piece: &TetPiece, seen: &mut [bool; 16 * 32 * 4], todo: &mut Vec<PiecePos>, placements: &mut Vec<PiecePos>, data: &mut PredData, pred: &impl Fn(PredData) -> bool) {
        if !self.does_collide(*piece) {
            let pos = PiecePos::from(*piece);
            if seen[pos.0 as usize] {
                return
            }
            let mut out_of_bounds = false;
            match piece.color() {
                PieceColor::Z | PieceColor::S => {
                    //println!("{:?} {} {} {} {}", piece, pos.0 & !0b11, pos.0 & 0b11, ((pos.0 + 2) % 4), ((pos.0 & !0b11) + ((pos.0 + 2) % 4)));
                    let mut idx = pos.0;
                            let offset = ((pos.0 + 2) % 4);
                    idx &= !0b11;
                    idx += offset;

                    match pos.0 % 4 {
                        0 => {
                            idx +=      0b00000000100;
                            if ((idx &   0b00001111100) >> 2) > 19 {
                                out_of_bounds = true;
                            }
                        },
                        1 => {
                            idx +=      0b00010000000;
                            if ((idx &   0b11110000000) >> 7) > 9 {
                                out_of_bounds = true;
                            }
                        },
                        2 => {
                            if (idx &   0b00001111100) == 0 {
                                out_of_bounds = true
                            } else {
                                idx -= 1 << 2;
                            }
                        },
                        3 => {
                            if (idx &   0b11110000000) == 0 {
                                out_of_bounds = true
                            } else {
                                idx -= 1 << 7;
                            }

                        },
                        _ => {}
                    }
                    if !out_of_bounds {
                        if seen[idx as usize] {
                            return
                        }
                    }

                },
                PieceColor::I => {
                    let mut idx = pos.0;
                    let offset = ((pos.0 + 2) % 4);
                    idx &= !0b11;
                    idx += offset;

                    match pos.0 % 4 {
                        0 => {
                            idx +=      0b00010000000;
                            if ((idx &   0b11110000000) >> 7) > 9 {
                                out_of_bounds = true;
                            }
                        },
                        1 => {

                            if (idx &   0b00001111100) == 0 {
                                out_of_bounds = true
                            } else {
                                idx -= 1 << 2;
                            }
                        },
                        2 => {
                            if (idx &   0b11110000000) == 0 {
                                out_of_bounds = true
                            } else {
                                idx -= 1 << 7;
                            }
                        },
                        3 => {
                            idx +=       0b00000000100;
                            if ((idx &   0b00001111100) >> 2) > 19 {
                                out_of_bounds = true;
                            }
                        },
                        _ => {}
                    }
                    if !out_of_bounds {
                        if seen[idx as usize] {
                            return
                        }
                    }
                }
                PieceColor::O => {
                    if seen[((pos.0 & !0b11) + ((pos.0 + 1) % 4)) as usize] {
                        return;
                    }
                    if seen[((pos.0 & !0b11) + ((pos.0 + 2) % 4)) as usize] {
                        return;
                    }
                    if seen[((pos.0 & !0b11) + ((pos.0 + 3) % 4)) as usize] {
                        return;
                    }
                },
                _ => ()
            }
            seen[pos.0 as usize] = true;
            todo.push(pos);
            data.piece = Some(*piece);
            if self.can_place(*piece) && pred(*data) { 
                placements.push(pos);
            }

        }
    }
    pub fn get_piece_placements(&mut self, mut piece: TetPiece, pred: Option<&impl Fn(PredData) -> bool>) -> Vec<PiecePos>{
        let mut pred_data = PredData {
            lines_cleared: 0,
            piece: None
        };
        let mut ret;
        let mut cleared = Vec::new();
        for row in self.get_filled_rows() {
            cleared.push(self.clear_row(row));
            pred_data.lines_cleared += 1;
        }
        if pred.is_none() {
            ret = self.get_piece_placements0(piece);
        } else {
            ret = self.get_piece_placements_pred(piece, &mut pred_data, pred.unwrap());
        }
        cleared.reverse();
        self.refill_rows(cleared);
        ret
    }
    fn get_piece_placements0(&mut self, mut piece: TetPiece) -> Vec<PiecePos> {
        let mut placements = Vec::new();
        let mut todo = Vec::new();
        let mut seen = [false; 16 * 32 * 4];
        let mut start_pos = PiecePos::from(piece);
        piece.set_piece_pos(start_pos);
        while self.does_collide(piece) {

            if piece.rotation.to_i8() + 1 > 3 {
                piece.rotation = Direction::North;
                piece.position.0 += 1;
                if piece.position.0 > 9 {
                    piece.position.1 += 1;
                    piece.position.0 = 0;
                }
            } else {
                piece.rotation += 1;
            }

        }
        self.check_piece_placement(&piece, &mut seen, &mut todo, &mut placements);

        let mut i = 0;
        while todo.len() > 0 {
            if let Some(mut start_pos) = todo.pop() {
                i += 1;

                piece.set_piece_pos(start_pos);
    //         println!("{} {}aaaaaa", Field::new(self.clone(), Some(piece), None), start_pos);
                piece.move_left(1);
                self.check_piece_placement(&piece, &mut seen, &mut todo, &mut placements);
                piece.set_piece_pos(start_pos);
                piece.move_right(1);
                self.check_piece_placement(&piece, &mut seen, &mut todo, &mut placements);
                piece.set_piece_pos(start_pos);
                piece.apply_gravity(1);
                self.check_piece_placement(&piece, &mut seen, &mut todo, &mut placements);
                piece.set_piece_pos(start_pos);
                self.rotate_piece(&mut piece, 1);
                self.check_piece_placement(&piece, &mut seen, &mut todo, &mut placements);
                piece.set_piece_pos(start_pos);
                self.rotate_piece(&mut piece, 2);
                self.check_piece_placement(&piece, &mut seen, &mut todo, &mut placements);
                piece.set_piece_pos(start_pos);
                self.rotate_piece(&mut piece, 3);
                self.check_piece_placement(&piece, &mut seen, &mut todo, &mut placements);

            }
        }

        placements 
    }
    fn get_piece_placements_pred(&mut self, mut piece: TetPiece, data: &mut PredData, pred: &impl Fn(PredData) -> bool) -> Vec<PiecePos> {
        let mut placements = Vec::new();
        let mut todo = Vec::new();
        let mut seen = [false; 32 * 16 * 4];
        let mut start_pos = PiecePos::from(piece);
        piece.set_piece_pos(start_pos);
        while self.does_collide(piece) {
            if piece.rotation.to_i8() + 1 > 3 {
                piece.rotation = Direction::North;
                piece.position.0 += 1;
                if piece.position.0 > 9 {
                    piece.position.1 += 1;
                    piece.position.0 = 0;
                }
            } else {
                piece.rotation += 1;
            }

        }

        piece.move_left(1);
        self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);
        piece.set_piece_pos(start_pos);
        piece.move_right(1);
        self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);
        piece.set_piece_pos(start_pos);
        piece.apply_gravity(1);
        self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);
        piece.set_piece_pos(start_pos);
        self.rotate_piece(&mut piece, 1);
        self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);
        piece.set_piece_pos(start_pos);
        self.rotate_piece(&mut piece, 2);
        self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);
        piece.set_piece_pos(start_pos);
        self.rotate_piece(&mut piece, 3);
        self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);

        let mut i = 0;
        while todo.len() > 0 {
            if let Some(mut start_pos) = todo.pop() {
                i += 1;

                piece.set_piece_pos(start_pos);
    //         println!("{} {}aaaaaa", Field::new(self.clone(), Some(piece), None), start_pos);
                piece.move_left(1);
                self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);
                piece.set_piece_pos(start_pos);
                piece.move_right(1);
                self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);
                piece.set_piece_pos(start_pos);
                piece.apply_gravity(1);
                self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);
                piece.set_piece_pos(start_pos);
                self.rotate_piece(&mut piece, 1);
                self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);
                piece.set_piece_pos(start_pos);
                self.rotate_piece(&mut piece, 2);
                self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);
                piece.set_piece_pos(start_pos);
                self.rotate_piece(&mut piece, 3);
                self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);

            }

        }

        placements 
    }
}
pub fn path(mut board: TetBoard, mut queue: Queue, height: u8, can_hold: bool, end_boards: &mut Vec<TetBoard>) {
    if can_hold && queue.len() > 1 {
        let mut q1 = queue.clone();
        let mut q2 = queue.clone();
        let mut p1 = TetPiece::new(
            q1.take_next_piece().unwrap(), 
            Direction::North, 
            Vec2(0, (height + 2).into())
        );
        let mut p2 = TetPiece::new(
            q2.take_next_piece().unwrap(), 
            Direction::North, 
            Vec2(0, (height + 2).into())
        );
        let pred = |data: PredData| data.piece.unwrap().get_minos().iter().all(|mino: &Vec2| mino.1 < (4 - data.lines_cleared).into());
        let placements1 = board.get_piece_placements(p1, Some(&pred));
        let placements2 = board.get_piece_placements(p2, Some(&pred));
        for placement in placements1 {
            p1.set_piece_pos(placement);
            let new_board = board.place_clone(p1);

            path(new_board, q1, height, can_hold, end_boards);
        }
        
        for placement in placements2 {
            p2.set_piece_pos(placement);
            let new_board = board.place_clone(p2);

            path(new_board, q2.clone(), height, can_hold, end_boards);
        }
    } else {
        let mut piece = TetPiece::new(
            queue.take_next_piece().unwrap(), 
            Direction::North, 
            Vec2(0, (height + 2).into())
        );
        let pred = |data: PredData| data.piece.unwrap().get_minos().iter().all(|mino: &Vec2| mino.1 < (4 - data.lines_cleared).into());
        let placements = board.get_piece_placements(piece, Some(&pred));

        if (queue.len() > 0) {
            for placement in placements {
                piece.set_piece_pos(placement);
                let new_board = board.place_clone(piece);

                path(new_board, queue.clone(), height, can_hold, end_boards);
            }
        } else {
            for placement in placements {
                piece.set_piece_pos(placement);
                end_boards.push(board.place_clone(piece));
            }
        }
    }
    

}
use crate::queue::Queue;

