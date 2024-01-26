#![allow(unused_must_use)]
use crate::piece::PieceColor;
use core::fmt;
use std::fmt::Write;
#[derive(Clone)]
pub struct Queue {
    pieces: Vec<PieceColor>
}
impl Queue {
    pub fn push(&mut self, color: PieceColor) {
        self.pieces.push(color);

    }
    pub fn pop(&mut self) -> Option<PieceColor> { 
        self.pieces.pop()
    }
}
impl fmt::Display for Queue { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        for i in 0..self.pieces.len() {
            let piece = self.pieces[i];
            f.write_char(piece.to_char());
            if i != self.pieces.len() - 1 {
                f.write_char(',');
                f.write_char(' ');

            } 
        }
        
        return fmt::Result::Ok(());
    }
}
#[derive(Clone)]
struct ChooseState {
    queue: Queue,
    choose: Choose
}
#[derive(Clone)]
pub struct Choose {
    pieces: Vec<PieceColor>,
    count: usize,
}
impl Choose {
    pub fn get_queues(&self) -> Vec<Queue> {
        println!("Base"); 
        let mut ret: Vec<Queue> = Vec::new();
        let state = ChooseState {
            queue: Queue {pieces: Vec::new()},
            choose: self.clone()
        };
        for i in 0..self.pieces.len() {
            
            let piece = state.choose.pieces[i];
            let mut state_clone = state.clone();
            state_clone.queue.push(piece);
            state_clone.choose.count -= 1;
            state_clone.choose.pieces.remove(i);
            println!("{:?}", state_clone.choose.pieces);
            ret.append(&mut self.get_queues0(state_clone));
            
            
        }

        return ret;
    }
    fn get_queues0(&self, state: ChooseState) -> Vec<Queue>{
        let mut ret: Vec<Queue> = Vec::new();

        for i in 0..state.choose.pieces.len() {
            let piece = state.choose.pieces[i];
            let mut state_clone = state.clone();
            state_clone.queue.push(piece);
            state_clone.choose.count -= 1;
            state_clone.choose.pieces.remove(i);
            if state_clone.choose.count == 0 {
                ret.push(state_clone.queue);
            } else {
                ret.append(&mut self.get_queues0(state_clone));
            }
            
        }
        return ret;
    }
}
pub fn choose(queue: Vec<PieceColor>, count: usize) -> Vec<Queue> {
    (Choose {
        pieces: queue,
        count: count,
    }).get_queues()
}
