enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Default)]
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
        let mut bitboard = BitBoard{..Default::default()};
        let fen_iterator = fen.chars();
        let mut i = 63;
        for c in fen_iterator {
            match c {
                'p' => bitboard.black_pawns += 2^i,
                'r' => bitboard.black_rooks += 2^i,
                'n' => bitboard.black_knights += 2^i, 
                'b' => bitboard.black_bishops += 2^i, 
                'q' => bitboard.black_queens += 2^i, 
                'k' => bitboard.black_kings += 2^i, 
                
                'P' => bitboard.white_pawns += 2^i,
                'R' => bitboard.white_rooks += 2^i,
                'N' => bitboard.white_knights += 2^i, 
                'B' => bitboard.white_bishops += 2^i, 
                'Q' => bitboard.white_queens += 2^i, 
                'K' => bitboard.white_kings += 2^i, 
                '1'..='8' => i -= c as u64 - 49,
                _ => i+= 1,
            }
            i-= 1;
        }
        bitboard.white_pieces |= bitboard.white_pawns | bitboard.white_rooks | bitboard.white_knights | bitboard.white_bishops | bitboard.white_queens | bitboard.white_kings;
        bitboard.black_pieces |= bitboard.black_pawns | bitboard.black_rooks | bitboard.black_knights | bitboard.black_bishops | bitboard.black_queens | bitboard.black_kings;
        bitboard.all_pieces = bitboard.white_pieces | bitboard.black_pieces;
        bitboard
    }
}

struct Board {
    pieces:BitBoard,
    active_color:String,
    white_castling_rights:(bool, bool),
    black_castling_rights:(bool, bool),
    en_passant_target:Option<u8>,
    halfmove_clock:u8,
    fullmove:usize
}

impl Board {
    fn new(fen:&str) -> Self {
        let mut fen_iterator = fen.split(' ');
        let pieces = BitBoard::new(fen_iterator.next().expect("FEN should have a minimum of 1 tokens, found 0"));
        let active_color = fen_iterator.next().unwrap_or("w").to_string();
        let castling_rights_str = fen_iterator.next().unwrap_or("-");
        let white_castling_rights = (castling_rights_str.contains('K'), castling_rights_str.contains('Q'));
        let black_castling_rights = (castling_rights_str.contains('k'), castling_rights_str.contains('q'));
        let en_passant_target = fen_iterator.next().map(|x| square_to_int(x)).flatten();
        let halfmove_clock= fen_iterator.next().map(|x| x.parse::<u8>().unwrap_or(1)).unwrap_or(1);
        let fullmove = fen_iterator.next().map(|x| x.parse::<usize>().unwrap_or_default()).unwrap_or_default();
        Board{pieces, active_color, white_castling_rights, black_castling_rights, en_passant_target, halfmove_clock, fullmove}     
    }
}
impl Default for Board {
    fn default() -> Self {
        Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

fn square_to_int(square:&str) -> Option<u8> {
    if square.len() != 2 {return None}
    let mut chars = square.chars();
    let file = chars.next()? as u8; 
    let rank = chars.next()? as u8;
    if file < 97 || file > 104 || rank < 49 || rank > 56 {return None}
    Some(8* (56 - rank) + file - 97) 
}

fn main() {
    println!("{}", square_to_int("a8").unwrap());
    println!("{}", square_to_int("h1").unwrap());
}
