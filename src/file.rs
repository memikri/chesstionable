use crate::errors::ChessError;
use crate::utils::impl_index;
use std::{fmt, mem::transmute, str::FromStr};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

pub const FILE_COUNT: usize = 8;

impl_index! { File(FILE_COUNT) }

impl File {
    pub fn to_char(self) -> char {
        (self.index() as u8 + b'a') as char
    }

    pub fn from_char(c: char) -> Option<File> {
        File::from_index((c as usize).wrapping_sub(b'a' as usize))
    }

    pub fn left(self) -> Option<File> {
        File::from_index(self.index().wrapping_sub(1))
    }

    pub fn right(self) -> Option<File> {
        File::from_index(self.index().wrapping_add(1))
    }

    pub fn left_n(self, n: usize) -> Option<File> {
        File::from_index(self.index().wrapping_sub(n))
    }

    pub fn right_n(self, n: usize) -> Option<File> {
        File::from_index(self.index().wrapping_add(n))
    }

    pub fn wrapping_left(self) -> File {
        unsafe { transmute((self.index().wrapping_sub(1) % FILE_COUNT) as u8) }
    }

    pub fn wrapping_right(self) -> File {
        unsafe { transmute((self.index().wrapping_add(1) % FILE_COUNT) as u8) }
    }
}

impl FromStr for File {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        chars
            .next()
            .ok_or(ChessError::ParseError("".to_string(), "File"))
            .and_then(|c| File::from_char(c).ok_or(ChessError::ParseError(c.to_string(), "File")))
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
