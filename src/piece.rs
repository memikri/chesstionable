use std::{mem::transmute, str::FromStr};

use crate::{errors::ChessError, utils::impl_index};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl FromStr for Color {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" | "W" => Ok(Color::White),
            "b" | "B" => Ok(Color::Black),
            _ => Err(ChessError::ParseError(s.to_string(), "Color")),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

pub const COLOR_COUNT: usize = 2;
pub const PIECE_TYPE_COUNT: usize = 6;
pub const PIECE_COUNT: usize = COLOR_COUNT * PIECE_TYPE_COUNT;

impl_index! { Color(COLOR_COUNT) }
impl_index! { PieceType(PIECE_TYPE_COUNT) }
impl_index! { Piece(PIECE_COUNT) }

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Piece {
        unsafe { transmute((color.index() + piece_type.index() * 2) as u8) }
    }

    pub fn piece_type(self) -> PieceType {
        unsafe { transmute((self.index() / 2) as u8) }
    }

    pub fn color(self) -> Color {
        unsafe { transmute((self.index() % 2) as u8) }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            Piece::WhitePawn => "P",
            Piece::BlackPawn => "p",
            Piece::WhiteKnight => "N",
            Piece::BlackKnight => "n",
            Piece::WhiteBishop => "B",
            Piece::BlackBishop => "b",
            Piece::WhiteRook => "R",
            Piece::BlackRook => "r",
            Piece::WhiteQueen => "Q",
            Piece::BlackQueen => "q",
            Piece::WhiteKing => "K",
            Piece::BlackKing => "k",
        }
    }
}

impl FromStr for Piece {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Piece, ChessError> {
        match s {
            "P" => Ok(Piece::WhitePawn),
            "p" => Ok(Piece::BlackPawn),
            "N" => Ok(Piece::WhiteKnight),
            "n" => Ok(Piece::BlackKnight),
            "B" => Ok(Piece::WhiteBishop),
            "b" => Ok(Piece::BlackBishop),
            "R" => Ok(Piece::WhiteRook),
            "r" => Ok(Piece::BlackRook),
            "Q" => Ok(Piece::WhiteQueen),
            "q" => Ok(Piece::BlackQueen),
            "K" => Ok(Piece::WhiteKing),
            "k" => Ok(Piece::BlackKing),
            _ => Err(ChessError::ParseError(s.to_string(), "Piece")),
        }
    }
}
