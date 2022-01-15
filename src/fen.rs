use std::str::FromStr;

use crate::{
    errors::ChessError,
    file::File,
    piece::*, // too lazy to import individually
    rank::Rank,
    square::Square,
    state::{Castling, State},
};

impl State {
    fn map_empty_fen(s: &str) -> Option<&str> {
        if s == "-" {
            None
        } else {
            Some(s)
        }
    }

    pub fn from_fen(fen: &str) -> Result<State, ChessError> {
        // pkbqrpn = black pieces
        // PKBQRPN = white pieces
        // '/' = new rank
        // [1-8] = empty squares

        let (pieces, metadata) = fen.split_at(fen.find(' ').unwrap()); // TODO remove unwrap

        let mut state = State::new();
        let ranks: Vec<&str> = pieces.split('/').collect();
        if ranks.len() != 8 {
            return Err(ChessError::InvalidFEN("Invalid number of ranks".into()));
        }

        for (rank_index, rank_data) in ranks.iter().enumerate() {
            let rank = Rank::from_index(rank_index).expect("Invalid rank index");
            let mut file = File::A;
            for piece in rank_data.chars() {
                if piece.is_digit(10) {
                    file = file
                        .right_n(piece.to_digit(10).unwrap() as usize - 1)
                        .ok_or(ChessError::InvalidFEN("Invalid file index".into()))?;
                } else {
                    let piece = Piece::from_str(&piece.to_string())?;
                    let square = Square::new(rank, file);
                    state.board.set_piece(square, Some(piece));
                    if file != File::H {
                        file = file
                            .right()
                            .ok_or(ChessError::InvalidFEN("Invalid file index".into()))?;
                    }
                }
            }
        }

        let mut metadata_iter = metadata.split(' ').filter(|s| !s.is_empty());
        let _turn = metadata_iter // TODO: what to do with this?
            .next()
            .ok_or(ChessError::InvalidFEN("Missing turn".into()))?
            .parse::<Color>()?;
        let castling = metadata_iter
            .next()
            .ok_or(ChessError::InvalidFEN("Missing castling".into()))?
            .parse::<Castling>()?;
        let en_passant = metadata_iter
            .next()
            .map(|s| State::map_empty_fen(s).map(|s| s.parse::<Square>()))
            .ok_or(ChessError::InvalidFEN("Missing/invalid en passant".into()))?;
        let half_move_clock = metadata_iter
            .next()
            .ok_or(ChessError::InvalidFEN("Missing half move clock".into()))?
            .parse::<u16>()
            .map_err(|_| ChessError::InvalidFEN("Invalid half move clock".into()))?;
        let full_move_number = metadata_iter
            .next()
            .ok_or(ChessError::InvalidFEN("Missing full move number".into()))?
            .parse::<u16>()
            .map_err(|_| ChessError::InvalidFEN("Invalid full move number".into()))?;

        // state.turn = turn;
        state.castling = castling;
        state.en_passant = en_passant.map(|s| s.unwrap());
        state.halfmove_clock = half_move_clock;
        state.ply = full_move_number;

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
            if i == rank {
                if piece.is_some() {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
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
