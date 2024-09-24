use rust_tetro::board::Board;
use getargs::{self, Arg, Opt, Options};
fn main() {
    let mut args = std::env::args().skip(1) .collect::<Vec<_>>();
    if args.is_empty() {
        args.push(String::from("--help"));
    }
    let mut opts = Options::new(args.iter().map(String::as_str));
    let mut command = "";
    
    while let Some(arg) = opts.next_arg().expect("argument parsing error") {
        match arg {
            Arg::Short('h') | Arg::Long("help") => {
                match command {
                    "placements" => {
                        eprintln!(
                            r"Usage: rust_tetro placements [args]
-t, --tetfu: fumen input
-p, --piece: piece to get placements of
        "
                        );
                    },
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
            Arg::Short("p") || Arg::Long("piece")       
            Arg::Positional(arg) => {
                command = arg;

            }
            _ => eprintln!("option: {:?}", arg),
        }
    }

}