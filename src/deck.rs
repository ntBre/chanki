use std::{error::Error, path::Path};

use serde::{Deserialize, Serialize};

use crate::{board::Board, pgn::Pgn, run_convert, run_latex};

#[derive(Serialize, Deserialize)]
pub struct Card {
    /// path to the generated PGN PNG (for now). eventually it will probably
    /// be an FEN that I'll render directly
    pub pgn: String,

    /// solution input by user and displayed directly as given
    pub answer: String,
}

impl Card {
    pub fn new(
        pgn: &Pgn,
        move_number: usize,
        output: String,
        answer: String,
    ) -> Self {
        let mut board = Board::new();
        let moves = board.play(pgn, move_number);

        let dir = std::env::temp_dir().join("chanki");
        // create_dir_all is okay with it already existing
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("test.tex"),
            board.to_latex(*moves.iter().last().unwrap()),
        )
        .unwrap();
        run_latex(dir.to_str().unwrap());
        run_convert(dir.join("test.pdf").to_str().unwrap(), &output);

        Self {
            pgn: output,
            answer,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// load a [Deck] from `path`
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let f = std::fs::File::open(path)?;
        Ok(serde_json::from_reader(f)?)
    }

    /// dump `self` to `path` in JSON format
    pub fn dump(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let mut f = std::fs::File::create(path)?;
        Ok(serde_json::to_writer(&mut f, &self)?)
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card)
    }
}
