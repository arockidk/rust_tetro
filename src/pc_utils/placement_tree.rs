use std::collections::HashMap;

use crate::{board::TetBoard, piece::PieceColor, queue::{self, Queue}};

use super::PiecePos;
pub struct PlacementNode {
    pub board: TetBoard,
    pub is_end: bool,
    t: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    i: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    l: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    j: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    o: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    s: Option<HashMap<PiecePos, Box<PlacementNode>>>,
    z: Option<HashMap<PiecePos, Box<PlacementNode>>>,

}
impl PlacementNode {
    pub fn new(board: TetBoard, is_end: bool) -> Self {
        PlacementNode {
            board,
            is_end,
            t: None,
            i: None,
            l: None,
            j: None,
            o: None,
            s: None,
            z: None,
        }
    }
    pub fn get_edge(&self, color: PieceColor) -> Option<&HashMap<PiecePos, Box<PlacementNode>>> {
        match color {
            PieceColor::T => self.t.as_ref(),
            PieceColor::I => self.i.as_ref(),
            PieceColor::L => self.l.as_ref(),
            PieceColor::J => self.j.as_ref(), 
            PieceColor::O => self.o.as_ref(),
            PieceColor::S => self.s.as_ref(),
            PieceColor::Z => self.z.as_ref(),
            _ => None
        }
    }
    pub fn get_edge_mut(&mut self, color: PieceColor) -> Option<&mut HashMap<PiecePos, Box<PlacementNode>>> {
        match color {
            PieceColor::T => self.t.as_mut(),
            PieceColor::I => self.i.as_mut(),
            PieceColor::L => self.l.as_mut(),
            PieceColor::J => self.j.as_mut(),
            PieceColor::O => self.o.as_mut(),
            PieceColor::S => self.s.as_mut(),
            PieceColor::Z => self.z.as_mut(),
            _ => None
        }
    }
    pub fn add_edge(&mut self, color: PieceColor, pos: PiecePos, node: Box<PlacementNode>) {
        if let Some(edge) = self.get_edge_mut(color) {
            edge.insert(pos, node);
        }
    }
    fn dfs(&self, queue: &Queue, path: Vec<PiecePos>, hold: bool) -> Option<Vec<PiecePos>> {
        if self.is_end {
            Some(path)
        } else {
            if queue.len() == 0 {
                return None;
            } else {
                let mut ret = None;
                if let Some(edge) 
                    = self.get_edge(queue.head().get_piece().unwrap()) {
                    for (k, v) in edge {
                        let mut new_path = path.clone();
                        new_path.push(k.clone());
                        let node = v;
                        if let Some(result) = node.dfs(queue, new_path, hold) {
                            ret = Some(result);
                            break;
                        }
                    }
                }
                if ret.is_none() && queue.len() > 1 {
                    if let Some(edge) 
                        = self.get_edge(queue.at(1).unwrap().get_piece().unwrap()) {
                        for (k, v) in edge {
                            let mut new_path = path.clone();
                            new_path.push(k.clone());
                            let node = v;
                            if let Some(result) = node.dfs(queue, new_path, hold) {
                                ret = Some(result);
                                break;
                            }
                        }
                    }
                }
                ret
            }
        }
    }
}
pub struct PlacementTree {
    head: Option<PlacementNode>
}
impl PlacementTree {
    pub fn new(head: Option<PlacementNode>) -> Self {
        PlacementTree {
            head
        }
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn find_first_solution(&self, queue: &Queue, hold: bool) -> Option<Vec<PiecePos>> { 
        if queue.choose_count() > 0 {
            panic!("Choose count should be 0, queue: {}", queue);
        } else {
            if let Some(head) = &self.head {
                head.dfs(queue, Vec::new(), hold)
            } else {
                None
            }
        }
    } 

}
