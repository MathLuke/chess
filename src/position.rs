use crate::bitboard::Bitboard;

pub struct Position {
    all_pieces:Bitboard,

    white_pieces:Bitboard,
    black_pieces:Bitboard,

    pawns:Bitboard,
    rooks:Bitboard,
    knights:Bitboard,
    bishops:Bitboard,
    queens:Bitboard,
    kings:Bitboard,
}

impl Position {
}
