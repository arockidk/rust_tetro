#![allow(dead_code)]
#![allow(unused)]
pub mod board;
pub mod colors;
pub mod field;
pub mod fumen;
pub mod gameplay;
pub mod kicks;
pub mod math;
pub mod pc_utils;
pub mod piece;
pub mod queue;
pub mod vec2;
#[cfg(test)]
pub mod tests {
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io;
    use std::io::BufWriter;
    use std::io::Write;
    use std::usize;

    use fumen::RotationState;

    use crate::board;
    use crate::board::Board;
    use crate::board::TSpinResult;
    use crate::board::TetBoard;
    use crate::field;
    use crate::field::Field;
    use crate::fumen::TetFumen;
    use crate::pc_utils::path;
    use crate::pc_utils::path_entry;
    use crate::pc_utils::PathOptions;
    use crate::pc_utils::PiecePos;
    use crate::pc_utils::PredData;
    use crate::piece;
    use crate::piece::get_pieces;
    use crate::piece::Direction;
    use crate::piece::PieceColor;
    use crate::piece::TetPiece;
    use crate::queue;
    use crate::queue::Choose;
    use crate::queue::Queue;
    use crate::queue::QueueNode;
    use crate::queue::QueueNodeType;
    use crate::vec2::Vec2;

    fn debug_log(thing: &str) {
        println!("{}", thing);
    }

    fn fumen_test() {
        {
            let mut pco = crate::fumen::TetFumen::new();
            let page = pco.add_page_rs();
            page.set_field(field::Field {
                board: board::TetBoard::from_4h_array([
                    8, 8, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 0, 0,
                    0, 8, 8, 8, 8, 8, 8, 0, 0, 0, 0, 8, 8, 8,
                ]),
                active_piece: None,
                hold: None,
            });
            println!("{}", page);
            let encoded_fumen = pco.encode_fumen();
            println!("{}", encoded_fumen);
            unsafe {
                println!(
                    "{:?}",
                    pco.get_page_at(0).get_field().board.get_tile_matrix()
                )
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

    fn queue_test() {
        let test_q = Queue::from_string("TILJSOZ".to_string());
        assert!(test_q.is_ok());
        let test_q = test_q.unwrap();
        for piece in test_q.head().iter() {
            println!("{}", piece);
        }
    }

    fn choose_test() {
        use crate::queue::Choose;
        use crate::queue::QueueNode;
        let allp4 = Queue::new();
        let choose = Choose::from_string("*p4".to_string());
        assert!(choose.is_some());
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
        let ilsz = Choose::from_string("*p7".to_string());
        // assert!(ilsz.is_ok());
        let first_pc = ilsz.unwrap();
        // assert_eq!(first_pc.to_string(), "*p7,*p4".to_string());
        let mut i = 1;
        let iter = first_pc.iter();
        println!("{}", iter.len());
        for q in iter {
            println!("{} {}", i, q);
            i += 1;
            // if (i % 10000 == 0) {
            //     println!("{}", i);
            // }
        }
        println!("{}", i);

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
    fn piece_test() {
        let mut p =
            piece::TetPiece::new(piece::PieceColor::I, piece::Direction::North, Vec2(4, 21));
        assert_eq!(
            p.get_minos(),
            [
                Vec2(-1 + 4, 21),
                Vec2(0 + 4, 21),
                Vec2(1 + 4, 21),
                Vec2(2 + 4, 21)
            ]
        );

        p.apply_gravity(1);

        assert_eq!(
            p.get_minos(),
            [Vec2(3, 20), Vec2(4, 20), Vec2(5, 20), Vec2(6, 20)]
        );
        p.rotation = Direction::East;
        assert_eq!(
            p.get_minos(),
            [Vec2(4, 21), Vec2(4, 20), Vec2(4, 19), Vec2(4, 18)]
        );
        print!("{}", p);
    }
    fn collision_test() {
        let mut board = board::TetBoard::new();
        let mut p =
            piece::TetPiece::new(piece::PieceColor::I, piece::Direction::North, Vec2(9, 20));
        assert_eq!(board.does_collide(p), true);
    }

    fn das_test() {
        let mut i = piece::TetPiece::new(PieceColor::I, Direction::North, Vec2(4, 20));
        let mut f = field::Field::new(board::TetBoard::new(), Some(i), None);
        f.das_piece(Direction::East, 1000);
        f.das_piece(Direction::South, 1000);
        println!("{}", f);
        f.rotate_piece(1);
        assert_eq!(f.active_piece.unwrap().rotation, Direction::East);
        println!("{}", f);
    }

    fn rotation_test() {
        let s = piece::TetPiece::new(PieceColor::S, Direction::North, Vec2(4, 16));
        let mut standard_s_kick = field::Field::new(
            board::TetBoard::from_4h_array([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 0, 0, 8,
                8, 8, 8, 8, 8, 8, 0, 0, 8, 8, 8, 8,
            ]),
            Some(s),
            None,
        );
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
        let mut fm = TetFumen::new();
        fm.add_page_rs().set_field(standard_s_kick);
        println!("{}", fm.encode_fumen());
    }
    //create a new piece for each of the piece colors and print them out with println
    #[allow(non_snake_case)]
    fn piece_color_test() {
        let I = piece::TetPiece::new(PieceColor::I, Direction::North, Vec2(4, 20));
        println!("{}", I);
        let L = piece::TetPiece::new(PieceColor::L, Direction::North, Vec2(4, 20));
        println!("{}", L);
        let O = piece::TetPiece::new(PieceColor::O, Direction::North, Vec2(4, 20));
        println!("{}", O);
        let T = piece::TetPiece::new(PieceColor::T, Direction::North, Vec2(4, 20));
        println!("{}", T);
        let J = piece::TetPiece::new(PieceColor::J, Direction::North, Vec2(4, 20));
        println!("{}", J);
        let S = piece::TetPiece::new(PieceColor::S, Direction::North, Vec2(4, 20));
        println!("{}", S);
        let Z = piece::TetPiece::new(PieceColor::Z, Direction::North, Vec2(4, 20));
        println!("{}", Z);
    }
    #[inline]
    fn pc_test() {
        let mut init_fumen = TetFumen::load(String::from("v115@9gC8FeC8GeE8EeD8FeA8JeAgH"));
        // init_fumen = TetFumen::load(String::from("v115@9gC8wwBeAtRpC8ywBtRpE8i0RpD8BeAtg0RpA8JeAg?H"));
        // init_fumen = TetFumen::load_slice("v115@9gC8CeF8CeR8BeE8JeAgH");
        // init_fumen = TetFumen::load_slice("v115@9gC8ywF8BewwR8BeE8JeAgH");
        init_fumen = TetFumen::load_slice("v115@9gC8FeC8GeN8AeB8BeB8JeAgH"); // Jigsaw
        let mut board = init_fumen.get_page_at(0).get_field().board.clone();
        let pos_pred = |data: PredData| data.piece.unwrap().get_minos().iter().all(|mino: &Vec2| {
            mino.1 < (4 - data.lines_cleared).into()
        });
        let mut file = OpenOptions::new().write(true).open("log.txt").unwrap();
        let mut buff = BufWriter::new(file);
        let mut fum = TetFumen::new();
        let mut boards = Vec::new();
        let mut queue = Queue::from_string(String::from("TJSI")).unwrap();
        // queue = Queue::from_string(String::from("Z")).unwrap();
        let options = PathOptions {
            tetfu: String::from("v115@9gC8FeC8GeN8AeB8BeB8JeAgH"),
            patterns: String::from("TJSI"),
            height: 4,
            hold: true,
            max_boards: usize::MAX,
        };
        path_entry(options, &mut boards);
        for board in boards {
            fum.add_page_rs().set_field(Field::from_board(board));
        }
        buff.write_all(fum.encode_fumen().as_bytes());
        //     let placement = placements[i];
        //     piece.set_piece_pos(placement);
        //     board.place(piece);
        //     let mut new_piece = TetPiece::new(PieceColor::Z, Direction::North, Vec2(0,6));
        //     let new_placements = board.get_piece_placements(new_piece,
        //         Some(&pos_pred)
        //     );
        //     buff.write_all(format!("aaaaaaaaa\n{}", board.no_color_string()).as_bytes());
        //     for new_placement in new_placements {
        //         new_piece.set_piece_pos(new_placement);
        //         board.place(new_piece);
        //         buff.write_all(format!("{}", board.no_color_string()).as_bytes());
        //         fum.add_page_rs().set_field(Field::new(board, None, None));  
        //         let mut new_piece1 = TetPiece::new(PieceColor::S, Direction::North, Vec2(0,6));
        //         let new_placements1 = board.get_piece_placements_pred(new_piece1,
        //             &pos_pred
        //         );
        //         for new_placement1 in new_placements1 {
        //             new_piece1.set_piece_pos(new_placement1);
        //             board.place(new_piece1);
        //             let mut new_piece2 = TetPiece::new(PieceColor::O, Direction::North, Vec2(0,6));
        //             let new_placements2 = board.get_piece_placements_pred(new_piece2,
        //                 &pos_pred
        //             );
        //             for new_placement2 in new_placements2 {
        //                 new_piece2.set_piece_pos(new_placement2);
        //                 board.place(new_piece2);
        //                 let mut new_piece3 = TetPiece::new(PieceColor::Z, Direction::North, Vec2(0,6));
        //                 let new_placements3 = board.get_piece_placements_pred(new_piece3,
        //                     &pos_pred
        //                 );
        //                 for new_placement3 in new_placements3 {
        //                     new_piece3.set_piece_pos(new_placement3);
        //                     board.place(new_piece3);
        //                     board.unplace(new_piece3);
        //                 }
        //                 board.unplace(new_piece2);
        //             }
        //             board.unplace(new_piece1);
        //         }
        //         board.unplace(new_piece);
        //     }
        //     board.unplace(piece);
        // }
        // buff.write_all(fum.encode_fumen().as_bytes());
    }

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
        let mut i = TetPiece::new(PieceColor::I, Direction::North, Vec2(1, 6));
        println!("{}", Field::new(board, Some(i), None));
        board.das_piece(&mut i, Direction::South, 999);
        board.place(i);
        let rows = board.get_filled_rows();
        assert_eq!(rows.len(), 0);
        let mut board = TetBoard::from_4h_array([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 8, 8,
            8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        let rows = board.get_filled_rows();
        assert_eq!(rows.len(), 1);
    }
    fn clear_test() {
        let f = TetBoard::new();
        assert!(f.check_pc());
        let mut pco = TetBoard::from_4h_array([
            8, 8, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 8, 0, 0, 8, 8, 8,
            8, 8, 8, 8, 0, 0, 0, 8, 8, 8, 8,
        ]);
        debug_log(pco.to_string().as_str());
        assert!(!pco.check_pc());
        let mut z = TetPiece::z();
        z.position = Vec2(4, 1);
        // debug_log(Field::new(pco, Some(z), None).to_string().as_str());
        pco.place_n_clear(z);

        let mut i = TetPiece::i();
        i.position = Vec2(3, 2);
        // debug_log(Field::new(pco, Some(i), None).to_string().as_str());
        pco.place_n_clear(i);
        let mut l = TetPiece::l();
        l.position = Vec2(5, 19);
        pco.rotate_piece(&mut l, 3);
        pco.das_piece(&mut l, Direction::South, 9999);
        pco.rotate_piece(&mut l, 1);
        pco.place_n_clear(l);
        assert!(pco.check_pc());
        let mut spin = TetBoard::from_4h_array([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 0,
        ]);
        let mut t = TetPiece::t();
        t.position = Vec2(4, 1);
        debug_log(Field::new(spin, Some(t), None).to_string().as_str());
        assert_eq!(spin.check_t_spin(t), TSpinResult::NoSpin);
        t.position = Vec2(1, 1);
        debug_log(Field::new(spin, Some(t), None).to_string().as_str());
        assert_eq!(spin.check_t_spin(t), TSpinResult::MiniSpin);
        spin.rotate_piece(&mut t, 2);
        debug_log(Field::new(spin, Some(t), None).to_string().as_str());
        assert_eq!(spin.check_t_spin(t), TSpinResult::TSpin);
    }
    fn cleared_test() {
        let mut board = TetFumen::load(String::from("v115@9gD8FeC8GeN8AeB8BeA8JeAgH")).get_page_at(0).get_field().board.clone();
        let mut t = TetPiece::t();
        t.position = Vec2(4,1);
        t.rotation = Direction::West;
        println!("{}", t);
        board.place(t);
        println!("{}", board.quick_fumen_encode());
    }
    #[test]
    fn test() {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                input = String::from(input.trim());

                if input == "clear" {
                    clear_test();
                } else if input == "fumen" {
                    fumen_test();
                } else if input == "queue" {
                    queue_test();
                } else if input == "choose" {
                    choose_test();
                } else if input == "piece" {
                    piece_test();
                } else if input == "collision" {
                    collision_test();
                } else if input == "das" {
                    das_test();
                } else if input == "rotation" {
                    rotation_test();
                } else if input == "piece_color" {
                    piece_color_test();
                } else if input == "pc" {
                    pc_test()
                } else if input == "q" {
                    q_test();
                } else if input == "line" {
                    line_test();
                } else if input == "cleared" {
                    cleared_test();
                }
            }
            Err(error) => println!("error reading input: {error}"),
        }
    }
}
