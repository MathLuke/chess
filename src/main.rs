enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

struct BitBoard {
    all_pieces:u64,

    white_pieces:u64,
    white_pawns:u64,
    white_rooks:u64,
    white_knights:u64,
    white_bishops:u64,
    white_queens:u64,
    white_kings:u64,

    black_pieces:u64,
    black_pawns:u64,
    black_rooks:u64,
    black_knights:u64,
    black_bishops:u64, 
    black_queens:u64,
    black_kings:u64,
}

impl BitBoard {
    fn new (fen:&str) -> Self {
       todo!() 
    }
}

struct Board {
    fen:String,
    pieces:BitBoard,
    active_color:char,
    white_castling_rights:(bool, bool),
    black_castling_rights:(bool, bool),
    en_passant_target:Option<u8>,
    halfmove_clock:u8,
    fullmove:usize
}

impl Board {
    fn new(fen:&str) -> Self {
        let mut fen_iterator = fen.split(' ').peekable();
        if fen_iterator.peek().is_none() {
            return Board::default();
        }
        Board { 
            fen: fen_iterator.next(),
            pieces: Bitboard::new(fen), 
            active_color: fen_iterator.next().unwrap_or('w'), 
            white_castling_rights: (fen_iterator.next().unwrap_or_default(), fen_iterator.next().unwrap_or_default()), 
            black_castling_rights: (fen_iterator.next().unwrap_or_default(), fen_iterator.next().unwrap_or_default()), 
            en_passant_target: fen_iterator.next(), 
            halfmove_clock: fen_iterator.next().unwrap_or(0), 
            fullmove: fen_iterator.next().unwrap_or(0)
        }        
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}
fn main() {
    println!("Hello, world!");
}
