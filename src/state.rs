use crate::bitboard::*;
use crate::errors::ChessError;
use crate::piece::*;
use crate::rank::Rank;
use crate::square::*;
use std::fmt;
use std::mem::transmute;
use std::str::FromStr;

pub type Ply = u16;

pub struct Board {
    pub board: [Option<Piece>; 64],
    pub color_bb: [BitBoard; 2],
    pub piece_bb: [BitBoard; 6],
}

// TODO: impl Iterator for RankIter

// do you know about fn() -> impl Iterator<Item=Rank>? no

// you can do stuff like
// How coudl that work with a Board though to access (Square, Piece)

pub fn ranks() -> impl Iterator<Item = Rank> {
    (0..8).map(Rank::from_index).map(Option::unwrap)
}

pub fn squares() -> impl Iterator<Item = Square> {
    (0..64).map(Square::from_index).map(Option::unwrap)
}

impl Board {
    pub fn pieces(&self) -> impl Iterator<Item = (Square, Option<Piece>)> + '_ {
        squares().map(move |square| (square, self.board[square.index()]))
        // this might work?
        // thanks copilot

        // basically if you can construct an iterator from other iterators you can just make kind of like a generator function that returns a "compound" iterator
        // the return values there is actually something like Map<Item=Square, IntoIter=FilterMap<Item=Square, IntoIter=Map<.........
        // but with impl Trait the compiler just figures it out
    }
}

#[derive(Debug, Clone, Copy)] // TODO: formatable using FEN-style notation (kKqQ etc.)
pub struct Castling {
    pub white_king: bool,
    pub black_king: bool,
    pub white_queen: bool,
    pub black_queen: bool,
}

impl FromStr for Castling {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut castling = Castling {
            white_king: false,
            black_king: false,
            white_queen: false,
            black_queen: false,
        };

        if s == "-" {
            return Ok(castling);
        }

        for c in s.chars() {
            match c {
                'K' => castling.white_king = true,
                'Q' => castling.white_queen = true,
                'k' => castling.black_king = true,
                'q' => castling.black_queen = true,
                _ => return Err(ChessError::ParseError(s.to_string(), "Castling")),
            }
        }

        Ok(castling)
    }
}

impl fmt::Display for Castling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut castling = String::new();

        if self.white_king {
            castling.push('K');
        }

        if self.white_queen {
            castling.push('Q');
        }

        if self.black_king {
            castling.push('k');
        }

        if self.black_queen {
            castling.push('q');
        }

        if castling.is_empty() {
            write!(f, "-")
        } else {
            write!(f, "{}", castling)
        }
    }
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
        for (i, rank) in self.board.board.iter().enumerate() {
            if rank.is_some() {
                write!(f, "{} ", rank.unwrap().as_str())?;
            } else {
                write!(f, ". ")?;
            }
            if i % 8 == 7 {
                write!(f, "\n")?;
            }
        }
        write!(
            f,
            "ply = {}, ep = {:?}, castling = {}\n",
            self.ply, self.en_passant, self.castling
        )?;
        Ok(())
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [None; 64],
            color_bb: [EMPTY; 2],
            piece_bb: [EMPTY; 6],
        }
    }

    pub fn piece(&self, square: Square) -> Option<Piece> {
        self.board[square.index()]
    }

    pub fn set_piece(&mut self, square: Square, piece: Option<Piece>) {
        let bb = BitBoard::from_square(square);
        if let Some(old_piece) = self.board[square.index()] {
            self.color_bb[old_piece.color().index()] ^= bb;
            self.piece_bb[old_piece.piece_type().index()] ^= bb;
        }
        self.board[square.index()] = piece;
        if let Some(piece) = piece {
            self.color_bb[piece.color().index()] ^= bb;
            self.piece_bb[piece.piece_type().index()] ^= bb;
        }
    }

    pub fn by_color(&self, color: Color) -> BitBoard {
        self.color_bb[color.index()]
    }

    pub fn by_piece_type(&self, piece_type: PieceType) -> BitBoard {
        self.piece_bb[piece_type.index()]
    }

    pub fn by_piece(&self, piece: Piece) -> BitBoard {
        self.by_color(piece.color()) & self.by_piece_type(piece.piece_type())
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
        unsafe { transmute((self.ply % 2) as u8) }
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
