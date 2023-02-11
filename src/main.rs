#![feature(iter_array_chunks, let_chains)]
use board::Board;
use pgn::Pgn;

use crate::{board::Color, pgn::Move};

mod board;
mod pgn;

#[cfg(test)]
mod tests;

const DEBUG: bool = false;

fn main() {
    let pgn = Pgn::load("test.pgn").unwrap();
    let mut board = Board::new();
    let mut moves = Vec::new();
    for Move { turn, white, black } in &pgn.moves {
        if DEBUG {
            print!("{turn:>3}. ");
        }
        moves.push(board.mov(white, Color::White));
        if DEBUG {
            print!(" ... ");
        }
        moves.push(board.mov(black, Color::Black));
    }
    std::fs::write("test.tex", pgn.to_latex(*moves.iter().last().unwrap()))
        .unwrap();
}
