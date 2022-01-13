use crate::fen::*;
use crate::piece::*;
use crate::square::*;

use logos::Logos;
use std::str::FromStr;
use std::{error::Error, fmt};

// ok i think we need a better interface for state
// we'll probably change this quite a lot when optimizing (and probably bitboards and stuff later) and it's best to write the fen parsing code in a way that it doesn't depend on the internals of this structure

pub type Ply = u16;

pub struct Board {
    pub board: [Option<Piece>; 64], // maybe just an array of 64? should be same performance  * // yeah i think 64 makes for cleaner code
}

#[derive(Debug)] // TODO: formatable using FEN-style notation (kKqQ etc.)
pub struct Castling {
    pub white_king: bool,
    pub black_king: bool,
    pub white_queen: bool,
    pub black_queen: bool,
}

pub struct State {
    pub board: Board,
    pub ply: Ply,
    pub halfmove_clock: Ply,
    pub en_passant: Option<Square>,
    pub castling: Castling,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in self.board.board.iter().rev() {
            for piece in rank {
                if let Some(piece) = piece {
                    write!(f, "{}", piece.as_str())?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(
            f,
            "ply = {}, ep = {:?}, castling = {:?}\n",
            self.ply, self.en_passant, self.castling
        )?;
        Ok(())
    }
}

impl Board {
    pub fn new() -> Board {
        Board { board: [None; 64] }
    }

    pub fn piece(&self, square: Square) -> Option<Piece> {
        self.board[square as usize]
    }

    pub fn set_piece(&mut self, square: Square, piece: Option<Piece>) {
        self.board[square as usize] = piece;
    }
}

impl Default for State {
    fn default() -> State {
        State::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}

impl State {
    pub fn new() -> State {
        // empty board for FEN parsing
        State {
            board: Board::new(),
            ply: 0,
            halfmove_clock: 0,
            en_passant: None,
            castling: Castling {
                white_king: false,
                black_king: false,
                white_queen: false,
                black_queen: false,
            },
        }
    }

    // maybe Side is a better name for Color?
    pub fn side_to_move(&self) -> Color {
        if self.ply % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn ply(&self) -> Ply {
        self.ply
    }

    pub fn set_ply(&mut self, ply: Ply) {
        self.ply = ply;
    }

    pub fn halfmove_clock(&self) -> Ply {
        self.halfmove_clock
    }

    pub fn set_halfmove_clock(&mut self, halfmove_clock: Ply) {
        self.halfmove_clock = halfmove_clock;
    }

    pub fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    pub fn set_en_passant(&mut self, en_passant: Option<Square>) {
        self.en_passant = en_passant;
    }

    pub fn castling_rights(&self) -> Castling {
        self.castling
    }

    pub fn set_castling_rights(&mut self, castling_rights: Castling) {
        self.castling = castling_rights;
    }
}
