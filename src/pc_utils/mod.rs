pub mod bitboard;
pub mod bitpiece;
pub mod placement_tree;
use core::fmt;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use placement_tree::PlacementTree;
use crate::board::{Board, TetBoard};
use crate::field::Field;
use crate::fumen::TetFumen;
use crate::gameplay::Action;
use crate::vec2::Vec2;
use crate::{math, piece, queue};
use crate::piece::{piece_color_to_char, Direction, PieceColor, TetPiece};
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
pub use bitboard::BitBoard;
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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
        rot = value.rotation as u8 & 0b11;
        //println!("x: {}, y: {}, rot: {}", x, y, rot);
        x <<= 7;
        y <<= 2;
        let mut ret = PiecePos(
            (x | y | rot as i32).try_into().unwrap(),
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
        Direction::from((self.0 & 0b11) as u8)
    }
}
pub struct PieceNode {
    val: PiecePos,
    actions: HashMap<u8, Box<PieceNode>>

}

impl BitBoard {
    pub fn get_piece_placements(&mut self, mut piece: TetPiece, height: u8) -> Vec<PiecePos> {
        let mut placements = Vec::new();
        piece.position.1 = (height) as i32;
        piece.position.0 = 1;
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
            seen[pos.0 as usize] = true;
            todo.push(pos);
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
            seen[pos.0 as usize] = true;
            todo.push(pos);
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
            // println!("{:?}", piece);

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
//         println!(
// "==================================
// Starting search of piece, {}
// ==================================",
//             piece_color_to_char(piece.color())
//         );
        let mut ret;
        let mut cleared = Vec::new();
        for row in self.get_filled_rows() {
            cleared.push(self.clear_row(row));
            pred_data.lines_cleared += 1;
        }
        // println!("{}", self);
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

            if piece.rotation as u8 + 1 > 3 {
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
            if piece.rotation as u8 + 1 > 3 {
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
        // println!("{:?}", piece);
        self.check_piece_placement_pred(&piece, &mut seen, &mut todo, &mut placements, data, pred);

        let mut i = 0;
        // print!("{:?}", todo.iter().map(|pos: &PiecePos| {
        //     (pos.get_pos(), pos.get_rot())
        // }));
        while todo.len() > 0 {
            if let Some(mut start_pos) = todo.pop() {
                i += 1;

                piece.set_piece_pos(start_pos);
                // println!("====================Searching====================\n{}", Field::new(self.clone(), Some(piece), None));
                if start_pos.get_pos() == Vec2(4,2) {
                    // println!("AAAAAAAAAAAAAAAAAAA{}", Field::new(self.clone(), Some(piece), None));
                }
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
    fn filled_mino_count(&self, height: usize) -> usize {
        let mut count = 0;
        for i in 0..height {
            for j in 0..self.width {
                if self.get_tile(j, i as isize) != 0 {
                    count += 1;
                }
            }
        }
        count
    }
    fn get_max_pieces(&self, height: usize) -> usize {
        let filled = self.filled_mino_count(height) as usize;
        let unfilled = height * self.width as usize - filled;
        unfilled / 4
    }
}
pub struct PathOptions {
    pub tetfu: String,
    pub patterns: String,
    pub height: usize,
    pub hold: bool,
    pub max_boards: usize
}
pub fn path_entry(
    options: PathOptions,
    output: &mut HashSet<TetBoard>
) -> i8 {
    let PathOptions {
        tetfu, 
        patterns, 
        height, 
        hold,
        max_boards
    } = options;
    let mut fum = TetFumen::load(tetfu);
    let pattern_str_vec = patterns.split(';')
        .collect::<Vec<_>>();
    let raw_pattern_vec = pattern_str_vec.iter()
        .map(|s| String::from(*s))
        .map(|mut s| s.chars().filter(|c| !c.is_whitespace()).collect())
        .map(|s| Queue::from_string(s))
        .map(|q| q.unwrap())
        .collect::<Vec<_>>();

    for i in 0..fum.len() {
        let page = fum.get_page_at(i);
        let board = page.get_field().board.clone();
        let pieces_remaining = board.get_max_pieces(height);
        
        for pattern in &raw_pattern_vec {
            println!("Current pattern: {:?}", pattern.head().node_type);
            if pattern.choose_count() > 0 {
                for queue in pattern.possible_q_iter() {
                    let queue_length = queue.len();
                    let mut max_depth = pieces_remaining;
                    if queue_length < pieces_remaining {
                        max_depth = queue_length;
                    }
                    let mut placement_tree = PlacementTree::new(None); 
                    path(
                        board,
                        queue.clone(), 
                        height,
                        hold,
                        output, 
                        max_boards, 
                        0, 
                        max_depth, 
                        Vec::new(),
                        &mut placement_tree
                    );
                }
            } else {
                let queue = pattern;
                let queue_length = queue.len();
                let mut max_depth = pieces_remaining;
                if queue_length < pieces_remaining {
                    max_depth = queue_length;
                }
                let mut placement_tree = PlacementTree::new(None); 
                path(
                    board,
                    queue.clone(), 
                    height,
                    hold,
                    output, 
                    max_boards, 
                    0, 
                    max_depth, 
                    Vec::new(),
                    &mut placement_tree
                );
            }
            
        }

    }

    return 0;
}
pub fn path(
    mut board: TetBoard,
    mut queue: Queue, 
    height: usize, 
    hold: bool, 
    ouptut: &mut HashSet<TetBoard>,
    max_boards: usize,
    depth: usize,
    max_depth: usize,
    mut used: Vec<PieceColor>,
    placement_tree: &mut PlacementTree
) {
    // println!("{} Cur queue: {}, used: {:?}", "-".repeat(depth*4), queue, used);
    let pieces_remaining = board.get_max_pieces(height);
    let last = depth == max_depth - 1;
    let pred = |data: PredData| data.piece.unwrap().get_minos().iter().all(|mino: &Vec2| mino.1 < (4 - data.lines_cleared).into());
    
    let pos = Vec2(0, (height + 2) as i32);
    let mut next = queue.take_next_piece().unwrap();
    let mut piece = TetPiece::from((next, pos));
    used.push(next);
    
    let mut placements = board.get_piece_placements(piece, Some(&pred));
    // println!("{:?} {:?}", piece, placements);
    if (!last) {
        
        for placement in placements {
            piece.set_piece_pos(placement);
            let new_board = board.place_clone(piece);
            path(
                new_board, 
                queue.clone(), 
                height, 
                hold, 
                ouptut,
                max_boards,
                depth + 1,
                max_depth,
                used.clone(),
                placement_tree
            );
        }
        
    } else {
        for placement in placements {
            piece.set_piece_pos(placement);
            ouptut.insert(board.place_clone(piece));
        }
    }
    
    queue.push_front(QueueNode::cpiece(next));
    used.pop();
    // println!("{} Pre hold branch queue: {}", "-".repeat(depth*4), queue);
    if hold && queue.len() > 1 {
        next = queue.pop_at(1).unwrap().piece();
        // println!("{} Hold branch, new queue: {}", "-".repeat(depth*4), queue);
        piece = TetPiece::from((next, pos));
        used.push(next);

        let mut placements = board.get_piece_placements(piece, Some(&pred));
        // println!("{:?} {:?}", piece, placements);
        if (!last) {
            for placement in placements {
                piece.set_piece_pos(placement);
                let new_board = board.place_clone(piece);
                path(
                    new_board, 
                    queue.clone(), 
                    height, 
                    hold, 
                    ouptut,
                    max_boards,
                    depth + 1,
                    max_depth,
                    used.clone(),
                    placement_tree
                );
            }
            
        } else {
            for placement in placements {
                piece.set_piece_pos(placement);
                ouptut.insert(board.place_clone(piece));
            }
        }
    }
}
use crate::queue::{Queue, QueueNode};

