use chanki::{deck::Card, pgn::Pgn};
use clap::Parser;
use std::{
    io::{stdin, Read},
    str::FromStr,
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Optional name of a PGN file to read. If absent, read the PGN from
    /// stdin
    pub(crate) pgn: Option<String>,

    /// Halfmove at which to generate the diagram. Black's second move is 4,
    /// for example
    #[arg(short, long)]
    pub(crate) move_number: usize,

    /// Name for the output diagram PNG file
    #[arg(short, long, default_value_t = String::from("out.png"))]
    pub(crate) output: String,

    /// Answer for the card,
    #[arg(short, long)]
    pub(crate) answer: String,
}

fn main() {
    let args = Args::parse();
    let pgn = if let Some(pgn) = args.pgn {
        Pgn::load(pgn).unwrap()
    } else {
        let mut s = String::new();
        stdin().read_to_string(&mut s).unwrap();
        Pgn::from_str(&s).unwrap()
    };

    Card::new(&pgn, args.move_number, args.output, args.answer);
}
