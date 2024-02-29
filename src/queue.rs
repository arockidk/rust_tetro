#![allow(unused_must_use)]
use fumen::Piece;
use wasm_bindgen::prelude::*;

use crate::{math::factorial, piece::{piece_color_to_char, PieceColor, TetPiece}};
use core::{fmt, panicking::panic};
use std::{collections::HashSet, fmt::Write};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum QueueNodeType {
    Choose,
    Piece
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[wasm_bindgen]
pub struct QueueNode {
    node_type: QueueNodeType,
    choose: Option<Choose>,
    piece: Option<PieceColor>,
    next: Option<Box<QueueNode>>
}
impl QueueNode {
    pub fn push_back(&mut self, node: QueueNode) {
        if let Some(next) = self.next {
            let mut cur = self;
            while cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next = Some(Box::new(node));
        } else {
            self.next = Some(Box::new(node));
        }
    }

    pub fn insert(&mut self, node: QueueNode) {
        if let Some(mut my_next) = self.next {

            let mut new = Box::new(node);
            if let Some(mut next) = new.next  {
                let mut cur = new;
                while cur.next.is_some() {
                    cur = *cur.next.as_ref().unwrap();
                }
                cur.next = Some(my_next);
            } 
            self.next = Some(new);
            

        } else {
            self.next = Some(Box::new(node));
        }
    }
    pub fn insert_at(&mut self, n: usize, node: QueueNode) {
        if let Some(mut next) = self.next {
            let mut next_ref = &mut next;
            for _ in 0..n {
                if next_ref.next.is_some() {
                    next_ref = next_ref.next.as_mut().unwrap();
                }
            }
            next_ref.next = Some(Box::new(node));
        }
    }
    pub fn pop(&mut self) -> Option<QueueNode> {
        if let Some(mut next ) = self.next {
            let mut next_ref = &mut self.next.unwrap();
            let mut cur_ref = self;
            while next_ref.next.is_some() {
                cur_ref = next_ref;
                next_ref = &mut cur_ref.next.unwrap();
            }
            let mut ret: QueueNode = next_ref.as_ref().clone();
            ret.next = None;
            cur_ref.next = None;
            Some(ret)
        } else {
            None
        }
        
    }
    pub fn pop_next(&mut self) -> Option<QueueNode> {
        if let Some(mut next) = self.next {
            if let Some(mut next_ref) = next.next {
                let mut ret = *next.clone();
                ret.next = None;
                self.next = Some(next_ref);
                Some(ret)
            } else {
                let mut ret = *next.clone();
                ret.next = None;
                self.next = None;
                Some(ret)
            }
        } else {
            None
        }
    }
    pub fn at(&self, index: usize) -> Option<QueueNode> {
        if index == 0 {
            return Some(self.clone());
        }
        if let Some(mut next) = self.next {
            let mut cur = Box::new(*self);
            for _ in 0..index {
                if cur.next.is_some() {
                    cur = cur.next.unwrap();
                }
            }
            Some(*cur.as_ref())

        } else {
            None
        }
    }
    pub fn mut_at(&mut self, index: usize) -> Option<&mut QueueNode> {
        if index == 0 {
            return Some(self)
        } 
        if let Some(mut next) = self.next {
            let mut cur = Box::new(*self);
            for _ in 0..index {
                if cur.next.is_some() {
                    cur = cur.next.unwrap();
                }
            }
            Some(cur.as_mut())
        } else {
            None
        }
        
    }
    pub fn last(&self) -> &QueueNode{
        if let Some(next) = self.next {
            let mut cur = self;
            while cur.next.is_some() {
                cur = cur.next.as_ref().unwrap();
            }
            cur
        } else {
            self
        }
    }
    pub fn last_mut(&mut self) -> &mut QueueNode{
        if let Some(next) = self.next {
            let mut cur = self;
            while cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
            }
            cur
        } else {
            self
        }
    }
    pub fn len(&self) -> usize {
        if let Some(next) = self.next {
            let mut cur = Box::new(*self);
            let mut len = 1;
            while cur.next.is_some() {
                cur = cur.next.unwrap();
                len += 1;
            }
            len
        } else {
            1
        }
    }
}

impl From<PieceColor> for QueueNode { 
    fn from(color: PieceColor) -> Self {
        QueueNode {
            node_type: QueueNodeType::Piece,
            piece: Some(PieceColor::from(color)),
            choose: None,
            next: None
        }
    }
}
#[derive(Clone)]
#[wasm_bindgen()]
pub struct Queue {
    head: Option<QueueNode>,
}
#[wasm_bindgen]
impl Queue {
    pub fn new() -> Queue {
        Queue {head: None}
    }
    pub fn push_js(&mut self, node: JsValue) {
        
        if let Ok(node) = serde_wasm_bindgen::from_value(node){
            self.push(node);
        }
        
    }
    pub fn pop_js(&mut self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let node = self.pop();
        
        serde_wasm_bindgen::to_value(&node)
    }
    pub fn get_queues(&self) -> Vec<Queue> {
        let queues: Vec<Queue> = Vec::new();
        let mut base_queue = Queue {pieces: Vec::new()};
        for node in &self.pieces {
            match node {
                QueueNode::Piece(_, _) => {
                    base_queue.push(node.clone());
                }
                QueueNode::Choose(_, _) => {
                    
                }
            }
        }
        queues
    }
    pub fn head(&self) -> QueueNode {
        if self.head.is_some() {
            self.head.unwrap()
        } else {
            panic!("Head node doesn't exist")
        }
    }
}
impl Queue {
    pub fn push(&mut self, node: QueueNode) {
        if let Some(mut head) = self.head {
            head.push_back(node);
        }
    }
    pub fn pop(&mut self) -> Option<QueueNode> { 
        if let Some(mut head) = self.head {
            head.pop()
        }
        else {
            None
        }
    }
    pub fn append(&mut self, queue: &mut Queue) {
        if let Some(mut head) = self.head {
            if let Some(mut other) = queue.head {
                head.push_back(other);
            }
        }
    }
    pub fn len(&self) -> usize {
        if let Some(head) = &self.head {
            head.len()
        } else {
            0
        }
    }
}
impl fmt::Display for Queue { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(mut head) = self.head {
            for i in 0..self.len() {
                
                if let Some(mut piece) = head.at(i).clone() {
                    match piece.node_type {
                        QueueNodeType::Piece => {
                            f.write_char(piece_color_to_char(piece.piece.unwrap()));
                        }
                        QueueNodeType::Choose => {
                            f.write_str(piece.choose.unwrap().to_string().as_str());
                        }
                    }
                    if i != self.len() - 1 {
                        f.write_char(',');
                        f.write_char(' ');
        
                    } 
                }
                
            }
        }

        
        fmt::Result::Ok(())
    }
}
#[derive(Clone)]
struct ChooseState {
    queue: Queue,
    choose: Choose
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Choose {
    pieces: Vec<PieceColor>,
    count: usize,
    inverse: bool
}
#[derive(Debug)]
pub struct InvalidQueueFormatError {}
impl std::error::Error for InvalidQueueFormatError {}
impl fmt::Display for InvalidQueueFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid queue format")   
    }
}
impl Choose {
    pub fn new(pieces: Vec<PieceColor>, count: usize, inverse: bool) -> Result<Choose, InvalidQueueFormatError> {
        let mut choose = Choose {
            pieces: Vec::new(),
            count,
            inverse
        };
        for piece in pieces {
            if !choose.pieces.contains(&piece) {
                choose.pieces.push(piece);
            } else {
                return Err(InvalidQueueFormatError {});
            }
        }
        Ok(choose)
    }
    pub fn size(&self) -> usize {
        (
            factorial(self.pieces.len().try_into().unwrap()) 
                            / 
            factorial((self.pieces.len() - self.count).try_into().unwrap())
        ).try_into().unwrap()   
    }
    pub fn get_queues(&self) -> Vec<Queue> {
         
        let mut ret: Vec<Queue> = Vec::new();
        let state = ChooseState {
            queue: Queue::new(),
            choose: self.clone()
        };
        for i in 0..self.pieces.len() {
            
            let piece = state.choose.pieces[i];
            let mut state_clone = state.clone();
            let mut new = QueueNode {   
                node_type: QueueNodeType::Piece,
                choose: None,
                piece: Some(piece),
                next: None
            };
            let last = state_clone.queue.head().last_mut();
            match last.node_type {
                QueueNodeType::Piece => {
                    last.next = Some(Box::new(new));
                }
                _ => (),
                
            }
            state_clone.queue.push(new);
            state_clone.choose.count -= 1;
            state_clone.choose.pieces.remove(i);
            println!("{:?}", state_clone.choose.pieces);
            ret.append(&mut self.get_queues0(state_clone));
            
            
        }

        ret
    }
    fn get_queues0(&self, state: ChooseState) -> Vec<Queue>{
        let mut ret: Vec<Queue> = Vec::new();

        for i in 0..state.choose.pieces.len() {
            let piece = state.choose.pieces[i];
            let mut state_clone = state.clone();
            let mut new = QueueNode {   
                node_type: QueueNodeType::Piece,
                choose: None,
                piece: Some(piece),
                next: None
            };
            let last = state_clone.queue.head().last_mut();
            match last.node_type {
                QueueNodeType::Piece => {
                    last.next = Some(Box::new(new));
                }
                _ => (),
                
            }
            state_clone.queue.push(new);
            state_clone.choose.count -= 1;
            state_clone.choose.pieces.remove(i);
            if state_clone.choose.count == 0 {
                ret.push(state_clone.queue);
            } else {
                ret.append(&mut self.get_queues0(state_clone));
            }
            
        }
        ret
    }
    pub fn from_string(mut s: String) -> Result<Self, InvalidQueueFormatError> {
        let mut pieces = Vec::new();
        let mut count = 0;
        let mut inverse = false;
        s.retain(|c| !c.is_whitespace());
        let str = s.as_str();
        if let Some(first) = str.chars().nth(0) {
            if first == '*' {
                // All set
                pieces.append(&mut Vec::from(crate::piece::get_pieces()));
                let c = str.chars().nth(1).unwrap();
                if c == 'p' {
                    let n = str.chars().nth(2).unwrap();
                    count = n.to_digit(10).unwrap() as usize;
                } else if c == '!' {
                    count = 7;
                } else {
                    return Err(InvalidQueueFormatError {});
                }
            } else if first == '[' {
                if !s.contains(']') {
                    return Err(InvalidQueueFormatError {});
                }
                let mut i = 0;
                let mut set_ended = false;
                while i < str.len() && str.chars().nth(i).unwrap() != ']' {
                    i += 1;
                    let c = str.chars().nth(i).unwrap();
                    if c == '^' {
                        inverse = true;
                    }
                    if c == ']' {
                        set_ended = true;
                        i += 1;
                        break;
                    }
                    if c == 'T' || c == 'I' || c == 'J' || c == 'L' || c == 'O' || c == 'S' || c == 'Z' {
                        pieces.push(PieceColor::from(c));
                        count += 1;
                    }
                }
                if !set_ended {
                    return Err(InvalidQueueFormatError {});
                } 
                let c = str.chars().nth(i).unwrap();
                if c == 'p' {
                    let n = str.chars().nth(i + 1).unwrap();
                    let n = n.to_digit(10).unwrap();
                    count = n as usize;
                }

            } else {
                return Err(InvalidQueueFormatError {});
            }
        }
        Ok(Choose {
            pieces,
            count,
            inverse
        })
    }
}


impl fmt::Display for Choose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        if self.pieces.len() == 7 {
            str.write_str("*");
        } else {
            if self.inverse { 
                str.write_str("[^");
            } else {
                str.write_str("[");
            }
            for piece in &self.pieces { 
                str.write_char(piece_color_to_char(piece.clone()));
            }
            str.write_str("]");
        }
        str.write_str("p");
        str.write_str(self.count.to_string().as_str());
        f.write_str(str.as_str());
        fmt::Result::Ok(())   
    }
}

pub fn choose(pieces: Vec<PieceColor>, count: usize, inverse: bool) -> Vec<Queue> {
    (Choose {
        pieces,
        count,
        inverse
    }).get_queues()
}
impl From<String> for Queue {
    fn from(s: String) -> Self {
        Queue {pieces: Vec::new()}
    }
}
