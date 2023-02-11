#![feature(iter_array_chunks, let_chains)]
use board::Board;
use pgn::Pgn;

use crate::{board::Color, pgn::Move};

mod pgn;

const DEBUG: bool = true;

mod board {
    use std::{
        fmt::Display,
        ops::{Index, IndexMut},
    };

    use crate::DEBUG;

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum PieceType {
        King,
        Queen,
        Rook,
        Bishop,
        Knight,
        Pawn,
    }

    impl Display for PieceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:?}")
        }
    }

    impl From<char> for PieceType {
        fn from(value: char) -> Self {
            use PieceType::*;
            match value {
                'R' => Rook,
                'N' => Knight,
                'B' => Bishop,
                'Q' => Queen,
                'K' => King,
                c => panic!("unrecognized piece type {c}"),
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Color {
        White,
        Black,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:?}")
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Piece {
        typ: PieceType,
        color: Color,
    }

    impl Piece {
        /// check if the [PieceType] in `self` can move from `(from_file,
        /// from_row)` to `(to_file, to_row)`
        fn can_move(
            &self,
            (from_file, from_row): (char, usize),
            (to_file, to_row): (char, usize),
        ) -> bool {
            let ff = to_idx(from_file) as isize;
            let tf = to_idx(to_file) as isize;
            let fr = from_row as isize;
            let tr = to_row as isize;
            let diff = ((tf - ff).abs(), (tr - fr).abs());
            match self.typ {
                PieceType::King => [(0, 1), (1, 0), (1, 1)].contains(&diff),
                PieceType::Queen => match diff {
                    // diagonal
                    (x, y) if x == y => true,
                    // vertical
                    (x, 0) if x > 0 => true,
                    // horizontal
                    (0, y) if y > 0 => true,
                    _ => false,
                },
                PieceType::Rook => match diff {
                    // vertical
                    (x, 0) if x > 0 => true,
                    // horizontal
                    (0, y) if y > 0 => true,
                    _ => false,
                },
                PieceType::Bishop => match diff {
                    (x, y) if x == y => true,
                    _ => false,
                },
                PieceType::Knight => [(1, 2), (2, 1)].contains(&diff),
                PieceType::Pawn => match diff {
                    (1, 0) | (2, 0) | (1, 1) => true,
                    _ => false,
                },
            }
        }
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

    type Coord = (char, usize);

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

        /// make `mov` for `color` on `self` and return the coordinates (from,
        /// to)
        pub(crate) fn mov(
            &mut self,
            mov: String,
            color: Color,
        ) -> (Coord, Coord) {
            if mov.starts_with(['R', 'N', 'B', 'Q', 'K']) {
                // piece move
                let mut mov = mov.chars().peekable();
                let typ = PieceType::from(mov.next().unwrap());
                if let Some(c) = mov.peek() && *c == 'x' {
		    // discard capture indicator
		    mov.next();
		}
                let mut file = mov.next().unwrap();
                // discriminant for moves like Rae8 vs Rfe8
                let mut disc = None;
                if let Some(c) = mov.peek() && c.is_alphabetic() {
		    disc = Some(file);
		    file = mov.next().unwrap();
		}
                let rank =
                    mov.next().unwrap().to_digit(10).unwrap() as usize - 1;
                for r in 0..8 {
                    if let Some(d) = disc {
                        if d.is_numeric() {
                            if d.to_digit(10).unwrap() as usize - 1 != r {
                                continue;
                            }
                        }
                    }
                    for f in 'a'..='h' {
                        let square = self[(f, r)];
                        if let Some(d) = disc {
                            if d.is_alphabetic() {
                                if d != f {
                                    continue;
                                }
                            }
                        }
                        if let Some(p) = square {
                            if p.typ == typ
                                && p.color == color
                                && p.can_move((f, r), (file, rank))
                            {
                                let to = (file, rank);
                                let from = (f, r);
                                self[to] = std::mem::take(&mut self[from]);
                                if DEBUG {
                                    println!(
                                        "moving {color} {typ} \
					      from {f}{r} to {file}{rank}"
                                    );
                                }
                                return (from, to);
                            }
                        }
                    }
                }
            } else if mov == "O-O" {
                // short castles
                if DEBUG {
                    println!("{color} short castles");
                }
                match color {
                    Color::White => {
                        let from = ('e', 0);
                        let to = ('g', 0);
                        self[to] = std::mem::take(&mut self[from]);
                        self[('f', 0)] = std::mem::take(&mut self[('h', 0)]);
                        return (from, to);
                    }
                    Color::Black => {
                        let from = ('e', 7);
                        let to = ('g', 7);
                        self[to] = std::mem::take(&mut self[from]);
                        self[('f', 7)] = std::mem::take(&mut self[('h', 7)]);
                        return (from, to);
                    }
                }
            } else if mov == "O-O-O" {
                // long castles
                if DEBUG {
                    println!("{color} long castles");
                }
                match color {
                    Color::White => {
                        let from = ('e', 0);
                        let to = ('c', 0);
                        self[to] = std::mem::take(&mut self[from]);
                        self[('d', 0)] = std::mem::take(&mut self[('a', 0)]);
                        return (from, to);
                    }
                    Color::Black => {
                        let from = ('e', 7);
                        let to = ('c', 7);
                        self[to] = std::mem::take(&mut self[from]);
                        self[('d', 7)] = std::mem::take(&mut self[('a', 7)]);
                        return (from, to);
                    }
                }
            } else {
                // pawn move
                let mut mov = mov.chars().peekable();
                let file = mov.next().unwrap();
                if let Some(c) = mov.peek() && *c == 'x' {
		    // pawn capture
		    mov.next();
		    let new_file = mov.next().unwrap();
		    let rank = mov.next().unwrap().to_digit(10).unwrap() as usize - 1;
		    match color {
			Color::White => {
			    let from = (file, rank - 1);
			    let to = (new_file, rank);
			    self[to] = std::mem::take(&mut self[from]);
			    if DEBUG {
				println!("pawn {file}{} takes {new_file}{}", rank - 1, rank);
			    }
			    return (from, to);
			}
			Color::Black => {
			    let from = (file, rank + 1);
			    let to = (new_file, rank);
			    self[to] = std::mem::take(&mut self[from]);
			    if DEBUG {
				println!("pawn {file}{} takes {new_file}{}", rank + 1, rank);
			    }
				return (from, to);;
			}
		    }
		} else {
		    let rank = mov.next().unwrap().to_digit(10).unwrap() as usize - 1;
		    // if there is a pawn 1 square away, move that one, else
		    // move one two squares away
		    match color {
			Color::White => {
			    if let Some(p) = self[(file, rank-1)] && p.typ == PieceType::Pawn {
				let from = (file, rank -1);
				let to = (file, rank);
				self[to] = std::mem::take(&mut self[from]);
				if DEBUG {
				    println!("pawn {file}{} to {file}{}", rank-1, rank);
				}
				return (from, to);;
			    } else if let Some(p) = self[(file, rank-2)] && p.typ == PieceType::Pawn {
				let from = (file, rank - 2);
				let to = (file, rank);
				self[to] = std::mem::take(&mut self[from]);
				if DEBUG {
				    println!("pawn {file}{} to {file}{}", rank-2, rank);
				}
				return (from, to);;
			    }
			}
			Color::Black => {
			    if let Some(p) = self[(file, rank+1)] && p.typ == PieceType::Pawn && p.color == color {
				let from = (file, rank +1);
				let to = (file, rank);
				self[to] = std::mem::take(&mut self[from]);
				if DEBUG {
				    println!("pawn {file}{} to {file}{}", rank+1, rank);
				}
				return (from, to);;
			    } else if let Some(p) = self[(file, rank+2)] && p.typ == PieceType::Pawn && p.color == color {
				let from = (file, rank + 2);
				let to = (file, rank);
				self[to] = std::mem::take(&mut self[from]);
				if DEBUG {
				    println!("pawn {file}{} to {file}{}", rank+2, rank);
				}
				return (from, to);;
			    }
			}
		    }
		}
            }
            unreachable!();
        }
    }

    /// convert the char file to a usize
    const fn to_idx(c: char) -> usize {
        c as usize - 'a' as usize
    }

    impl Index<(char, usize)> for Board {
        type Output = Option<Piece>;

        fn index(&self, (file, rank): (char, usize)) -> &Self::Output {
            &self.squares[7 - rank][to_idx(file)]
        }
    }

    impl IndexMut<(char, usize)> for Board {
        fn index_mut(
            &mut self,
            (file, rank): (char, usize),
        ) -> &mut Self::Output {
            &mut self.squares[7 - rank][to_idx(file)]
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
    let mut board = Board::new();
    for Move { turn, white, black } in pgn.moves {
        if DEBUG {
            print!("{turn:>3}. ");
        }
        board.mov(white, Color::White);
        if DEBUG {
            print!(" ... ");
        }
        board.mov(black, Color::Black);
    }
}
