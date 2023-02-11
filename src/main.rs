#![feature(iter_array_chunks)]
use board::Board;
use pgn::Pgn;

mod pgn;

mod board {
    use std::fmt::Display;

    #[derive(Clone, Copy)]
    enum PieceType {
        King,
        Queen,
        Rook,
        Bishop,
        Knight,
        Pawn,
    }

    #[derive(Clone, Copy)]
    enum Color {
        White,
        Black,
    }

    #[derive(Clone, Copy)]
    struct Piece {
        typ: PieceType,
        color: Color,
    }

    pub struct Board {
        squares: [[Option<Piece>; 8]; 8],
    }

    macro_rules! black {
	($($piece:expr$(,)*)*) => {
	    [
		$(Some(Piece { typ: $piece, color: Color::Black }),)*
	    ]
	}
    }

    macro_rules! white {
	($($piece:expr$(,)*)*) => {
	    [
		$(Some(Piece { typ: $piece, color: Color::White }),)*
	    ]
	}
    }

    impl Board {
        pub fn new() -> Self {
            use PieceType::*;
            Self {
                squares: [
                    //
                    black!(
                        Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook
                    ),
                    black!(Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn,),
                    [None; 8],
                    [None; 8],
                    [None; 8],
                    [None; 8],
                    white!(Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn,),
                    white!(
                        Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook
                    ),
                ],
            }
        }
    }

    impl Display for Board {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut empty = 0;
            for (i, row) in self.squares.iter().enumerate() {
                for piece in row {
                    if let Some(p) = piece {
                        if empty > 0 {
                            write!(f, "{empty}")?;
                        }
                        empty = 0;
                        write!(
                            f,
                            "{}",
                            match (p.color, p.typ) {
                                (Color::White, PieceType::King) => "K",
                                (Color::White, PieceType::Queen) => "Q",
                                (Color::White, PieceType::Rook) => "R",
                                (Color::White, PieceType::Bishop) => "B",
                                (Color::White, PieceType::Knight) => "N",
                                (Color::White, PieceType::Pawn) => "P",
                                (Color::Black, PieceType::King) => "k",
                                (Color::Black, PieceType::Queen) => "q",
                                (Color::Black, PieceType::Rook) => "r",
                                (Color::Black, PieceType::Bishop) => "b",
                                (Color::Black, PieceType::Knight) => "n",
                                (Color::Black, PieceType::Pawn) => "p",
                            }
                        )?;
                    } else {
                        empty += 1;
                    }
                }
                if empty > 0 {
                    write!(f, "{empty}")?;
                }
                empty = 0;
                write!(f, "{}", if i < 7 { "/" } else { " " })?;
            }
            // for now just say it's white's move, all castling possible, no en
            // passant targets, no halfmoves since capture or pawn advance, and
            // move 1
            write!(f, "w KQkq - 0 1")
        }
    }
}

fn main() {
    let pgn = Pgn::load("test.pgn").unwrap();
    std::fs::write("test.tex", pgn.to_latex()).unwrap();
    let board = Board::new();
    println!("{board}");
}
