use std::{
    error::Error, fmt::Display, fs::read_to_string, path::Path, str::FromStr,
};

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
        Ok(Self::from_str(&s)?)
    }
}

impl FromStr for Pgn {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // locate the newline between tags and game itself
        let Some(start) = s.lines().position(str::is_empty) else {
		    return Err(ParseError);
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

impl Display for Pgn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for Move { turn, white, black } in &self.moves {
            write!(f, "{turn}. {white} {black} ")?;
        }
        Ok(())
    }
}
