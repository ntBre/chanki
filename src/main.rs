#![feature(iter_array_chunks, let_chains)]
use std::{
    io::{stdin, Read},
    process::Command,
    str::FromStr,
};

use board::Board;
use cli::Args;
use pgn::Pgn;

mod board;
mod pgn;

#[cfg(test)]
mod tests;

const DEBUG: bool = false;

mod cli {
    use clap::Parser;

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
    }
}

use clap::Parser;

fn run_latex() {
    Command::new("pdflatex")
        .arg("test.tex")
        .output()
        .expect("failed to compile test.tex");
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
    let mut board = Board::new();
    let moves = board.play(&pgn, args.move_number);
    std::fs::write("test.tex", board.to_latex(*moves.iter().last().unwrap()))
        .unwrap();
    run_latex();
}
