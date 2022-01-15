use crate::direction::Direction;
use crate::errors::ChessError;
use crate::file::{File, FILE_COUNT};
use crate::piece::Color;
use crate::rank::Rank;
use crate::utils::impl_index;
use std::fmt;
use std::mem::transmute;
use std::str::FromStr;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Square {
    A1 = 0o00,
    B1 = 0o01,
    C1 = 0o02,
    D1 = 0o03,
    E1 = 0o04,
    F1 = 0o05,
    G1 = 0o06,
    H1 = 0o07,
    A2 = 0o10,
    B2 = 0o11,
    C2 = 0o12,
    D2 = 0o13,
    E2 = 0o14,
    F2 = 0o15,
    G2 = 0o16,
    H2 = 0o17,
    A3 = 0o20,
    B3 = 0o21,
    C3 = 0o22,
    D3 = 0o23,
    E3 = 0o24,
    F3 = 0o25,
    G3 = 0o26,
    H3 = 0o27,
    A4 = 0o30,
    B4 = 0o31,
    C4 = 0o32,
    D4 = 0o33,
    E4 = 0o34,
    F4 = 0o35,
    G4 = 0o36,
    H4 = 0o37,
    A5 = 0o40,
    B5 = 0o41,
    C5 = 0o42,
    D5 = 0o43,
    E5 = 0o44,
    F5 = 0o45,
    G5 = 0o46,
    H5 = 0o47,
    A6 = 0o50,
    B6 = 0o51,
    C6 = 0o52,
    D6 = 0o53,
    E6 = 0o54,
    F6 = 0o55,
    G6 = 0o56,
    H6 = 0o57,
    A7 = 0o60,
    B7 = 0o61,
    C7 = 0o62,
    D7 = 0o63,
    E7 = 0o64,
    F7 = 0o65,
    G7 = 0o66,
    H7 = 0o67,
    A8 = 0o70,
    B8 = 0o71,
    C8 = 0o72,
    D8 = 0o73,
    E8 = 0o74,
    F8 = 0o75,
    G8 = 0o76,
    H8 = 0o77,
}

pub const SQUARE_COUNT: usize = 64;

impl_index! { Square(SQUARE_COUNT) }

impl Square {
    pub fn new(rank: Rank, file: File) -> Square {
        // unsafe { transmute(file as u8 + rank as u8 * 8) }
        unsafe { transmute((rank.index() * FILE_COUNT + file.index()) as u8) }
    }

    pub fn rank(self) -> Rank {
        unsafe { transmute((self.index() / FILE_COUNT) as u8) }
    }

    pub fn file(self) -> File {
        unsafe { transmute((self.index() % FILE_COUNT) as u8) }
    }

    pub fn up(self) -> Option<Square> {
        Some(Square::new(self.rank().up()?, self.file()))
    }

    pub fn down(self) -> Option<Square> {
        Some(Square::new(self.rank().down()?, self.file()))
    }

    pub fn left(self) -> Option<Square> {
        Some(Square::new(self.rank(), self.file().left()?))
    }

    pub fn right(self) -> Option<Square> {
        Some(Square::new(self.rank(), self.file().right()?))
    }

    pub fn absolute(self, direction: Direction) -> Option<Square> {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),

            Direction::UpLeft => self.up().and_then(|s| s.left()),
            Direction::UpRight => self.up().and_then(|s| s.right()),
            Direction::DownLeft => self.down().and_then(|s| s.left()),
            Direction::DownRight => self.down().and_then(|s| s.right()),
        }
    }

    pub fn relative(self, direction: Direction, color: Color) -> Option<Square> {
        match color {
            Color::White => self.absolute(direction),
            Color::Black => self.absolute(!direction),
        }
    }

    pub fn wrapping_up(self) -> Option<Square> {
        Some(Square::new(self.rank().wrapping_up(), self.file()))
    }

    pub fn wrapping_down(self) -> Option<Square> {
        Some(Square::new(self.rank().wrapping_down(), self.file()))
    }

    pub fn wrapping_left(self) -> Option<Square> {
        Some(Square::new(self.rank(), self.file().wrapping_left()))
    }

    pub fn wrapping_right(self) -> Option<Square> {
        Some(Square::new(self.rank(), self.file().wrapping_right()))
    }
}

impl FromStr for Square {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let rank = chars
            .next()
            .ok_or(ChessError::ParseError("".to_string(), "Rank"))
            .and_then(|c| {
                Rank::from_char(c).ok_or(ChessError::ParseError(c.to_string(), "Rank"))
            })?;
        let file = chars
            .next()
            .ok_or(ChessError::ParseError("".to_string(), "File"))
            .and_then(|c| {
                File::from_char(c).ok_or(ChessError::ParseError(c.to_string(), "File"))
            })?;
        Ok(Square::new(rank, file))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
