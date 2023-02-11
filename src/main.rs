#![feature(iter_array_chunks, let_chains)]
use board::Board;
use pgn::Pgn;

mod board;
mod pgn;

#[cfg(test)]
mod tests;

const DEBUG: bool = false;

fn main() {
    let pgn = Pgn::load("test.pgn").unwrap();
    let mut board = Board::new();
    let moves = board.play(&pgn);
    std::fs::write("test.tex", pgn.to_latex(*moves.iter().last().unwrap()))
        .unwrap();
}
