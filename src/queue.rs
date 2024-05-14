#![allow(unused_must_use)]
use fumen::Piece;
use wasm_bindgen::prelude::*;

use crate::{math::{factorial, usize_factorial}, piece::{is_piece_color, piece_color_to_char, PieceColor, TetPiece}};
use core::fmt;
use std::{collections::{HashMap, HashSet}, fmt::Write, io::Cursor, iter::{self, Map}};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[wasm_bindgen]
pub enum QueueNodeType {
    Choose,
    Piece
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[wasm_bindgen]
pub struct QueueNode {
    pub node_type: QueueNodeType,
    choose: Option<Choose>,
    piece: Option<PieceColor>,
    next: Option<Box<QueueNode>>
}
#[wasm_bindgen]
impl QueueNode {

    #[wasm_bindgen(constructor)]
    pub fn js_new(
        node_type: QueueNodeType,
        choose: JsValue,
        piece: JsValue,
        next: JsValue
    ) -> QueueNode {
        Self::new(
            node_type,
            serde_wasm_bindgen::from_value(choose).unwrap(),
            serde_wasm_bindgen::from_value(piece).unwrap(),
            serde_wasm_bindgen::from_value(next).unwrap()
        )
    }
    

    #[wasm_bindgen(getter = choose)]
    pub fn js_choose(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.choose).unwrap() 
    }

    #[wasm_bindgen(getter)]
    pub fn piece(&self) -> PieceColor {
        self.piece.unwrap()
    }
    

}
impl QueueNode {
    
    pub fn new(node_type: QueueNodeType,
        choose: Option<Choose>,
        piece: Option<PieceColor>,
        next: Option<Box<QueueNode>>
    ) -> QueueNode {
        QueueNode {
            node_type,
            choose,
            piece,
            next
        }
    }
    pub fn choose(&self) -> Choose {
        self.choose.clone().unwrap()
    }
    pub fn get_choose(&self) -> Option<Choose> {
        self.choose.clone()
    }
    pub fn get_next(&self) -> Option<Box<QueueNode>> {
        self.next.clone()
    }
    pub fn get_piece(&self) -> Option<PieceColor> {
        self.piece
    }
    pub fn push_back(&mut self, node: QueueNode) {
        if let Some(next) = &self.next {
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
        if self.next.is_some() {

            let mut new = Box::new(node);
            new.next = self.next.clone();
            self.next = Some(new);
            

        } else {
            self.next = Some(Box::new(node));
        }
    }
    pub fn insert_at(&mut self, n: usize, node: QueueNode) {
        if let Some(mut next) = self.next.clone() {
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
        if let Some(mut next ) = self.next.clone() {
            let mut cur = Box::new(self.clone());
            while next.next.is_some() { 
                cur = next;
                next = cur.next.unwrap();
            }
            let mut ret = next;
            cur.next = None;
            Some(*ret)
        } else {
            None
        }
        
    }
    pub fn pop_next(&mut self) -> Option<QueueNode> {
        if let Some(mut next) = self.next.clone() {
            let mut ret = *next.clone();
            self.next = None;
            Some(ret)
        } else {
            None
        }
    }
    pub fn at(&self, index: usize) -> Option<&QueueNode> {
        if index == 0 {
            return Some(self);
        }
        if self.next.is_some() {
            let mut cur = self;
            for _ in 0..index {
                if let Some(next) = &cur.next {
                    cur = next.as_ref();
                }
            }
            Some(cur)

        } else {
            None
        }
    }
    pub fn mut_at(&mut self, index: usize) -> Option<&mut QueueNode> {
        if index == 0 {
            return Some(self)
        } 
        if self.next.is_some() {
            let mut cur = self;
            for _ in 0..index {
                if cur.next.is_some() {
                    cur = cur.next.as_mut().unwrap().as_mut();
                }
            }
            Some(cur)
        } else {
            None
        }
        
    }
    pub fn last(&self) -> &QueueNode{
        if self.next.is_some() {
            let mut cur = self;
            while cur.next.is_some() {
                cur = cur.next.as_ref().unwrap();
            }
            cur
        } else {
            self
        }
    }
    pub fn last_mut(&mut self) -> &mut QueueNode {
        if self.next.is_some() { 
            let mut cur = self;
            while cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
            }
            cur
        } else {
            self
        }
    }
    pub fn isolated_clone(&self) -> QueueNode {
        QueueNode {
            node_type: self.node_type.clone(),
            choose: self.choose.clone(),
            piece: self.piece.clone(),
            next: None
        }
    }
    pub fn len(&self) -> usize {
        if self.next.is_some() {
            let mut cur = Box::new(self.clone());
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
    pub fn iter(&self) -> QueueNodeIterator {
        QueueNodeIterator { cur: Some(Box::new(self.clone())) }
    }

    pub fn next(&self) -> Box<QueueNode> {
        self.next.clone().unwrap()
    }
}
impl fmt::Display for QueueNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.node_type {
            QueueNodeType::Choose => {
                if let Some(choose) = &self.choose {
                    write!(f, "{}", choose);
                }
            } 
            QueueNodeType::Piece => {
                if let Some(piece) = &self.piece {
                    write!(f, "{}", piece_color_to_char(piece.clone()));
                }
            }
        }
        fmt::Result::Ok(())
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
pub struct QueueNodeIterator {
    cur: Option<Box<QueueNode>>
}
impl ExactSizeIterator for QueueNodeIterator {
    fn len(&self) -> usize {
        if self.cur.is_some() {
            self.cur.as_ref().unwrap().len()
        } else {
            0
        }
    }
}
impl Iterator for QueueNodeIterator {
    type Item = QueueNode;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = &self.cur {
            let ret = cur.as_ref().clone();
            self.cur = cur.next.clone();
            Some(ret)
        } else {
            None
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
    #[wasm_bindgen(constructor)]
    pub fn new() -> Queue {
        Queue {head: None}
    }
    pub fn append(&mut self, queue: Queue) {
        if let Some(mut head) = self.head.clone() {
            if let Some(mut other) = queue.head {
                head.push_back(other);
            }
        } else {
            self.head = queue.head;
            
        }
    }
    #[wasm_bindgen(js_name = pushBack)]
    pub fn js_push_back(&mut self, node: JsValue) {
        if let Ok(node) = serde_wasm_bindgen::from_value(node){
            self.push_back(node);
        }
    }
    #[wasm_bindgen(js_name = popBack)]
    pub fn js_pop_back(&mut self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let node = self.pop_back();
        serde_wasm_bindgen::to_value(&node)
    }
    #[wasm_bindgen(js_name = pushFront)]
    pub fn js_push_front(&mut self, node: JsValue) {
        if let Ok(node) = serde_wasm_bindgen::from_value(node){
            self.push_front(node);
        }
    }
    #[wasm_bindgen(js_name = popFront)]
    pub fn js_pop_front(&mut self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let node = self.pop_front();
        serde_wasm_bindgen::to_value(&node)
    }
    pub fn head(&self) -> QueueNode {
        if self.head.is_some() {
            self.head.clone().unwrap()
        } else {
            panic!("Head node doesn't exist")
        }
    }
    #[wasm_bindgen(js_name = fromString)]
    pub fn js_from_string(s: &str) -> Option<Queue> {
        let res = Self::from_string(String::from(s));
        if res.is_err() {
            None
        } else {
            Some(res.unwrap())
        }
    }
    #[wasm_bindgen(js_name = insertPiece)] 
    pub fn insert_piece(&mut self, piece: PieceColor) {
        let node = QueueNode::from(piece);
        self.push_back(node);
    }
    #[wasm_bindgen(js_name = takeNextPiece)]
    pub fn take_next_piece(&mut self) -> Option<PieceColor> {
        let node = self.pop_front();
        if let Some(node) = node {
            if let Some(piece) = node.get_piece() {
                Some(piece)
            } else {
                None
            }
        } else {
            None
        }
    }

}
impl Queue {
    pub fn push_back(&mut self, node: QueueNode) {
        if let Some(ref mut head) = self.head {
            head.push_back(node);
        } else {
            self.head = Some(node);
        }
    }
    pub fn pop_back(&mut self) -> Option<QueueNode> { 
        if let Some(ref mut head) = self.head {
            head.pop()
        }
        else {
            None
        }
    }
    pub fn push_front(&mut self, mut node: QueueNode) {
        if let Some(head) = self.head.clone() {
            node.next = Some(Box::new(head));
            self.head = Some(node);
        } else {
            self.head = Some(node);
        }
    }
    pub fn pop_front(&mut self) -> Option<QueueNode> {
        if let Some(mut head) = self.head.clone() {
            if let Some(mut next) = head.next.clone()  {
                head.next = None;
                self.head = Some(*next);
                Some(head)    
            } else {
                self.head = None;
                Some(head)
            }
            
        } else {
            None
        }
    }

    pub fn last(&self) -> QueueNode {
        if let Some(ref head) = self.head {
            head.last().clone()
        } else {
            panic!("Head node doesn't exist")
        }
    }
    pub fn last_mut(&mut self) -> &mut QueueNode {
        if let Some(ref mut head) = self.head {
            head.last_mut()
        } else {
            panic!("Head node doesn't exist")
        }
    }
    pub fn len(&self) -> usize {
        if let Some(head) = &self.head {
            head.len()
        } else {
            0
        }
    }
    pub fn from_string(mut s: String) -> Result<Self, InvalidQueueFormatError> {
        let mut base = Queue::new();
        s.retain(|c| !c.is_whitespace());
  
        let mut idx = 0;
        while idx < s.len() {
            let mut c = s.chars().nth(idx).unwrap();
            if c == '[' || c == '*' {
                let mut pieces = Vec::new();
                let mut inverse = false;
                let mut count = 0;
                if c == '[' {
                    idx += 1;
                    c = s.chars().nth(idx).unwrap();
                    if c == '^' {
                        inverse = true;
                        
                    }
                    while s.chars().nth(idx).unwrap() != ']' {
                        c = s.chars().nth(idx).unwrap();
                        count += 1;
                        if is_piece_color(c) {
                            if pieces.contains(&PieceColor::from(c)) {
                                return Err(InvalidQueueFormatError {  })
                            } else {
                                pieces.push(PieceColor::from(c));
                                idx += 1;
                            }
                        } else {
                            return Err(InvalidQueueFormatError {  })
                        }
                    }

                } else if c == '*' {
                    count = 7;
                    pieces = Vec::from(crate::piece::get_pieces());
                } else {
                    return Err(InvalidQueueFormatError {  })
                }
                idx += 1;
                c = s.chars().nth(idx).unwrap();
                if c == 'p' {
                    idx += 1;
                    count = s.chars().nth(idx).unwrap().to_digit(10).unwrap() as usize;
                    if count > pieces.len() {
                        return Err(InvalidQueueFormatError {  })
                    }
                } else if c == '!' {
                    count = count;
                } else {
                    return Err(InvalidQueueFormatError {  })
                }
                let choose = Choose {
                    pieces,
                    count,
                    inverse
                };
                base.push_back(QueueNode {
                    node_type: QueueNodeType::Choose,
                    choose: Some(choose),
                    piece: None,
                    next: None
                });
                idx += 1;
            } else if c == ',' {
                idx += 1;
            } else if is_piece_color(c) {
                base.push_back(QueueNode {
                    node_type: QueueNodeType::Piece,
                    piece: Some(PieceColor::from(c)),
                    choose: None,
                    next: None
                });
                idx += 1;
            }
            
        }
        Ok(base)
    }
    pub fn iter(&self) -> QueueNodeIterator {
        if let Some(head) = &self.head {
            head.iter()
        } else {
            panic!()
        }
    }
    pub fn possible_q_iter(&self) -> QueueChooseIterator<'_> {
        QueueChooseIterator::new(self)
    }
}
impl fmt::Display for Queue { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        if let Some(ref head) = &self.head {
            // f.write_char('a');
            for i in 0..self.len() {
                
                if let Some(piece) = head.at(i).clone() {
                    match piece.node_type {
                        QueueNodeType::Piece => {
                            f.write_char(piece_color_to_char(piece.piece.unwrap()));
                        }
                        QueueNodeType::Choose => {
                            f.write_str(piece.choose.clone().unwrap().to_string().as_str());
                        }
                    }
                    if i != self.len() - 1 {
                        if let Some(next) = head.at(i + 1).clone() {
                            if next.node_type == QueueNodeType::Choose {
                                f.write_char(',');

                            }
                        }
                        
        
                    } 
                }
                
            }
        }

        
        fmt::Result::Ok(())
    }
}
pub struct QueueChooseIterator<'a> {
    queue: &'a Queue,
    chooses: Vec<Choose>,
    choose_map: HashMap<usize, Choose>,
    states: HashMap<usize, Vec<usize>>,
    idxs: Vec<usize>,
    c: u64
}
impl QueueChooseIterator<'_> {
    pub fn new(queue: &Queue) -> QueueChooseIterator {
        let mut iter = QueueChooseIterator {
            queue,
            chooses: Vec::new(),
            choose_map: HashMap::new(),
            states: HashMap::new(),
            idxs: Vec::new(),
            c: 0
        };
        let mut i = 0;
        for node in queue.head().iter() {
            match node.node_type {
                QueueNodeType::Choose => { 
                    iter.chooses.push(node.choose().clone());
                    iter.choose_map.insert(i, node.choose().clone());
                    iter.states.insert(i, vec![0; node.choose().count]);
                    iter.idxs.push(i);
                    
                }
                _ => {}
            }
            i += 1;
        }

        iter

    }
    pub fn size(&self) -> u64 {
        let mut base = 1;
        for choose in self.chooses.iter() {
            base *= choose.size() as u64;
        }
        base
    }
}

impl Iterator for QueueChooseIterator<'_> { 
    type Item = Queue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.len() == 0 || self.c >= self.size() {
            return None;
        }
        if self.chooses.len() == 0 {
            self.c += 1;
            return Some(self.queue.clone());
        }

        if self.c != 0 {
            let mut state_idx_idx = self.idxs.len() - 1;
            let mut state_idx = &self.idxs[state_idx_idx];
            let mut state = self.states.get_mut(state_idx).unwrap();
            let mut choose = self.choose_map.get_mut(state_idx).unwrap();
            let mut idx = state.len() - 1;
            state[idx] += 1;
            while state[idx] >= choose.pieces.len() - idx {
                state[idx] = 0;
                if idx != 0 {
                    idx -= 1;
                    state[idx] += 1
                } else {
                    state_idx_idx -= 1;
                    state_idx = &self.idxs[state_idx_idx];
                    state = self.states.get_mut(state_idx).unwrap();
                    choose = self.choose_map.get_mut(state_idx).unwrap();
                    idx = state.len() - 1;
                    state[idx] += 1;
                }

            }
        }
        let mut base = Queue::new();
            
        for i in 0..self.queue.len() {
            if let Some(node) = self.queue.head().at(i).clone() {
                match node.node_type {
                    QueueNodeType::Piece => {
                        base.push_back(node.isolated_clone());
                    } 
                    QueueNodeType::Choose => {
                        let mut q = ChooseIterator::idxs_to_queue(
                            self.states.get(&i).unwrap().clone(),
                            node.isolated_clone().choose().clone()
                        );
                        base.append(q);
                    }
                }
            }
        }
        self.c += 1;
        Some(base)
    }
}
#[derive(Clone)]
struct ChooseState {
    queue: Queue,
    choose: Choose
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Choose {
    pub pieces: Vec<PieceColor>,
    pub count: usize,
    pub inverse: bool
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
        let n = self.pieces.len();
        let r = self.count;
            usize_factorial(n) 
                            / 
            usize_factorial(n - r)
        
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
            state_clone.queue.push_back(new);
            state_clone.choose.count -= 1;
            state_clone.choose.pieces.remove(i);

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

            state_clone.queue.push_back(new);
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
    /**
     * Reorders the choose's pieces into TILJOSZ order. 
     */
    pub fn sort(&mut self) {
        self.pieces.sort()
    }
    pub fn iter(&self) -> ChooseIterator<'_> {
        ChooseIterator::new(self)
    }
}

pub struct ChooseIterator<'a> {
    choose: &'a Choose,
    state: Vec<usize>,
    c: usize,
    first: bool
}
impl ExactSizeIterator for ChooseIterator<'_> {
    fn len(&self) -> usize {
        self.choose.size()

    }
}
impl ChooseIterator<'_> {
    pub fn new (choose: &Choose) -> ChooseIterator {
        
        ChooseIterator {
            choose,
            state: vec![0; choose.count],
            c: 0,
            first: true
        }
    }
    fn idxs_to_queue(idxs: Vec<usize>, choose: Choose) -> Queue {
        let mut choose = choose;
        choose.sort();
        let mut queue = Queue::new();
        for idx in idxs {
            queue.push_back(QueueNode {
                node_type: QueueNodeType::Piece,
                piece: Some(choose.pieces[idx]),
                choose: None,
                next: None
            });
            choose.pieces.remove(idx);
        }
        queue
    }
}
impl<'a> Iterator for ChooseIterator<'a> {
    type Item = Queue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.choose.count == 0 {
            None
        } else if self.c >= self.len() {
            
            None
        } else if self.first {
            self.c += 1;
            self.first = false;
            return Some(ChooseIterator::idxs_to_queue(self.state.clone(), self.choose.clone()));
        } else {
            self.c += 1;
            let mut stop_incr = false;
            let mut idx = self.state.len() - 1;
            while !stop_incr {
                
                self.state[idx] += 1;
                if self.state[idx] < self.choose.pieces.len() - idx  {
                    stop_incr = true;
                } else {
                    self.state[idx] = 0;
                    idx -= 1;
                }
            }
            return Some(ChooseIterator::idxs_to_queue(self.state.clone(), self.choose.clone()));
        }
        
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
impl Queue {
    fn from(s: String) -> Result<Self, InvalidQueueFormatError> {
        let mut chars = s.chars();
        let mut queue = Queue::new();
        let mut idx = 0;
        while idx < s.len() {
            let mut c = chars.nth(idx).unwrap();
            if c == '[' || c == '*' {
                let mut pieces = Vec::new();
                let mut inverse = false;
                let mut count = 0;
                if c == '[' {
                    idx += 1;
                    if let Some(inv) = chars.next() {
                        
                        if inv == '^' {
                            inverse = true;
                            idx += 1;
                        }
                        
                        while chars.nth(idx).unwrap() != ']' {
                            c = chars.nth(idx).unwrap();
                            if is_piece_color(c) {
                                if pieces.contains(&PieceColor::from(c)) {
                                    return Err(InvalidQueueFormatError {  })
                                }
                                pieces.push(PieceColor::from(c));
                            } else {
                                return Err(InvalidQueueFormatError {  })
                            }
                            idx += 1;
                        }
                    }
                }
                else if c == '*' {
                    pieces = Vec::from(crate::piece::get_pieces());
                }
                idx += 1;
                
                c = chars.nth(idx).unwrap();
                if c == '!' {
                    count = pieces.len();
                } else if c == 'p' {
                    idx += 1;
                    c = chars.nth(idx).unwrap();
                    count = c.to_digit(10).unwrap() as usize;
                } else {
                    count = pieces.len();
                }
                let choose = Choose {
                    pieces,
                    count,
                    inverse
                };
                queue.push_back(QueueNode {
                    node_type: QueueNodeType::Choose,
                    choose: Some(choose),
                    piece: None,
                    next: None
                });
                idx += 1;

            } else if is_piece_color(c) {
                queue.push_back(QueueNode {
                    node_type: QueueNodeType::Piece,
                    piece: Some(PieceColor::from(c)),
                    choose: None,
                    next: None
                });
                idx += 1;
            } else if c == ',' {
                idx += 1;
            } else {
                return Err(InvalidQueueFormatError {  })
                
            }
            

                
            
        }
        Ok(queue)
    }
}
    