use crate::{
    bitboard::{BitBoard, EMPTY},
    bitboard_moves::PextTable,
    direction::Direction,
    square::Square,
};

use super::{byteboard::ByteBoard, bytesquare::PieceType};

#[derive(Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
    pub capture: bool,
}

impl ByteBoard {
    pub fn moves(&self) -> Vec<Move> {
        // Generate a bitboard of all possible moves for each square.
        // For each bitboard, generate all possible moves.
        // For each move, check if it's legal. (TODO)
        let mut res = Vec::new();
        let pext_table = PextTable::new();

        for from_idx in self.occupied() {
            let from = self.get(from_idx);
            let from_piece = from.piece_type();
            let from_color = from.color();

            let mut moves = EMPTY;
            let mut captures = EMPTY;

            match from_piece {
                PieceType::Bishop => {
                    moves |= pext_table.bishop_moves(
                        BitBoard::from_square(Square::from_index(from_idx).unwrap())
                            & !self.bitboard(),
                        Square::from_index(from_idx).unwrap(),
                    );
                }
                PieceType::Rook => {
                    // moves |= pext_table
                    //     .rook_moves(self.bitboard(), Square::from_index(from_idx).unwrap());
                    moves |= pext_table.rook_moves(
                        BitBoard::from_square(Square::from_index(from_idx).unwrap())
                            & !self.bitboard(),
                        Square::from_index(from_idx).unwrap(),
                    );
                }
                PieceType::Queen => {
                    // moves |= pext_table
                    //     .queen_moves(self.bitboard(), Square::from_index(from_idx).unwrap());
                    moves |= pext_table.queen_moves(
                        BitBoard::from_square(Square::from_index(from_idx).unwrap())
                            & !self.bitboard(),
                        Square::from_index(from_idx).unwrap(),
                    );
                }
                PieceType::King => {
                    // moves |= self.bitboard().king_moves();
                    moves |= BitBoard::from_square(Square::from_index(from_idx).unwrap())
                        .king_moves()
                        & !self.bitboard();
                }
                PieceType::Knight => {
                    // moves |= self.bitboard().knight_moves();
                    moves |= BitBoard::from_square(Square::from_index(from_idx).unwrap())
                        .knight_moves()
                        & !self.bitboard();
                }
                PieceType::Pawn => {
                    // moves |= self.bitboard().pawn_moves(from_color);
                    // captures |= self.bitboard().pawn_captures(from_color);
                    // TODO
                }
            };

            for to_idx in moves {
                let to = self.get(to_idx.index());
                let to_piece = to.piece_type();
                let to_color = to.color();

                if !to.occupied() {
                    res.push(Move {
                        from: Square::from_index(from_idx).unwrap(),
                        to: Square::from_index(to_idx.index()).unwrap(),
                        promotion: None,
                        capture: false,
                    });
                } else if to_color != from_color {
                    res.push(Move {
                        from: Square::from_index(from_idx).unwrap(),
                        to: Square::from_index(to_idx.index()).unwrap(),
                        promotion: None,
                        capture: true,
                    });
                }
            }
        }

        res
    }
}
