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

        /// Name for the output diagram PNG file
        #[arg(short, long, default_value_t = String::from("out.png"))]
        pub(crate) output: String,
    }
}

use clap::Parser;

fn run_latex(dir: &str) {
    Command::new("pdflatex")
        .args(["-output-directory", dir, "test.tex"])
        .output()
        .expect("failed to compile test.tex");
}

fn run_convert(pdf: &str, png: &str) {
    Command::new("convert")
        .args(["-density", "300", pdf, "-quality", "90", png])
        .output()
        .expect("failed to convert test.tex to out.png");
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

    let dir = std::env::temp_dir().join("chanki");
    // create_dir_all is okay with it already existing
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("test.tex"),
        board.to_latex(*moves.iter().last().unwrap()),
    )
    .unwrap();
    run_latex(dir.to_str().unwrap());
    run_convert(dir.join("test.pdf").to_str().unwrap(), &args.output);
}
