#![allow(dead_code)]
#![allow(unused)]
pub mod field;
pub mod piece;
pub mod fumen;
pub mod queue;
pub mod math;
pub mod kicks;
pub mod board;
pub mod vec2;
pub mod colors;
pub mod pc_utils;
pub mod gameplay;
#[cfg(test)]
pub mod tests {
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::BufWriter;
    use std::io::Write;

    use fumen::RotationState;

    use crate::board::Board;
    use crate::board::TetBoard;
    use crate::field;
    use crate::board;
    use crate::field::Field;
    use crate::pc_utils::PiecePos;
    use crate::piece::get_pieces;
    use crate::piece::PieceColor;
    use crate::piece::Direction;
    use crate::piece::TetPiece;
    use crate::queue::Queue;
    use crate::queue::choose;
    use crate::piece;
    use crate::queue::QueueNode;
    use crate::queue::QueueNodeType;
    use crate::vec2::Vec2;
    use crate::fumen::TetFumen;
    

  

    #[test]
    fn fumen_test() {
        {
            let mut pco = crate::fumen::TetFumen::new();
            let page = pco.add_page_rs();
            page.set_field(field::Field { board: board::TetBoard::from_4h_array([
                8,8,0,0,0,0,0,8,8,8,
                8,8,8,0,0,0,0,8,8,8,
                8,8,8,8,0,0,0,8,8,8,
                8,8,8,0,0,0,0,8,8,8
            ]), active_piece: None, hold: None});
            println!("{}", page);
            let encoded_fumen = pco.encode_fumen();
            println!("{}", encoded_fumen);
            unsafe {
                println!("{:?}", pco.get_page_at(0).get_field().board.get_tile_matrix())
            }
        }
        // {
        //     let mut ms2 = TetFumen::new();
        //     ms2.decode_fumen("v115@VghlGewhhlGewhhlh0Eewhhlg0DeR4xhQ4g0AeBtR4?RpwhR4BeBtwwRpwhg0Q4AeBtxwRpwhi0AeBtwwRpJeAgH".to_string());
        //     println!("{}", ms2.get_page_at(0));
        //     let encoded = ms2.encode_fumen();
        //     println!("{}", encoded);
        //     ms2.decode_fumen(encoded);
        //     println!("{}", ms2.get_page_at(0));
        // }

        
    }
    #[test]
    fn queue_test () {
        let test_q = Queue::from_string("TILJSOZ".to_string());
        assert!(test_q.is_ok());
        let test_q = test_q.unwrap();
        for piece in test_q.head().iter() {
            println!("{}", piece);
        }
    }
    #[test]
    fn choose_test() {
        use crate::queue::QueueNode;
        use crate::queue::Choose;
        let allp4 = Queue::new();
        let choose = Choose::from_string(
            "*p4".to_string()
        );
        assert!(choose.is_ok());
        let choose = choose.unwrap();
        // println!("{:?} {} {}", choose.pieces, choose.count, choose.inverse);
        assert_eq!(choose.to_string(), "*p4".to_string());
        // let hill_1st_11 = Queue::from_string(
        //     "ILSO,[TJZ]!,*p4".to_string()
        // );
        // assert!(hill_1st_11.is_ok());
        // let hill_1st_11 = hill_1st_11.unwrap();
        // // println!("{}", hill_1st_11.to_string());
        // println!("{}", hill_1st_11.iter().size());
        // let mut i = 1;
        // for q in hill_1st_11.iter() {
        //     println!("{} {}", i, q);
        //     i += 1;
        // }
        let ilsz = Queue::from_string(
            "ILSZ,[^ILSZ]!,*p4".to_string()
        );
        assert!(ilsz.is_ok());
        let first_pc = ilsz.unwrap();
        // assert_eq!(first_pc.to_string(), "*p7,*p4".to_string());
        let mut i = 1;
        for q in first_pc.possible_q_iter() {
            println!("{}", i);  
            i += 1;
        }
        // let queues = choose.get_queues();
        // assert_eq!(queues.len(), 840);
        // for queue in queues {
        //     println!("{}", queue);
        // }
        // for q in choose.iter() {
        //     println!("{}", q);
            
        // }
        // let hill_1st = Choose::from_string(String::from("[ILSO]!"));
        // let mut hill_1st = hill_1st.unwrap();
        // for q in hill_1st.iter() {
        //     println!("{}", q);
        // }

    }
    #[test]
    fn piece_test () {
        let mut p = piece::TetPiece::new(
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
        let board = board::TetBoard::new();
        let mut p = piece::TetPiece::new(
            piece::PieceColor::I,
            piece::Direction::North,
            Vec2(9,20)
        );
        assert_eq!(board.does_collide(p), true);
    }
    #[test]
    fn das_test() {
        let mut i = piece::TetPiece::new(PieceColor::I, Direction::North, Vec2(4,20));
        let mut f = field::Field::new(board::TetBoard::new(), Some(i), None);
        f.das_piece(Direction::East, 1000);
        f.das_piece(Direction::South, 1000);
        println!("{}", f);
        f.rotate_piece(1);
        assert_eq!(f.active_piece.unwrap().rotation, Direction::East);
        println!("{}", f);
    }
    #[test]
    fn rotation_test () {
        let s = piece::TetPiece::new(PieceColor::S, Direction::North, Vec2(4,16));
        let mut standard_s_kick = field::Field::new(board::TetBoard::from_4h_array([
            0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,
            8,8,8,8,8,0,0,8,8,8,
            8,8,8,8,0,0,8,8,8,8
        ]),  Some(s), None);
        print!("{}", standard_s_kick);
        standard_s_kick.rotate_piece(1);
        print!("{}", standard_s_kick);
        println!("{:?}", standard_s_kick.active_piece);
    //    print!("{}", standard_s_kick);    
       standard_s_kick.das_piece(Direction::South, 1000);
    //    print!("{:?}", standard_s_kick.active_piece.position);
    //    print!("{}", standard_s_kick);
    //    standard_s_kick.active_piece.position += Vec2(1,0);
       print!("{}", standard_s_kick);
       standard_s_kick.rotate_piece(1);
       print!("{}", standard_s_kick);
       print!("{}", standard_s_kick.can_place_active_piece());
    }
    #[test]
    //create a new piece for each of the piece colors and print them out with println
    #[allow(non_snake_case)]
    fn piece_color_test() {
        
        let I = piece::TetPiece::new(PieceColor::I, Direction::North, Vec2(4,20));
        println!("{}", I);
        let L = piece::TetPiece::new(PieceColor::L, Direction::North, Vec2(4,20));
        println!("{}", L);
        let O = piece::TetPiece::new(PieceColor::O, Direction::North, Vec2(4,20));
        println!("{}", O);
        let T = piece::TetPiece::new(PieceColor::T, Direction::North, Vec2(4,20));
        println!("{}", T);
        let J = piece::TetPiece::new(PieceColor::J, Direction::North, Vec2(4,20));
        println!("{}", J);
        let S = piece::TetPiece::new(PieceColor::S, Direction::North, Vec2(4,20));
        println!("{}", S);
        let Z = piece::TetPiece::new(PieceColor::Z, Direction::North, Vec2(4,20));
        println!("{}", Z);
    }
    #[test]
    fn pc_test() {
        // use crate::pc_utils::u64_field;
        // let mut board = u64_field::new();
        // board.set_tile(4, 2, 1);
        // println!("{}", board);
        // let mut t = piece::TetPiece::new(PieceColor::T, Direction::North, Vec2(0,0));
        // assert_eq!(board.does_collide(t), true);
        // t.position += Vec2(1,0);
        // assert_eq!(board.does_collide(t), false);
        // assert_eq!(board.can_place(t), true);
        let mut board = TetBoard::new();
        let mut piece = TetPiece::new(PieceColor::T, Direction::North, Vec2(0,1));
        let placements = board.get_piece_placements(piece);
        println!("{:?}", placements);
        let mut new_piece = TetPiece::new(PieceColor::J, Direction::North, Vec2(0,4));
        let mut file = OpenOptions::new().write(true).open("log.txt").unwrap();
        let mut buff = BufWriter::new(file);
        for placement in placements {

            piece.set_piece_pos(placement);
            board.place(piece);
            let new_placements = board.get_piece_placements(new_piece);

            for new_placement in new_placements {
                new_piece.set_piece_pos(new_placement);

                board.place(new_piece);
                // buff.write_all(board.to_string().as_bytes());
                board.unplace(new_piece);
                
            }
            board.unplace(piece);

        }
    }
    #[test]
    fn q_test() {
        use crate::queue::QueueNodeType::Piece;
        let mut a = Queue::new();
        let mut b = Queue::new();
        a.insert_piece(PieceColor::I);
        a.insert_piece(PieceColor::J);
        assert_eq!(a.len(), 2);
        b.insert_piece(PieceColor::L);
        b.insert_piece(PieceColor::S);
        assert_eq!(b.len(), 2);
        a.append(b);
        println!("{}", a);
        assert_eq!(a.len(), 4);
    }
    #[test]
    fn line_test() {
        let mut board = TetBoard::new();
        let rows = board.get_filled_rows();
        assert_eq!(rows.len(), 0);

        board.set_tile(0, 0, 8);
        assert_eq!(rows.len(), 0);

        for j in 1..9 {
            board.set_tile(j, 0, 8);
            let rows = board.get_filled_rows();
            assert_eq!(rows.len(), 0);
        }

        
        board.set_tile(9, 0, 8);
        let rows = board.get_filled_rows();
        assert_eq!(rows.len(), 1);
        for j in 0..10 {
            board.clear_tile(j, 0);
        }
        let mut i = TetPiece::new(PieceColor::I, Direction::North, Vec2(1,6));
        println!("{}", Field::new(board, Some(i), None));
        board.das_piece(&mut i, Direction::South, 999);
        board.place(i);
        let rows = board.get_filled_rows();
        assert_eq!(rows.len(), 0);
        let mut board = TetBoard::from_4h_array([
            0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,
            8,8,8,8,8,8,8,8,8,8,
            0,0,0,0,0,0,0,0,0,0
        ]);
        let rows = board.get_filled_rows();
        assert_eq!(rows.len(), 1);
    }
}
