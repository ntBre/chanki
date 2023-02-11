use std::{error::Error, fmt::Display, fs::read_to_string, path::Path};

#[derive(Debug)]
pub struct Move {
    pub turn: usize,
    pub white: String,
    pub black: String,
}

impl Move {
    fn new(turn: usize, white: String, black: String) -> Self {
        Self { turn, white, black }
    }
}

#[derive(Debug)]
pub struct Pgn {
    pub moves: Vec<Move>,
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
        // locate the newline between tags and game itself
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

    pub fn to_latex(&self) -> String {
        // TODO take markmoves from pairs of to/from, but first I have to
        // play over the game with a board representation
        format!(
            r#"
\documentclass{{standalone}}
\usepackage{{xskak}}
\begin{{document}}
\newchessgame
\hidemoves{{{self}}}
\chessboard[showmover=false, pgfstyle=straightmove, markmoves={{a1-c3}}]
\end{{document}}
"#
        )
    }
}

impl Display for Pgn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for Move { turn, white, black } in &self.moves {
            write!(f, "{turn}. {white} {black} ")?;
        }
        Ok(())
    }
}
