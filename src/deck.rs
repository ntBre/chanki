use std::{error::Error, path::Path};

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{board::Board, pgn::Pgn, run_convert, run_latex};

#[derive(Serialize, Deserialize)]
pub struct Card {
    /// path to the generated PGN PNG (for now). eventually it will probably
    /// be an FEN that I'll render directly
    pub pgn: String,

    /// solution input by user and displayed directly as given
    pub answer: String,

    /// the next time the card is due to be reviewed
    pub next_due: chrono::DateTime<Utc>,

    /// how many times the card has been reviewed
    pub repetition: usize,

    /// easiness factor for the card
    pub e_factor: f64,
}

/// the quality of a response in SM-2.
///
/// 0 - complete blackout.
/// 1 - incorrect response; the correct one remembered
/// 2 - incorrect response; where the correct one seemed easy to recall
/// 3 - correct response recalled with serious difficulty
/// 4 - correct response after a hesitation
/// 5 - perfect response
#[derive(Clone, Copy)]
pub enum Quality {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
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
            next_due: Utc::now(),
            repetition: 0,
            e_factor: 2.5,
        }
    }

    pub fn update_card(&mut self, q: Quality) {
        // failed the card, restart repetitions from the beginning without
        // touching e_factor
        if (q as usize) < 3 {
            self.repetition = 0;
            self.next_due = Utc::now();
            return;
        }
        // this is the least clear part to me, but it says "After each
        // repetition session of a given day repeat again all items that scored
        // below four in the quality assessment. Continue the repetitions until
        // all of these items score at least four." so do you increment
        // repetition? I think not since they are measured in days
        if (q as usize) < 4 {
            self.next_due = Utc::now();
        }
        self.repetition += 1;
        self.update_e_factor(q);
        self.update_interval();
    }

    fn update_e_factor(&mut self, q: Quality) {
        let a = 5.0 - q as usize as f64;
        self.e_factor += 0.1 - a * (0.08 + a * 0.02);
        if self.e_factor < 1.3 {
            self.e_factor = 1.3;
        }
    }

    pub fn is_due(&self) -> bool {
        Utc::now() > self.next_due
    }

    fn update_interval(&mut self) {
        let days = Self::interval(self.repetition, self.e_factor);
        self.next_due = Utc::now() + Duration::days(days);
    }

    fn interval(n: usize, ef: f64) -> i64 {
        match n {
            1 => 1,
            2 => 6,
            n if n > 2 => (Self::interval(n - 1, ef) as f64 * ef).ceil() as i64,
            // for usize n, this only covers 0, but the compiler doesn't know
            // that exactly
            _ => panic!("unknown n = {n} for interval"),
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
