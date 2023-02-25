#![feature(iter_array_chunks, let_chains)]

use std::process::Command;

pub mod board;
pub mod deck;
pub mod pgn;

#[cfg(test)]
mod tests;

const DEBUG: bool = false;

/// hard-coded path to single deck for now. eventually use
/// $HOME/.config/chanki/deck.json
pub const DECK_PATH: &str = "test_deck.json";

pub fn run_latex(dir: &str) {
    Command::new("pdflatex")
        .args(["-output-directory", dir, "test.tex"])
        .output()
        .expect("failed to compile test.tex");
}

pub fn run_convert(pdf: &str, png: &str) {
    Command::new("convert")
        .args(["-density", "300", pdf, "-quality", "90", png])
        .output()
        .expect("failed to convert test.tex to out.png");
}
