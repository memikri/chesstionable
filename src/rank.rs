use crate::errors::ChessError;
use crate::utils::impl_index;
use std::{fmt, mem::transmute, str::FromStr};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Rank {
    Rank1,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
}

pub const RANK_COUNT: usize = 8;

impl_index! { Rank(RANK_COUNT) }

impl Rank {
    pub fn to_char(self) -> char {
        (self.index() as u8 + b'1') as char
    }

    pub fn from_char(c: char) -> Option<Rank> {
        Rank::from_index((c as usize).wrapping_sub(b'1' as usize))
    }

    pub fn up(self) -> Option<Rank> {
        Rank::from_index(self.index().wrapping_add(1))
    }

    pub fn down(self) -> Option<Rank> {
        Rank::from_index(self.index().wrapping_sub(1))
    }

    pub fn up_n(self, n: usize) -> Option<Rank> {
        Rank::from_index(self.index().wrapping_add(n))
    }

    pub fn down_n(self, n: usize) -> Option<Rank> {
        Rank::from_index(self.index().wrapping_sub(n))
    }

    pub fn wrapping_up(self) -> Rank {
        unsafe { transmute((self.index().wrapping_add(1) % RANK_COUNT) as u8) }
    }

    pub fn wrapping_down(self) -> Rank {
        unsafe { transmute((self.index().wrapping_sub(1) % RANK_COUNT) as u8) }
    }
}

impl FromStr for Rank {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        chars
            .next()
            .ok_or(ChessError::ParseError("".to_string(), "Rank"))
            .and_then(|c| Rank::from_char(c).ok_or(ChessError::ParseError(c.to_string(), "Rank")))
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
