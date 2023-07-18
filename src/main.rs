use std::fmt;

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
            if i == 0 {break;}
            i-= 1;
        }
        bitboard.white_pieces |= bitboard.white_pawns | bitboard.white_rooks | bitboard.white_knights | bitboard.white_bishops | bitboard.white_queens | bitboard.white_kings;
        bitboard.black_pieces |= bitboard.black_pawns | bitboard.black_rooks | bitboard.black_knights | bitboard.black_bishops | bitboard.black_queens | bitboard.black_kings;
        bitboard.all_pieces = bitboard.white_pieces | bitboard.black_pieces;
        bitboard
    }
}

struct Board {
    fen:String,
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
        Board::try_from(fen).expect("FEN is formatted improperly")
    } 
}

impl Default for Board {
    fn default() -> Self {
        Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl TryFrom<&str> for Board {
    type Error = &'static str;
    fn try_from(fen: &str) -> Result<Self, Self::Error> {
        let mut fen_iterator = fen.split(' ');
        let pieces = BitBoard::new(fen_iterator.next().ok_or("FEN should have a minimum of 1 tokens, found 0")?);
        let active_color = fen_iterator.next().unwrap_or("w").to_string();
        let castling_rights_str = fen_iterator.next().unwrap_or("-");
        let white_castling_rights = (castling_rights_str.contains('K'), castling_rights_str.contains('Q'));
        let black_castling_rights = (castling_rights_str.contains('k'), castling_rights_str.contains('q'));
        let en_passant_target = fen_iterator.next().map(|x| square_to_int(x).ok()).flatten();
        let halfmove_clock= fen_iterator.next().map(|x| x.parse::<u8>().unwrap_or(1)).unwrap_or(1);
        let fullmove = fen_iterator.next().map(|x| x.parse::<usize>().unwrap_or_default()).unwrap_or_default();
        Ok(Board{fen:fen.to_string(), pieces, active_color, white_castling_rights, black_castling_rights, en_passant_target, halfmove_clock, fullmove}) //TODO: Better FEN generation
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = "   _ _ _ _ _ _ _ _\n8 ".to_string();
        let mut rank_num = 7;
        for c in self.fen.split(' ').next().ok_or(fmt::Error)?.chars() {
            match c {
                'p' | 'r' | 'n' | 'b' | 'q' | 'k' | 'P' | 'R' | 'N' | 'B' | 'Q' | 'K' => {
                    output.push('|');
                    output.push(c);
                },
                '1'..='8' => {
                    output.push_str(&"|_".repeat(c as usize - 48))
                },
                '/' => {
                    output.push_str(&format!("|\n{} ", rank_num));
                    rank_num -= 1;
                }
                _ => ()
            }
        }
        output.push_str("|\n   a b c d e f g h");
        write!(f, "{output}\nfen:{}", self.fen)
    }
}

fn square_to_int(square:&str) -> Result<u8, ()> { 
    if square.len() != 2 {return Err(())}
    let mut chars = square.chars();
    let file = chars.next().ok_or(())? as u8; 
    let rank = chars.next().ok_or(())? as u8;
    if file < 97 || file > 104 || rank < 49 || rank > 56 {return Err(())}
    Ok(8* (56 - rank) + file - 97) 
}

fn main() {
    let board = Board::default();
    println!("{}", board);
}
