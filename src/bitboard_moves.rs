use crate::bitboard::*;
use crate::direction::*;
use crate::file::*;
use crate::piece::*;
use crate::rank::*;
use crate::square::*;

#[derive(Default, Copy, Clone)]
pub struct PextBitBoard {
    mask: BitBoard,
    offset: usize,
}

pub struct PextTable {
    table: [BitBoard; 107648],
    rooks: [PextBitBoard; 64],
    bishops: [PextBitBoard; 64],
}

impl BitBoard {
    pub fn pawn_moves(self, color: Color) -> BitBoard {
        match color {
            Color::White => self.up(),
            Color::Black => self.down(),
        }
    }

    pub fn pawn_pushes(self, color: Color) -> BitBoard {
        match color {
            Color::White => self.up().up(),
            Color::Black => self.down().down(),
        }
    }

    pub fn pawn_attacks(self, color: Color) -> BitBoard {
        let moves = self.pawn_moves(color);
        moves.left() | moves.right()
    }

    pub fn knight_moves(self) -> BitBoard {
        let v1 = self.up() | self.down();
        let v2 = self.up_n(2) | self.down_n(2);
        // let v2 = self.up().up() | self.down().down();
        // let h1 = v1.left().left() | v1.right().right();
        let h1 = v1.left_n(2) | v1.right_n(2);
        let h2 = v2.left() | v2.right();
        h1 | h2
    }

    pub fn king_moves(self) -> BitBoard {
        let v = self.up() | self.down();
        let h = v | self;
        h.left() | h.right() | v
    }
}

impl PextBitBoard {
    pub fn new<F: Fn(BitBoard, Square) -> BitBoard>(
        table: &mut [BitBoard],
        index: &mut usize,
        square: Square,
        gen: F,
    ) -> PextBitBoard {
        let h = (BitBoard::from_file(File::A) | BitBoard::from_file(File::H))
            & !BitBoard::from_file(square.file());
        let v = (BitBoard::from_rank(Rank::Rank1) | BitBoard::from_rank(Rank::Rank8))
            & !BitBoard::from_rank(square.rank());

        let mut bb = EMPTY;
        let result = PextBitBoard {
            mask: gen(EMPTY, square) & !(h | v),
            offset: *index,
        };

        loop {
            table[result.index(bb)] = gen(bb, square);
            bb = BitBoard(bb.0.wrapping_sub(result.mask.0)) & result.mask;
            *index += 1;
            if bb == EMPTY {
                break;
            }
        }

        result
    }

    pub fn index(&self, bb: BitBoard) -> usize {
        (unsafe { std::arch::x86_64::_pext_u64(bb.0, self.mask.0) }) as usize + self.offset
    }
}

pub fn ray_attacks(bb: BitBoard, square: Square, direction: &[Direction]) -> BitBoard {
    let mut result = BitBoard::from_square(square);
    for _ in 0..8 {
        let mut tmp = result;
        for dir in direction {
            tmp = tmp.shift(*dir);
        }
        result |= tmp & !bb;
    }
    result & !BitBoard::from_square(square)
}

pub fn rook_attacks(bb: BitBoard, square: Square) -> BitBoard {
    let mut result = EMPTY;
    result |= ray_attacks(bb, square, &[Direction::Up]);
    result |= ray_attacks(bb, square, &[Direction::Down]);
    result |= ray_attacks(bb, square, &[Direction::Left]);
    result |= ray_attacks(bb, square, &[Direction::Right]);
    result
}

pub fn bishop_attacks(bb: BitBoard, square: Square) -> BitBoard {
    let mut result = EMPTY;
    result |= ray_attacks(bb, square, &[Direction::Up, Direction::Left]);
    result |= ray_attacks(bb, square, &[Direction::Down, Direction::Left]);
    result |= ray_attacks(bb, square, &[Direction::Up, Direction::Right]);
    result |= ray_attacks(bb, square, &[Direction::Down, Direction::Right]);
    result
}

impl PextTable {
    pub fn new() -> PextTable {
        let mut index = 0;
        let mut table = PextTable {
            table: [EMPTY; 107648],
            rooks: [PextBitBoard::default(); 64],
            bishops: [PextBitBoard::default(); 64],
        };

        for i in 0..64 {
            table.rooks[i] = PextBitBoard::new(
                &mut table.table,
                &mut index,
                Square::from_index(i).unwrap(),
                rook_attacks,
            );
            table.bishops[i] = PextBitBoard::new(
                &mut table.table,
                &mut index,
                Square::from_index(i).unwrap(),
                bishop_attacks,
            );
        }

        table
    }

    pub fn rook_moves(&self, bb: BitBoard, sq: Square) -> BitBoard {
        self.table[self.rooks[sq.index()].index(bb)]
    }

    pub fn bishop_moves(&self, bb: BitBoard, sq: Square) -> BitBoard {
        self.table[self.bishops[sq.index()].index(bb)]
    }

    pub fn queen_moves(&self, bb: BitBoard, sq: Square) -> BitBoard {
        self.rook_moves(bb, sq) | self.bishop_moves(bb, sq)
    }
}
