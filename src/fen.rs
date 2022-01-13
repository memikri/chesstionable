use core::fmt;
use std::str::FromStr;

use crate::{
    piece::*, // too lazy to import individually
    square::{File, Square},
    state::{Castling, Ply, State},
};
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum FENToken {
    #[token("p")]
    BlackPawn,
    #[token("n")]
    BlackKnight,
    #[token("b")]
    BlackBishop,
    #[token("r")]
    BlackRook,
    #[token("q")]
    BlackQueen,
    #[token("k")]
    BlackKing,
    //
    #[token("P")]
    WhitePawn,
    #[token("N")]
    WhiteKnight,
    #[token("B")]
    WhiteBishop,
    #[token("R")]
    WhiteRook,
    #[token("Q")]
    WhiteQueen,
    #[token("K")]
    WhiteKing,
    //
    #[regex(r"[1-8]", |lex| lex.slice().parse::<u8>().unwrap())]
    Empty(u8),
    #[token("/")]
    Slash,
    //
    #[error]
    Error,
    // metadata handled separately
}

#[derive(Logos, Debug, PartialEq)]
pub enum FENMetadata {
    #[token("w")]
    White,
    #[token("b")]
    Black,
    #[token("-")]
    Dash,
    #[token("K")]
    WhiteCanCastleKingside,
    #[token("Q")]
    WhiteCanCastleQueenside,
    #[token("k")]
    BlackCanCastleKingside,
    #[token("q")]
    BlackCanCastleQueenside,
    #[regex(r"[a-h][1-8]", |lex| Square::from_str(lex.slice()).unwrap())]
    EnPassant(Square),
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<u16>().unwrap())]
    Ply(u16),
    #[token(" ", logos::skip)]
    Whitespace,
    #[error]
    Error,
}

#[derive(Debug)]
pub enum FENError {
    InvalidToken,
    InvalidMetadata,
    Overflow,
}

impl std::error::Error for FENError {}

impl fmt::Display for FENError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FENError::InvalidToken => write!(f, "FEN error token reached"),
            FENError::InvalidMetadata => write!(f, "FEN metadata error token reached"),
            FENError::Overflow => write!(f, "FEN error: too much data"),
        }
    }
}

impl State {
    // Result<State> for parse errors?
    pub fn from_fen(fen: &str) -> Result<State, FENError> {
        // pkbqrpn = black pieces
        // PKBQRPN = white pieces
        // '/' = new rank
        // [1-8] = empty squares

        let (pieces, metadata) = fen.split_at(fen.find(' ').unwrap()); // TODO remove unwrap

        let lexer = FENToken::lexer(pieces);
        let mut state = State::new();
        let mut square = Square::A8;

        for token in lexer.into_iter() {
            match token {
                FENToken::BlackPawn => state.board.set_piece(square, Some(BLACK_PAWN)),
                FENToken::BlackKnight => state.board.set_piece(square, Some(BLACK_KNIGHT)),
                FENToken::BlackBishop => state.board.set_piece(square, Some(BLACK_ROOK)),
                FENToken::BlackRook => state.board.set_piece(square, Some(BLACK_ROOK)),
                FENToken::BlackQueen => state.board.set_piece(square, Some(BLACK_QUEEN)),
                FENToken::BlackKing => state.board.set_piece(square, Some(BLACK_KING)),
                FENToken::WhitePawn => state.board.set_piece(square, Some(WHITE_PAWN)),
                FENToken::WhiteKnight => state.board.set_piece(square, Some(WHITE_KNIGHT)),
                FENToken::WhiteBishop => state.board.set_piece(square, Some(WHITE_BISHOP)),
                FENToken::WhiteRook => state.board.set_piece(square, Some(WHITE_ROOK)),
                FENToken::WhiteQueen => state.board.set_piece(square, Some(WHITE_QUEEN)),
                FENToken::WhiteKing => state.board.set_piece(square, Some(WHITE_KING)),
                FENToken::Empty(num) =>
                    for _ in 0..num {
                        state.board.set_piece(square, None);
                        square = Square::new(square.file().next().unwrap(), square.rank());
                    },
                FENToken::Slash => {
                    square = Square::new(File::FileA, square.rank().prev().unwrap());
                }
                FENToken::Error => return Err(FENError::InvalidToken),
            }

            if state.board.piece(square).is_some() {
                square = Square::new(square.file().next().unwrap(), square.rank());
            }
        }

        let meta_lexer = FENMetadata::lexer(metadata);

        let mut color: Option<Color> = None;
        let mut ply: Option<Ply> = None;
        let mut halfmove_clock: Option<Ply> = None;

        for token in meta_lexer.into_iter() {
            match token {
                FENMetadata::White => color = Some(Color::White),
                FENMetadata::Black => color = Some(Color::Black),
                FENMetadata::Dash => (),
                FENMetadata::WhiteCanCastleKingside => state.castling.white_king = true,
                FENMetadata::WhiteCanCastleQueenside => state.castling.white_queen = true,
                FENMetadata::BlackCanCastleKingside => state.castling.black_king = true,
                FENMetadata::BlackCanCastleQueenside => state.castling.black_queen = true,
                FENMetadata::EnPassant(square) => state.en_passant = Some(square),
                FENMetadata::Ply(v) =>
                    if halfmove_clock.is_none() {
                        halfmove_clock = Some(v);
                    } else if ply.is_none() {
                        ply = Some(v * 2);
                    } else {
                        return Err(FENError::Overflow);
                    },
                FENMetadata::Error => return Err(FENError::InvalidMetadata),
                FENMetadata::Whitespace => unreachable!(),
            }
        }

        state.ply = ply.unwrap() + color.unwrap() as Ply;
        state.halfmove_clock = halfmove_clock.unwrap();

        Ok(state)
    }

    fn rank_fen(&self, rank: usize) -> String {
        if rank > 7 {
            panic!("Invalid rank");
        }
        // i'm gonna make ranges work with files and ranks and squares
        // so you can do stuff like: for rank in Rank::Rank1..Rank::Rank8
        // how would that work with Board?
        // i think it's better to loop over the "indices" (meaning our enums) cause enumerate doesn't give you our enums

        let mut fen = String::new();
        let mut empty_count: u8 = 0;
        for (i, piece) in self.board.board.iter().rev().enumerate() {
            if i % 8 == rank {
                // TODO: is this correct?
                if empty_count > 0 {
                    fen.push_str(&empty_count.to_string());
                    empty_count = 0;
                }
                if piece.is_some() {
                    fen.push_str(&piece.unwrap().as_str());
                } else {
                    empty_count += 1;
                }
            }
        }
        if empty_count > 0 {
            fen.push_str(&empty_count.to_string());
        }
        fen
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for rank in 0..8 {
            fen.push_str(&self.rank_fen(7 - rank));
            if rank < 7 {
                fen.push('/');
            }
        }
        fen.push(' ');
        fen.push(if self.side_to_move() == Color::White {
            'w'
        } else {
            'b'
        });
        fen.push(' ');
        if self.castling.white_king {
            fen.push('K');
        }
        if self.castling.white_queen {
            fen.push('Q');
        }
        if self.castling.black_king {
            fen.push('k');
        }
        if self.castling.black_queen {
            fen.push('q');
        }
        if !self.castling.white_king
            && !self.castling.white_queen
            && !self.castling.black_king
            && !self.castling.black_queen
        {
            fen.push('-');
        }
        fen.push(' ');
        if let Some(square) = self.en_passant {
            fen.push_str(&square.to_string());
        } else {
            fen.push('-');
        }
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.ply.to_string());
        fen
    }
}
