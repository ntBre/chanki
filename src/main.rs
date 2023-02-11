#![feature(iter_array_chunks)]
use pgn::Pgn;

mod pgn {
    use std::{error::Error, fmt::Display, fs::read_to_string, path::Path};

    #[derive(Debug)]
    struct Move {
        turn: usize,
        white: String,
        black: String,
    }

    impl Move {
        fn new(turn: usize, white: String, black: String) -> Self {
            Self { turn, white, black }
        }
    }

    #[derive(Debug)]
    pub struct Pgn {
        moves: Vec<Move>,
    }

    #[derive(Debug)]
    pub struct ParseError;

    impl Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:?}")
        }
    }

    impl Error for ParseError {}

    impl Pgn {
        /// load a PGN from `path`
        pub fn load<P>(path: P) -> Result<Self, Box<dyn Error>>
        where
            P: AsRef<Path>,
        {
            let s = read_to_string(path)?;
            let Some(start) = s.lines().position(str::is_empty) else {
		return Err(Box::new(ParseError));
	    };
            let game: Vec<_> = s.lines().skip(start + 1).collect();
            let game = game.join(" ");
            Ok(Self {
                moves: game
                    .split_ascii_whitespace()
                    .array_chunks::<3>()
                    .map(|[mov, white, black]| {
                        Move::new(
                            mov.trim_end_matches('.').parse().unwrap(),
                            white.to_owned(),
                            black.to_owned(),
                        )
                    })
                    .collect(),
            })
        }
    }
}

fn main() {
    let pgn = Pgn::load("test.pgn").unwrap();
    dbg!(pgn);
}
