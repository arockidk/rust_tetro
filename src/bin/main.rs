use std::collections::HashSet;

use rust_tetro::{board::Board, field::Field, fumen::TetFumen, pc_utils::{path_entry, PathOptions, PredData}, piece::{piece_color_from_char, piece_color_from_str, piece_color_to_char, TetPiece}, queue::Queue, vec2::Vec2};
use getargs::{self, Arg, Opt, Options};
// v115@9gC8FeC8GeE8EeD8FeA8JeAgH
fn main() {
    let mut args = std::env::args().skip(1) .collect::<Vec<_>>();
    if args.is_empty() {
        args.push(String::from("--help"));
    }
    let mut opts = Options::new(args.iter().map(String::as_str));
    let mut command = "";
    let mut piece = None;
    let mut height = 4;
    let mut fum_string = "";
    let mut hold = false;
    let mut patterns = "";
    while let Some(arg) = opts.next_arg().expect("argument parsing error") {
        match arg {
            Arg::Short('h') | Arg::Long("help") => {
                match command {
                    "placements" => {
                        eprintln!(
                            r"Usage: rust_tetro placements [args]
-t, --tetfu:    fumen input
--piece:        piece to get placements of
--height:       max height to search for piece placements (default: 4)
        "
                        );
                    },
                    "path" => {
                        eprintln!(
                            r"Usage: rust_tetro path [args]
-t, --tetfu:    fumen input
-p, --patterns: patterns to search
--hold:     use hold piece or not (default: true)
--height:       max height to search (default: 4)
")
                    }
                    _ => {
                        eprintln!(
                            r"Usage: rust_tetro <command> [args]
        
            <command>:
                - placements: gets all possible placements of a piece on a fumen
        "
                        );
                    }
                }
                return;
            },
            Arg::Long("height") => {
                let value = opts.value().unwrap();
                height = value.parse().expect("Expected number value for option \"height\"");
            },
            Arg::Long("piece") => {
                let value = opts.value().unwrap();
                piece = Some(piece_color_from_str(value.to_uppercase().as_str()));
            },
            Arg::Long("hold") => {
                let value = opts.value().unwrap();
                hold = value.parse().expect("Expected boolean value for option \"hold\"");
            },
            Arg::Short('t') | Arg::Long("tetfu") => {
                let value = opts.value().unwrap();
                fum_string = value;
            },
            Arg::Short('p') | Arg::Long("patterns") => {
                let value = opts.value().unwrap();
                patterns = value;
            },

            Arg::Positional(arg) => {
                command = arg;

            }
            _ => eprintln!("option: {:?}", arg),
        }
    }
    match command {
        "placements" => {
            assert!(piece.is_some(), "No piece provided");
            let piece = piece.unwrap();
            let fum = TetFumen::load_slice(fum_string);
            for i in 0..fum.len() {
                let field = fum.get_page_at(i).field();
                let mut board = field.board;
                board.to_gray();
                println!("Finding placements of piece {} on board {}", piece_color_to_char(piece), board.quick_fumen_encode());
                println!("Max height: {}", height); 
                let mut tetp = TetPiece::from((piece, Vec2(0, height + 2)));
                let pos_pred = |data: PredData| data.piece.unwrap().get_minos().iter().all(|mino: &Vec2| {
                    mino.1 < (height - data.lines_cleared as i32).into()
                });
                let placements = board.get_piece_placements(tetp, Some(&pos_pred));
                println!("{:?}", placements);
                let mut place_fum = TetFumen::new();
                for placement in placements {
                    tetp.set_piece_pos(placement);
                    let placed = board.place_clone(tetp);
                    let mut pg = place_fum.add_page_rs();
                    pg.set_field(Field::from_board(placed));
                }
                println!("Placements fumen: {}", place_fum.encode_fumen());
            }
        },
        "path" => {
            let fum = TetFumen::load_slice(fum_string);
            
            for i in 0..fum.len() {
                println!("Searching patterns: {}", patterns);
                let mut output = HashSet::new();
                let options = PathOptions {
                    tetfu: fum_string.to_string(),
                    height: height.try_into().unwrap(),
                    hold,
                    max_boards: usize::MAX,
                    patterns: patterns.to_string(),
                };
                path_entry(options, &mut output);
                println!("Found {} boards", output.len());
                let mut result = TetFumen::new();
                for board in output {
                    let pg = result.add_page_rs();
                    pg.set_field(Field::from_board(board));
                }
                println!("Result: {}", result.encode_fumen());
            }
        }
        _ => {}
    }

}