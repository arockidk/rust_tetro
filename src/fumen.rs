use core::fmt;
use std::cell::Cell;
use std::collections::LinkedList;
use fumen::{CellColor, Piece, PieceType, RotationState};
use wasm_bindgen::{convert::FromWasmAbi, prelude::wasm_bindgen};

use crate::{board::{Board, TetBoard}, field::Field, piece::{Direction, PieceColor, TetPiece}, vec2::Vec2};
#[derive(Clone, Debug)]

#[wasm_bindgen]
pub struct TetPage {
    fumen_page: fumen::Page,
    field: Field,
    pub rise: bool,
    pub lock: bool,
    pub mirror: bool,
    comment: Option<String>
} 
#[derive(Clone)]
#[wasm_bindgen()]
pub struct TetFumen {
    pages: Vec<TetPage>,
    fumen: fumen::Fumen,
    pub guideline: bool
   
}
// impl FromWasmAbi for Fumen {
//     type Abi = ;

//     unsafe fn from_abi(js: Self::Abi) -> Self {
//         todo!()
//     }
// }
fn cell_color_to_u8(c: CellColor) -> u8 {
    match c {
        CellColor::Empty => 0,
        CellColor::I => 1,
        CellColor::L => 2,
        CellColor::O => 3,
        CellColor::Z => 4,
        CellColor::T => 5,
        CellColor::J => 6,
        CellColor::S => 7,
        CellColor::Grey => 8
    }
}
pub fn fumen_piece_type_to_piece_color(pt: PieceType) -> PieceColor {
    match pt {
        PieceType::I => PieceColor::I,
        PieceType::L => PieceColor::L,
        PieceType::O => PieceColor::O,
        PieceType::Z => PieceColor::Z,
        PieceType::T => PieceColor::T,
        PieceType::J => PieceColor::J,
        PieceType::S => PieceColor::S
    }
}
pub fn rotation_state_to_direction(rs: RotationState) -> Direction {
    match rs {
        RotationState::East => Direction::East,
        RotationState::South => Direction::South,
        RotationState::West => Direction::West,
        RotationState::North => Direction::North
    }
}
fn u8_to_cell_color(i: u8) -> CellColor { 
    match i {
        0 => CellColor::Empty,
        1 => CellColor::I,
        2 => CellColor::L,
        3 => CellColor::O,
        4 => CellColor::Z,
        5 => CellColor::T,
        6 => CellColor::J,
        7 => CellColor::S,
        8 => CellColor::Grey,
        _ => CellColor::Empty
    }
}
fn piece_color_to_fumen_piece_type(c: PieceColor) -> PieceType {
    match c {
        PieceColor::I => PieceType::I,
        PieceColor::L => PieceType::L,
        PieceColor::O => PieceType::O,
        PieceColor::Z => PieceType::Z,
        PieceColor::T => PieceType::T,
        PieceColor::J => PieceType::J,
        PieceColor::S => PieceType::S,
        PieceColor::B => PieceType::T,
        PieceColor::G => panic!("PieceColor::G cannot be converted to PieceType"),
    }
}
fn piece_color_from_fumen_piece_type(c: PieceType) -> PieceColor {
    match c {
        PieceType::I => PieceColor::I,
        PieceType::L => PieceColor::L,
        PieceType::O => PieceColor::O,
        PieceType::Z => PieceColor::Z,
        PieceType::T => PieceColor::T,
        PieceType::J => PieceColor::J,
        PieceType::S => PieceColor::S,
    }
}
fn direction_to_rotation_state(dir: Direction) -> RotationState {
    match dir {
        Direction::East => RotationState::East,
        Direction::South => RotationState::South,
        Direction::West => RotationState::West,
        Direction::North => RotationState::North
    }
}
impl std::default::Default for TetPage {
    fn default() -> Self {
        TetPage {
            field: Field::new(TetBoard::new(), None, None),
            rise: false,
            lock: true,
            mirror: false,
            fumen_page: fumen::Page::default(),
            comment: None
        }
    }
}
#[wasm_bindgen]
impl TetPage  {
    // pub fn to_fumen_page(&self) -> fumen::Page {
    //     fumen::Page {
    //         field: self.field.board.get_tile_matrix().map(
    //             |v| v.map(|c| u8_to_cell_color(c)) 
    //         )[0..23].try_into().unwrap(),
    //         piece: Some(Piece { 
    //             kind: piece_color_to_fumen_piece_type(self.field.active_piece.color),
    //             rotation: direction_to_rotation_state(self.field.active_piece.rotation),
    //             x: self.field.active_piece.position.0 as u32,
    //             y: (23 - self.field.active_piece.position.1) as u32,
                
    //         }),
    //         garbage_row: self.field.board.get_tile_matrix().map(
    //             |v| v.map(|c| u8_to_cell_color(c)) 
    //         )[23],
    //         rise: self.rise,
    //         mirror: self.mirror,
    //         lock: self.lock,
    //         comment: self.comment,
    //     }
    // }
    #[wasm_bindgen(js_name = "createBlankPiece")]
    pub fn create_blank_piece(&mut self) {
        self.fumen_page.piece = Some(Piece { kind: 
            PieceType::T,
            rotation: RotationState::North,
            x: 4, 
            y: 10 
        });
    }
    #[wasm_bindgen(setter)]
    pub fn set_piece_color(&mut self, c: PieceColor) {
        match self.fumen_page.piece {
            Some(ref mut p) => 
            {
                p.kind = piece_color_to_fumen_piece_type(c); 
                match self.field.active_piece {
                    Some(ref mut piece) => {
                        piece.set_color(c);
                    }
                    None => ()
                }
                
            },
            None => ()
        }   
    } 
    #[wasm_bindgen(setter)]
    pub fn set_piece_rotation(&mut self, dir: Direction) {
        match self.fumen_page.piece {
            Some(ref mut p) => {
                p.rotation = direction_to_rotation_state(dir);
                match self.field.active_piece {
                    Some(ref mut piece) => {
                        piece.rotation = dir;
                    }
                    None => ()
                }
            }
            None => ()
        }
    }
    #[wasm_bindgen(setter)]
    pub fn set_piece_position(&mut self, pos: Vec2) {
        match self.fumen_page.piece {
            Some(ref mut p) => {
                p.x = pos.0 as u32;
                p.y = (23 - pos.1) as u32;
                match self.field.active_piece {
                    Some(ref mut piece) => {
                        piece.position = pos;
                    }
                    None => ()
                }
            }
            None => ()
            
        }
    }
    #[wasm_bindgen(setter)]
    pub fn set_field(&mut self, field: Field) {

        let mut inversed_field: Field = field;
        
        for y in 0..24 {
            for x in 0..10 {
                // println!("{}", x);
                inversed_field.board.set_tile(x, y, field.board.get_tile(x, 23 - y));
                
            }
        }
        // println!("{}", field);
        // println!("{}", inversed_field);
        self.field = field;
        self.fumen_page.set_field_rs(inversed_field.board.get_tile_matrix().map(
            |v| v.map(|c| u8_to_cell_color(c))
        )[0..23].try_into().unwrap());
        // println!("{}", self.fumen_page.field.map(
        //     |row| format!("{:?}\n", row.map(|c| cell_color_to_u8(c)))
        // ).join("\n")); 
        // self.fumen_page.garbage_row = inversed_field.board.get_tile_matrix().map(
        //     |v| v.map(|c| u8_to_cell_color(c))
        // )[];
        if field.active_piece.is_some() {
            self.create_blank_piece();
            let active = field.active_piece.unwrap();
            self.set_piece_color(active.color());
            self.set_piece_position(active.position);
            self.set_piece_rotation(active.rotation);
        }
    }
    
    #[wasm_bindgen(setter)]
    pub fn set_comment(&mut self, comment: Option<String>) {
        self.comment = comment.clone();
        self.fumen_page.set_comment_rs(comment.clone());
        
    }
    #[wasm_bindgen(getter)]
    pub fn field(&self) -> Field {
        self.field.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn fumen_page(&self) -> fumen::Page {
        self.fumen_page.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn comment(self) -> Option<String> {
        self.comment.clone()
    }
    #[wasm_bindgen(js_name = "fromFumenPage")]
    pub fn from_fumen_page(pg: fumen::Page) -> TetPage {
        let mut new_pg = TetPage { 
            field: Field::new(TetBoard::new(), None, None),
            rise: pg.rise,
            lock: pg.lock,
            mirror: pg.mirror,
            comment: pg.get_comment().clone(),
            fumen_page: pg.clone(),
        };
        if pg.piece.is_some() {
            let pg_p = pg.piece.unwrap();
            let mut p = TetPiece::new(
                piece_color_from_fumen_piece_type(pg_p.kind),
                rotation_state_to_direction(pg_p.rotation),
                Vec2(pg_p.x.try_into().unwrap(), pg_p.y.try_into().unwrap())
            );
            new_pg.field.active_piece = Some(p);
        }
        for y in 0..23 {
            for x in 0..10 {
                if y == -2 {
                    new_pg.field.board.set_tile(x, y, pg.get_garbage_row()[x as usize] as u8)
                } else {
                    new_pg.field.board.set_tile(x, y, pg.get_field()[y as usize][x as usize] as u8)
                }
            }
        }
        new_pg 
    }
}
impl TetPage {
    pub fn get_field(&self) -> &Field {
        &self.field   
    }
    pub fn get_comment(&self) -> &Option<String> {
        &self.comment
    }
    pub fn get_fumen_page(&self) -> &fumen::Page {
        &self.fumen_page
    }
    pub fn get_field_mut(&mut self) -> &mut Field {
        &mut self.field   
    }
    pub fn get_comment_mut(&mut self) -> &mut Option<String> {
        &mut self.comment
    }
    pub fn get_fumen_page_mut(&mut self) -> &mut fumen::Page {
        &mut self.fumen_page
    }
}
impl fmt::Display for TetPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.field.to_string().as_str())
    }
}
#[wasm_bindgen]
impl TetFumen { 
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        TetFumen {
            pages: Vec::new(),
            fumen: fumen::Fumen::new(),
            guideline: true
        }
    }
    pub fn load(code: String) -> Self {
        let mut new_fum = Self::new();
        new_fum.decode_fumen(code);
        return new_fum;
    }
    #[wasm_bindgen(js_name = "loadSlice")]
    pub fn load_slice(code: &str) -> Self {
        Self::load(String::from(code))
    }
    pub fn len(&self) -> usize {
        self.pages.len()
    }
    #[wasm_bindgen(js_name = "addPage")]
    pub fn add_page(&mut self) -> *mut TetPage {
        let mut page = TetPage::default();
        self.pages.push(page.clone());
        self.pages.last_mut().unwrap()
        
    }
    pub fn update(&mut self) {
        self.fumen.get_pages_mut().clear();
        for page in self.pages.iter() {
            let pg = page.clone();
            self.fumen.get_pages_mut().push(pg.fumen_page);
        }
    }
    #[wasm_bindgen(js_name = "encodeFumen")]
    pub fn encode_fumen(&self) -> String { 
        let mut clne = self.clone();
        clne.update();
        return clne.encode();
    }
    fn encode(&self) -> String {
        self.fumen.encode()
    }

    #[wasm_bindgen(js_name = "decodeFumen")]
    pub fn decode_fumen(&mut self, fumen: String) {
        println!("{}", fumen);
        let mut new_pagee = fumen::Fumen::decode(fumen.as_str());
        if new_pagee.is_err() {
            let err = new_pagee.clone().err().unwrap();
            eprintln!("{:?}", err);
            return
        }
        let new_page = new_pagee.unwrap();
        
        // println!("AAAe {:?}", new_page.get_pages()[0].piece.unwrap());
        self.fumen = new_page.clone();
        self.pages = Vec::new();
        for page in new_page.get_pages() { 
            self.pages.push(TetPage::from_fumen_page(page.clone()));
        }
        
    }
    #[wasm_bindgen(js_name = "getPageAt")]
    pub fn js_get_page_at(&self, idx: usize) -> TetPage {
        self.pages[idx].clone()
    }

}
impl TetFumen {
    pub fn add_page_rs(&mut self) -> &mut TetPage {
        let mut page = TetPage::default();
        self.pages.push(page.clone());
        self.pages.last_mut().unwrap()
        
    }
    pub fn get_page_at(&self, idx: usize) -> &TetPage {
        &self.pages[idx]
    }
    pub fn get_page_at_mut(&mut self, idx: usize) -> &mut TetPage {
        &mut self.pages[idx]
    }
}
impl TetBoard {
    pub fn quick_fumen_encode(&self) -> String {
        let mut fum = TetFumen::new();
        let page = fum.add_page_rs();
        page.set_field(Field::new(self.clone(), None, None));
        fum.encode_fumen()
    }
}
impl Field {
    pub fn encode_fum(&self) -> String {
        let mut fum = TetFumen::new();
        let pg = fum.add_page_rs();
        pg.set_field(self.clone());
        fum.encode_fumen()
    }
}