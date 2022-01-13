use std::error::Error;
use std::fmt;
use std::iter::{DoubleEndedIterator, ExactSizeIterator, Iterator};
use std::mem::transmute;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum File {
    FileA = 0,
    FileB = 1,
    FileC = 2,
    FileD = 3,
    FileE = 4,
    FileF = 5,
    FileG = 6,
    FileH = 7,
}
// maybe it woudl be better to have someething like:
/*
    for rank in Board::ranks() {
        for file in Board::files() {
            let square = Square::new(file, rank);
            let neighbors = square.neighbors();
            let above = square.above(); // should square be aware of the board somehow?
            // for square.above() it doesn't need to be // oh right lmao I'm too tired for this lmao
            // yooo we could make a 3d chess mode :)
            // unless we're gonna have different size boards but fuck that
            println!("{}", square);
        }
    }
*/
// maybe have ranks() and files() be static though because it doesn't really depend on the state

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rank {
    Rank1 = 0,
    Rank2 = 1,
    Rank3 = 2,
    Rank4 = 3,
    Rank5 = 4,
    Rank6 = 5,
    Rank7 = 6,
    Rank8 = 7,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

// btw can you help with the fen iterator thing? // show me

#[derive(Debug)]
pub enum ParseSquareError {
    InvalidFile(char),
    InvalidRank(char),
    InvalidLength,
}

impl Error for ParseSquareError {}

impl fmt::Display for ParseSquareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseSquareError::InvalidFile(c) => write!(f, "Invalid file: {}", c),
            ParseSquareError::InvalidRank(c) => write!(f, "Invalid rank: {}", c),
            ParseSquareError::InvalidLength => write!(f, "Invalid length"),
        }
    }
}

impl File {
    pub fn next(&self) -> Option<File> {
        match *self {
            File::FileA => Some(File::FileB),
            File::FileB => Some(File::FileC),
            File::FileC => Some(File::FileD),
            File::FileD => Some(File::FileE),
            File::FileE => Some(File::FileF),
            File::FileF => Some(File::FileG),
            File::FileG => Some(File::FileH),
            File::FileH => None,
        }
    }

    pub fn wrapping_next(&self) -> File {
        unsafe { transmute((*self as u8).wrapping_add(1) % 8) }
    }

    pub fn prev(&self) -> Option<File> {
        match *self {
            File::FileA => None,
            File::FileB => Some(File::FileA),
            File::FileC => Some(File::FileB),
            File::FileD => Some(File::FileC),
            File::FileE => Some(File::FileD),
            File::FileF => Some(File::FileE),
            File::FileG => Some(File::FileF),
            File::FileH => Some(File::FileG),
        }
    }

    pub fn wrapping_prev(&self) -> File {
        unsafe { transmute((*self as u8).wrapping_sub(1) % 8) }
    }

    fn from_char(c: char) -> Result<Self, ParseSquareError> {
        Ok(match c {
            'a' | 'A' => File::FileA,
            'b' | 'B' => File::FileB,
            'c' | 'C' => File::FileC,
            'd' | 'D' => File::FileD,
            'e' | 'E' => File::FileE,
            'f' | 'F' => File::FileF,
            'g' | 'G' => File::FileG,
            'h' | 'H' => File::FileH,
            c => return Err(ParseSquareError::InvalidFile(c)),
        })
    }
}

impl Rank {
    pub fn next(&self) -> Option<Rank> {
        match *self {
            Rank::Rank1 => Some(Rank::Rank2),
            Rank::Rank2 => Some(Rank::Rank3),
            Rank::Rank3 => Some(Rank::Rank4),
            Rank::Rank4 => Some(Rank::Rank5),
            Rank::Rank5 => Some(Rank::Rank6),
            Rank::Rank6 => Some(Rank::Rank7),
            Rank::Rank7 => Some(Rank::Rank8),
            Rank::Rank8 => None,
        }
    }

    pub fn wrapping_next(&self) -> Rank {
        unsafe { transmute((*self as u8).wrapping_add(1) % 8) }
    }

    pub fn prev(&self) -> Option<Rank> {
        match *self {
            Rank::Rank1 => None,
            Rank::Rank2 => Some(Rank::Rank1),
            Rank::Rank3 => Some(Rank::Rank2),
            Rank::Rank4 => Some(Rank::Rank3),
            Rank::Rank5 => Some(Rank::Rank4),
            Rank::Rank6 => Some(Rank::Rank5),
            Rank::Rank7 => Some(Rank::Rank6),
            Rank::Rank8 => Some(Rank::Rank7),
        }
    }

    pub fn wrapping_prev(&self) -> Rank {
        unsafe { transmute((*self as u8).wrapping_sub(1) % 8) }
    }

    fn from_char(c: char) -> Result<Self, ParseSquareError> {
        Ok(match c {
            '1' => Rank::Rank1,
            '2' => Rank::Rank2,
            '3' => Rank::Rank3,
            '4' => Rank::Rank4,
            '5' => Rank::Rank5,
            '6' => Rank::Rank6,
            '7' => Rank::Rank7,
            '8' => Rank::Rank8,
            c => return Err(ParseSquareError::InvalidRank(c)),
        })
    }
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Square {
        unsafe { transmute(file as u8 + rank as u8 * 8) }
    }

    pub fn next(&self) -> Option<Square> {
        match *self {
            Square::H8 => None,
            _ => Some(self.wrapping_next()),
        }
    }

    pub fn wrapping_next(&self) -> Square {
        unsafe { transmute((*self as u8).wrapping_add(1) % 8) }
    }

    pub fn prev(&self) -> Option<Square> {
        match *self {
            Square::A1 => None,
            _ => Some(self.wrapping_prev()),
        }
    }

    pub fn wrapping_prev(&self) -> Square {
        unsafe { transmute((*self as u8).wrapping_sub(1) % 8) }
    }

    pub fn file(&self) -> File {
        unsafe { transmute(*self as u8 % 8) }
    }

    pub fn rank(&self) -> Rank {
        unsafe { transmute(*self as u8 / 8) }
    }
}

impl FromStr for Square {
    type Err = ParseSquareError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let file = File::from_char(chars.next().ok_or(ParseSquareError::InvalidLength)?)?;
        let rank = Rank::from_char(chars.next().ok_or(ParseSquareError::InvalidLength)?)?;
        match chars.next() {
            None => Ok(Square::new(file, rank)),
            Some(_) => Err(ParseSquareError::InvalidLength),
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "abcdefgh".chars().nth(*self as usize).unwrap())
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (*self as u8 + 1))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

impl Iterator for RangeInclusive<File> {
    // try running the "implement default members" quick fix on exactsizeiterator
    // i don't have that
    // ctrl+. while on the token?
    // yeah i don't have any quickfixes
    // weird
    // here
    // probably live share being fuck again
    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let next = self.start.next().unwrap();
            self.start = next;
            Some(next)
        }
    }
}

impl DoubleEndedIterator for RangeInclusive<File> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let next = self.end.prev().unwrap();
            self.end = next;
            Some(next)
        }
    }
}

// I gtg to bed, I'll leave the liveshare open
// nah dw about it i should also go to bed
// it's 5:52 am here

// lmao I thought so
// ok let's pick this back up tmr somtime
// when are you free?

// uh like 10pm GMT ish
// i'll be here :ok_hand:

impl ExactSizeIterator for RangeInclusive<File> {
    fn len(&self) -> usize {
        self.end.wrapping_sub(self.start) as usize + 1
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // that's copilot's take
}
