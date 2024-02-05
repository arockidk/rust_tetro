#![allow(dead_code)]
#![allow(unused)]
mod field;
mod piece;
mod fumen;
mod queue;
mod math;
mod kicks;
mod board;
mod vec2;
mod colors;
mod tetra;
#[cfg(test)]
mod tests {
    use crate::field;
    use crate::board;
    use crate::fumen;
    use crate::piece::get_pieces;
    use crate::piece::PieceColor;
    use crate::piece::Direction;
    use crate::queue::Queue;
    use crate::queue::choose;
    use crate::piece;
    use crate::vec2::Vec2;
    

  

    #[test]
    fn fumen_test() {
        let demo = board::Board::new();
        // fumen::encode_fumen(&demo);
        assert_eq!(fumen::encode_fumen(&demo), "v115@vh");
        let mut two_grey = board::Board::new();
        two_grey.set_tile(0, 0, 8);
        two_grey.set_tile(1, 0, 8);
        assert_eq!(fumen::encode_fumen(&two_grey),"v115@B8th");
        let grey_6p_pco = board::Board::from_4h_array([
            8,8,0,0,0,0,0,8,8,8,
            8,8,8,0,0,0,0,8,8,8,
            8,8,8,8,0,0,0,8,8,8,
            8,8,8,0,0,0,0,8,8,8
        ]);
        assert_eq!(fumen::encode_fumen(&grey_6p_pco),"v115@9gB8EeF8DeG8CeF8DeC8Je");
        print!("{}", grey_6p_pco);
    }
    #[test]
    fn queue_test () {
        let allp4 = choose(Vec::from(get_pieces()), 4);
        for queue in allp4 {
            println!("{}", queue)
        }
    }
    #[test]
    fn piece_test () {
        let mut p = piece::Piece::new(
            piece::PieceColor::I,
            piece::Direction::North,
            Vec2(4,21)
        );
        assert_eq!(p.get_minos(), [
            Vec2(-1 + 4, 21), Vec2(0 + 4, 21), Vec2(1 + 4, 21), Vec2(2 + 4, 21)
        ]);
        
        p.apply_gravity(1);
        
        assert_eq!(p.get_minos(), [
            Vec2(3, 20), Vec2(4, 20), Vec2(5, 20), Vec2(6, 20)
        ]);
        p.rotation = Direction::East;
        assert_eq!(p.get_minos(), [
            Vec2(4, 21), Vec2(4, 20), Vec2(4, 19), Vec2(4, 18)    
        ]);
        print!("{}", p);
        
    }
    #[test]
    fn collision_test () {
        let board = board::Board::new();
        let mut p = piece::Piece::new(
            piece::PieceColor::I,
            piece::Direction::North,
            Vec2(9,20)
        );
        assert_eq!(board.does_collide(&p), true);
    }
    #[test]
    fn das_test() {
        let mut i = piece::Piece::new(PieceColor::I, Direction::North, Vec2(4,20));
        let mut f = field::Field::new(board::Board::new(), i);
        f.das_piece(Direction::East);
        f.das_piece(Direction::South);
        print!("{}", f);
        f.rotate_piece(1);
        print!("{}", f);
    }
    #[test]
    fn rotation_test () {
        let s = piece::Piece::new(PieceColor::S, Direction::North, Vec2(4,20));
        let mut standard_s_kick = field::Field::new(board::Board::from_4h_array([
            0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,
            8,8,8,8,8,0,0,8,8,8,
            8,8,8,8,0,0,8,8,8,8
        ]),  s);
        standard_s_kick.rotate_piece(1);
    //    print!("{}", standard_s_kick);
       standard_s_kick.das_piece(Direction::South);
    //    print!("{:?}", standard_s_kick.active_piece.position);
    //    print!("{}", standard_s_kick);
    //    standard_s_kick.active_piece.position += Vec2(1,0);
       print!("{}", standard_s_kick);
       standard_s_kick.rotate_piece(1);
       print!("{}", standard_s_kick);
    }
    #[test]
    //create a new piece for each of the piece colors and print them out with println
    #[allow(non_snake_case)]
    fn piece_color_test() {
        
        let I = piece::Piece::new(PieceColor::I, Direction::North, Vec2(4,20));
        println!("{}", I);
        let L = piece::Piece::new(PieceColor::L, Direction::North, Vec2(4,20));
        println!("{}", L);
        let O = piece::Piece::new(PieceColor::O, Direction::North, Vec2(4,20));
        println!("{}", O);
        let T = piece::Piece::new(PieceColor::T, Direction::North, Vec2(4,20));
        println!("{}", T);
        let J = piece::Piece::new(PieceColor::J, Direction::North, Vec2(4,20));
        println!("{}", J);
        let S = piece::Piece::new(PieceColor::S, Direction::North, Vec2(4,20));
        println!("{}", S);
        let Z = piece::Piece::new(PieceColor::Z, Direction::North, Vec2(4,20));
        println!("{}", Z);
    }
}
