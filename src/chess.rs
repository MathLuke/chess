pub mod board;
pub mod bitboard;
pub mod square;
pub mod fen;

pub use square::Square;
pub use board::Board;
pub use bitboard::BitBoard;
pub use fen::FENString;

pub enum Move {
    Normal{from:Square, to:Square},
    Castle (CastlingDirection),
    Promotion {
        from:Square,
        to:Square,
        into_piece:Piece,
    },
}

pub enum Player {
    White,
    Black
}

pub struct CastlingRights {
    kingside:bool,
    queenside:bool,
}

impl CastlingRights {
    fn get_castling_rights(s:&str, player:Player) -> Self {
        match player {
            Player::White => {
                CastlingRights{
                    kingside:s.contains("K"),
                    queenside:s.contains("Q")
                }
            },
            Player::Black => {
                CastlingRights {
                    kingside:s.contains("k"),
                    queenside:s.contains("q")
                }
            }
        }
    }
}

pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

pub enum CastlingDirection {
    Queenside,
    Kingside,
}

