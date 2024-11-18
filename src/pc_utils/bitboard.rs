use std::{collections::{HashMap, HashSet, LinkedList}, fmt::Write, ops::{BitAnd, BitAndAssign}};

use crate::{board::{Board, ClearedLine, TetBoard}, fumen::TetFumen, pc_utils::{bitboard, bitpiece::BOUND_MAPS}, piece::PieceColor, queue::Queue};

use super::{bitpiece::{BitPiece, K_CW}, bitqueue::BitQueue};

const ROW: u64 = 0b1111111111;
const LEFT_WALL: u64  = 0b1000000000_1000000000_1000000000_1000000000;
const RIGHT_WALL: u64 = 0b0000000001_0000000001_0000000001_0000000001;
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BitBoard(pub u64);
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Placements(pub [u64; 4], pub [u64; 4]);
pub fn sgn_shl(a: u64, b: i8) -> u64 {
    if b < 0 {
        a >> -b
    } else {
        a << b
    }
}
pub fn sgn_shr(a: u64, b: i8) -> u64 {
    if b < 0 {
        a << -b
    } else {
        a >> b
    }
}
pub fn sgn_shr_eq(a: &mut u64, b: i8) {
    if b < 0 {
        *a <<= -b
    } else {
        *a >>= b
    }
}
pub fn sgn_shl_eq(a: &mut u64, b: i8) {
    if b < 0 {
        *a >>= -b
    } else {
        *a <<= b
    }
}
pub fn sgn_shr_i32(a: u64, b: i32) -> u64 {
    if b < 0 {
        a << -b
    } else {
        a >> b
    }
}
pub fn sgn_shl_i32(a: u64, b: i32) -> u64 {
    if b < 0 {
        a >> -b
    } else {
        a << b
    }
}
macro_rules! placements_cw {
    ($self:ident, $src:expr, $out:ident) => {
        let mut s = $self.0[$src];
        let mut temp = s;
        s = $self.0[($src+1)%4];
        let mut i = 0;
        while i < 5 && temp != 0 {
            let diff = K_CW[$src][i];
            sgn_shl_eq(&mut temp, diff);
            let to_change = temp & $self.1[($src+1)%4];
            s |= to_change;
            temp ^= to_change;
            sgn_shr_eq(&mut temp, diff);
            i += 1;
        }   
        $out = s;
    };
}
pub fn create_boards(placements: Placements, base: BitBoard, color: usize) -> Vec<BitBoard> {
    let mut boards = Vec::new();
    for 
            let pos = bs.trailing_zeros() as usize;
            bp.0 = pos;
            bp.1 = rot;
            let clr_result = base.cleared();
            let mut cleared = BitBoard(clr_result.0);
            cleared.0 |= bp.bit_repr();
            cleared.unclear(clr_result.1);
            // println!("{} {} {}", cleared, bp.0, bp.1);

            boards.push(cleared);
            bs &= !(1 << pos);
        }
    }
    boards
}
impl Placements {
    pub fn new(collsion_maps: [u64; 4]) -> Self {
        let base = (ROW << 50) | (ROW << 40);
        Self([base, base, base, base], collsion_maps)
    }
    pub fn move_left(&mut self) {
        let s0 = (self.0[0] << 1) & !RIGHT_WALL;
        let s1 = (self.0[1] << 1) & !RIGHT_WALL;
        let s2 = (self.0[2] << 1) & !RIGHT_WALL;
        let s3 = (self.0[3] << 1) & !RIGHT_WALL;
        self.0[0] |= s0;
        self.0[1] |= s1;
        self.0[2] |= s2;
        self.0[3] |= s3;
    }
    pub fn move_right(&mut self) {
        let s0 = (self.0[0] >> 1) & !LEFT_WALL;
        let s1 = (self.0[1] >> 1) & !LEFT_WALL;
        let s2 = (self.0[2] >> 1) & !LEFT_WALL;
        let s3 = (self.0[3] >> 1) & !LEFT_WALL;
        self.0[0] |= s0;
        self.0[1] |= s1;
        self.0[2] |= s2;
        self.0[3] |= s3;     
    }
    pub fn move_down(&mut self) {
        let s0 = (self.0[0] >> 10);
        let s1 = (self.0[1] >> 10);
        let s2 = (self.0[2] >> 10);
        let s3 = (self.0[3] >> 10);
        self.0[0] |= s0;
        self.0[1] |= s1;
        self.0[2] |= s2;
        self.0[3] |= s3;
    }
    pub fn cw(&mut self) {
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;
        let mut s0 = 0;
        placements_cw!(self, 0, s1);
        placements_cw!(self, 1, s2);
        placements_cw!(self, 2, s3);
        placements_cw!(self, 3, s0);
        self.0[0] = s0;
        self.0[1] = s1;
        self.0[2] = s2;
        self.0[3] = s3;
    }
    pub fn filter_placeable(&mut self) {
        self.0[0] = (((self.0[0] >> 10) & !self.1[0]) << 10) | self.0[0] & ROW;
        self.0[1] = (((self.0[1] >> 10) & !self.1[1]) << 10) | self.0[1] & ROW;
        self.0[2] = (((self.0[2] >> 10) & !self.1[2]) << 10) | self.0[2] & ROW;
        self.0[3] = (((self.0[3] >> 10) & !self.1[3]) << 10) | self.0[3] & ROW;
    }
    pub fn filter_bounds(&mut self, maps: [u64; 4]) {
        *self &= maps;
    }
    pub fn filter_congruents(&mut self, color: usize) {
        match color {
            1 => {
                // NS
                self.0[0] |= self.0[2] >> 9;
                
                // EW
                self.0[1] |= self.0[3] >> 11;

                self.0[2] = 0;
                self.0[3] = 0;
            }
            4 | 7 => {
                // NS
                self.0[0] |= self.0[2] >> 10;
                // EW
                self.0[1] |= self.0[3] >> 1;

                self.0[2] = 0;
                self.0[3] = 0;

            }
            _ => {}
        }
    }
    pub fn step(&mut self) {
        self.move_down();
        *self &= self.1;
        self.move_left();
        *self &= self.1;
        self.move_right();
        *self &= self.1;
        self.cw();
        *self &= self.1;
    }

    pub fn gen_all(&mut self) {
        let mut old0 = self.0[0];
        let mut old1 = self.0[1];
        let mut old2 = self.0[2];
        let mut old3 = self.0[3];
        self.step();
        let mut diff0 = self.0[0] ^ old0;
        let mut diff1 = self.0[1] ^ old1;
        let mut diff2 = self.0[2] ^ old2;
        let mut diff3 = self.0[3] ^ old3;
        while (diff0 | diff1 | diff2 | diff3) != 0 {
            old0 = diff0 ^ old0;
            old1 = diff1 ^ old1;
            old2 = diff2 ^ old2;
            old3 = diff3 ^ old3;
            self.step();
            diff0 = self.0[0] ^ old0;
            diff1 = self.0[1] ^ old1;
            diff2 = self.0[2] ^ old2;
            diff3 = self.0[3] ^ old3;
        }
        self.filter_placeable();
    }
    pub fn for_each(&self, func: &impl Fn(BitPiece), color: usize) {
        let mut boards = Vec::new();
        let mut bp = BitPiece(0, 0, color);
        for rot in 0..4 {
            let mut bs = self.0[rot];
            while bs != 0 {
                let pos = bs.trailing_zeros() as usize;
                bp.0 = pos;
                bp.1 = rot;
                let clr_result = base.cleared();
                let mut cleared = BitBoard(clr_result.0);
                cleared.0 |= bp.bit_repr();
                cleared.unclear(clr_result.1);
                // println!("{} {} {}", cleared, bp.0, bp.1);

                boards.push(cleared);
                bs &= !(1 << pos);
            }
        }
        boards
    }
}
impl Placements {
    pub fn to_fumen(&self, color: usize, mut base: BitBoard) -> TetFumen {
        let mut tetfu = TetFumen::new();
        let mut bp = BitPiece(0, 0, color);
        let mut lines = base.clear();
        let mut rows: Vec<ClearedLine> = Vec::new();
        for i in 0..4 {
            if lines[i] == 1 {
                rows.push(ClearedLine(i as isize, vec![PieceColor::from(lines[i] * 8); 10]));
            }
        }

        println!("{:?}", lines);
        let mut dummy_board = base.to_tet_board();
        for rot in 0..4 {
            for pos in 0..40 {
                if self.0[rot] & (1 << pos) == 0 { continue; }
                bp.0 = pos;
                bp.1 = rot;
                let tetpiece = bp.to_tet_piece();
                // print/ln!("{}", dummy_board);
                let mut clone = dummy_board.place_clone(tetpiece);
                println!("{:?}", rows.as_slice());
                clone.refill_rows(&rows);
                tetfu.append_board(clone);
            }
        }
        tetfu
    }
}
impl BitAnd<u64> for Placements {
    type Output = Placements;
    fn bitand(self, rhs: u64) -> Self::Output {
        Placements([self.0[0] & rhs, self.0[1] & rhs, self.0[2] & rhs, self.0[3] & rhs], self.1)
    }
}
impl BitAndAssign<u64> for Placements {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0[0] &= rhs;
        self.0[1] &= rhs;
        self.0[2] &= rhs;
        self.0[3] &= rhs;
    }
}
impl BitAnd<[u64; 4]> for Placements {
    type Output = Placements;
    fn bitand(self, rhs: [u64; 4]) -> Self::Output {
        Placements([self.0[0] & rhs[0], self.0[1] & rhs[1], self.0[2] & rhs[2], self.0[3] & rhs[3]], self.1)
    }
}
impl BitAndAssign<[u64; 4]> for Placements {
    fn bitand_assign(&mut self, rhs: [u64; 4]) {
        self.0[0] &= rhs[0];
        self.0[1] &= rhs[1];
        self.0[2] &= rhs[2];
        self.0[3] &= rhs[3];
    }
}
impl BitBoard {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        let shifted =  (val as u64) << (y * 10 + x);
        self.0 = self.0 & !shifted | shifted;
    }
    pub fn set_bool(&mut self, x: usize, y: usize, val: bool) {
        // println!("{}", (y * 10 + 9 - x));
        let shifted =  (val as u64) << (y * 10 + x);
        self.0 = self.0 & !shifted | shifted;
    }
    pub fn get(&self, x: usize, y: usize) -> u64 {
        self.0 & (1 << (y * 10 + 9 - x)) >> (y * 10 + x)
    }
    pub fn repr(&self) -> String {
        let mut str = String::new();
        
        for i in 0..4 {
            for j in 0..10 {
                if self.0 & (1 << ((3-i) * 10 + j)) != 0 {
                    str.write_char('X');
                } else {
                    str.write_char('_');
                }
            }
            str.write_char('\n');
        }

        str
    }
    #[inline]

    pub fn shift_cleared(&mut self) -> u64 {
        let mut base = self.0;
        
        let a = base & ROW;
        base >>= 10;
        let b = base & ROW;
        base >>= 10;
        let c = base & ROW;
        base >>= 10;
        let d = base & ROW;

        let e = (a == ROW) as u64;
        let f = (b == ROW) as u64;
        let g = (c == ROW) as u64;
        let h = (d == ROW) as u64;
        let lines = e + f + g + h;
        let i = (a != ROW) as u64;
        let j = (b != ROW) as u64;
        let k = (c != ROW) as u64;
        let l = (d != ROW) as u64;

        let mut end: u64 = 0;
        end |= d * l;
        end = end << 10 * k | c * k;
        end = end << 10 * j | b * j;
        end = end << 10 * i | a * i;
        end |= ROW * h;
        end = end << 10 * g | ROW * g;
        end = end << 10 * f | ROW * f;
        end = end << 10 * e | ROW * e;


        
        self.0 = end;

        lines
    }
    #[inline]
    pub fn get_shift_cleared(&self) -> (u64, u64) {
        let mut base = self.0;
        
        let a = base & ROW;
        base >>= 10;
        let b = base & ROW;
        base >>= 10;
        let c = base & ROW;
        base >>= 10;
        let d = base & ROW;

        let e = (a == ROW) as u64;
        let f = (b == ROW) as u64;
        let g = (c == ROW) as u64;
        let h = (d == ROW) as u64;
        let lines = e + f + g + h;
        let i = (a != ROW) as u64;
        let j = (b != ROW) as u64;
        let k = (c != ROW) as u64;
        let l = (d != ROW) as u64;

        let mut end: u64 = 0;
        end |= d * l;
        end = end << 10 * k | c * k;
        end = end << 10 * j | b * j;
        end = end << 10 * i | a * i;
        end |= ROW * h;
        end = end << 10 * g | ROW * g;
        end = end << 10 * f | ROW * f;
        end = end << 10 * e | ROW * e;

        (lines, end)
    }
    pub fn clear(&mut self) -> [u64; 4] {
        let mut base = self.0;
        
        let a = base & ROW;
        base >>= 10;
        let b = base & ROW;
        base >>= 10;
        let c = base & ROW;
        base >>= 10;
        let d = base & ROW;

        let e = (a == ROW) as u64;
        let f = (b == ROW) as u64;
        let g = (c == ROW) as u64;
        let h = (d == ROW) as u64;

        let i = (a != ROW) as u64;
        let j = (b != ROW) as u64;
        let k = (c != ROW) as u64;
        let l = (d != ROW) as u64;
        let ipsum0 = 1 - e;
        let ipsum1 = 2 - e - f;
        let ipsum2 = 3 - e - f - g;
        let ipsum3 = 4 - e - f - g - h;
        let mut end = 0;

        end |= d * l << 10 * ipsum2;
        end |= c * k << 10 * ipsum1;
        end |= b * j << 10 * ipsum0;
        end |= a * i;
        self.0 = end;

        [e, f, g, h]
    }
    pub fn cleared(&self) -> (u64, [u64; 4]) {
        let mut base = self.0;
        
        let a = base & ROW;
        base >>= 10;
        let b = base & ROW;
        base >>= 10;
        let c = base & ROW;
        base >>= 10;
        let d = base & ROW;

        let e = (a == ROW) as u64;
        let f = (b == ROW) as u64;
        let g = (c == ROW) as u64;
        let h = (d == ROW) as u64;

        let i = (a != ROW) as u64;
        let j = (b != ROW) as u64;
        let k = (c != ROW) as u64;
        let l = (d != ROW) as u64;
        let ipsum0 = 1 - e;
        let ipsum1 = 2 - e - f;
        let ipsum2 = 3 - e - f - g;
        let ipsum3 = 4 - e - f - g - h;
        let mut end = 0;

        end |= d * l << 10 * ipsum2;
        end |= c * k << 10 * ipsum1;
        end |= b * j << 10 * ipsum0;
        end |= a * i;


        (end, [e, f, g, h])
        
    }
    pub fn unclear(&mut self, [e, f, g, h]: [u64; 4]) {
        let a = self.0 & ROW;
        let b = self.0 >> 10 & ROW;
        let c = self.0 >> 20 & ROW;
        let d = self.0 >> 30 & ROW;
        let psum0 = e;
        let psum1 = e + f;
        let psum2 = e + f + g;
        let psum3 = e + f + g + h;
        self.0 = 0;
        self.0 |= ROW * e;
        self.0 |= ROW * f << 10;
        self.0 |= ROW * g << 20;
        self.0 |= ROW * h << 30;
        self.0 |= a << 10 * (0 + psum0);
        self.0 |= b << 10 * (1 + psum1);
        self.0 |= c << 10 * (2 + psum2);
        self.0 |= d << 10 * (3 + psum3);
    }
    pub fn collides(&self, piece: BitPiece) -> u64 {
        // println!("{}{}", self, BitBoard::from(piece) );
        return (self.0 & piece.bit_repr() > 0) as u64
    }
    pub fn no_collides(&self, piece: BitPiece) -> u64 {
        return (self.0 & piece.bit_repr() == 0) as u64
    }
    pub fn gen_collision_map(&self, mut piece: BitPiece) -> u64 {
        let (cleared, sums) = self.cleared();
        let mut dummy = BitBoard(cleared);
        let mut base_bounds = BOUND_MAPS[
            piece.2
        ][
            piece.1
        ].1;
        let mut collision_map = base_bounds;
        // println!("{}", BitBoard(collision_map));
        let mut shifted = 0;
        let mut to_shift = 0;
        let mut inv_map = 1;
        while base_bounds.trailing_zeros() < 60  {
            // println!("{} {}", shifted, base_bounds.trailing_zeros());
            to_shift = base_bounds.trailing_zeros();
            inv_map <<= to_shift;
            base_bounds >>= to_shift;
            shifted += to_shift;

            piece.0 = shifted as usize;
            
            let bit = dummy.no_collides(piece) << shifted;

            collision_map = collision_map & !inv_map | bit;
            base_bounds >>= 1;
            shifted += 1;
            inv_map <<= 1;
        }
        collision_map
    }
    pub fn gen_collision_maps(&self, color: usize) -> [u64; 4] {
        let mut collision_maps = [0; 4];
        let cbounds = BOUND_MAPS[color as usize]; 
        let mut piece = BitPiece(0, 0, color);

        for i in 0..4 {
            piece.1 = i;
            collision_maps[i] = self.gen_collision_map(piece);
        }

        collision_maps
    }
    pub fn gen_collision_maps_bounded(&self, color: usize) -> [u64; 4] {
        let mut collision_maps = self.gen_collision_maps(color);
        let (_, mut lines) = self.cleared();
        for i in 0..4 {
            let bounds = BOUND_MAPS[color][i].0;
            let e = lines[0];
            let f = lines[1];
            let g = lines[2];
            let h = lines[3];
            lines[0] = !lines[0] & 1;
            lines[1] = !lines[1] & 1;
            lines[2] = !lines[2] & 1;
            lines[3] = !lines[3] & 1;
            let sum = (e + f + g + h) as i32;
            let psum0 = 0 - sum;
            let psum1 = psum0 + 1;
            let psum2 = psum1 + 1;
            let psum3 = psum2 + 1;

            let mut end = 0;
            let a = bounds & ROW;
            let b = bounds >> 10 & ROW;
            let c = bounds >> 20 & ROW;
            let d = bounds >> 30 & ROW;
            end |= sgn_shl_i32(a * lines[0], 10 * psum0);
            end |= sgn_shl_i32(b * lines[1], 10 * psum1);
            end |= sgn_shl_i32(c * lines[2], 10 * psum2);
            end |= sgn_shl_i32(d * lines[3], 10 * psum3);
            collision_maps[i] &= end;
            lines[0] = !lines[0] & 1;
            lines[1] = !lines[1] & 1;
            lines[2] = !lines[2] & 1;
            lines[3] = !lines[3] & 1;
        }
        collision_maps
    }
    pub fn gen_collision_map_w_bounds(&self, mut piece: BitPiece, mut base_bounds: u64) -> u64 {
        let mut collision_map = base_bounds;
        let mut shifted = 0;
        while base_bounds.trailing_zeros() < 40  {
            shifted += base_bounds.trailing_zeros();
            base_bounds << shifted;
            piece.0 = shifted as usize;
            collision_map &= (0b1 & self.collides(piece)) << shifted;
        }
        collision_map
    }
    pub fn to_tet_board(&self) -> TetBoard {
        let mut board = TetBoard::new();
        for i in 0..40 {
            
            let y = i / 10;
            let x = i % 10;
            let filled = self.0 & (0b1 << i) > 0;
            board.set_tile(x as isize, y as isize, filled as u8 * 8);
        }
        board
    }
}

impl From<TetBoard> for BitBoard {
    fn from(val: TetBoard) -> Self {
        let mut board = BitBoard(0);
        for i in 0..4 {
            for j in 0..10 {
                board.set_bool(j, i, val.get_tile(j as isize, i as isize) != 0);
            }
        }
        board
    }
}
impl From<BitPiece> for BitBoard {
    fn from(val: BitPiece) -> Self {
        BitBoard(val.bit_repr())
    }
}
impl BitBoard {
    pub fn from_placements(placements: &Placements) -> [Self; 4] {
        [
            BitBoard(placements.0[0]),
            BitBoard(placements.0[1]),
            BitBoard(placements.0[2]),
            BitBoard(placements.0[3])
        ]
    }
}
impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}
impl std::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}
pub fn find_placements(base: BitBoard, color: usize) -> Placements { 
    let cmaps = base.gen_collision_maps(color);
    let cmaps_bounded = base.gen_collision_maps_bounded(color);
    let mut placements = Placements::new(cmaps);
    placements.gen_all();
    placements.filter_congruents(color);
    placements.filter_bounds(cmaps_bounded);
    placements
}
pub struct PathLeaf<const len: usize>(BitBoard, [BitPiece; len]);
pub fn fast_path(
    mut base: BitBoard,
    mut queue: BitQueue,
    hold: bool,
    ouptut: &mut HashMap<BitQueue, BitBoard>,
    depth: usize,
    max_depth: usize

) {
    let last = depth == max_depth - 1;
    let a = queue.pop_front();
    
    let placements = find_placements(base, a);
    if !last {
        for bb in placements.create_boards(base, a) {
            fast_path(
                bb,
                queue.clone(),
                hold,
                ouptut,
                depth + 1,
                max_depth
            );
        }
    } else {
        for bb in placements.create_boards(base, a) {
            ouptut.insert(bb);
        }
    }

    // let b = queue.pop_front().unwrap(); 
    // queue.push_front(a);
}