use super::{bitboard::BitBoard, movement};
use std::fmt;

pub struct Board {
    fen:String,
    pieces:BitBoard,
    active_color:Player,
    white_castling_rights:CastlingRights,
    black_castling_rights:CastlingRights,
    en_passant_target:Option<u8>,
    halfmove_clock:u8,
    fullmove:usize
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

impl Board {
    pub fn new(fen:&str) -> Self {
        Board::try_from(fen).expect("FEN is formatted improperly")
    } 

    pub fn make_move(m:movement::Move) {

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
        let active_color = match fen_iterator.next() {
            Some("B") | Some("b") => Player::Black,
            _ => Player::White
        };
        let castling_rights_str = fen_iterator.next().unwrap_or("-");
        let white_castling_rights = CastlingRights::get_castling_rights(castling_rights_str, Player::White);
        let black_castling_rights = CastlingRights::get_castling_rights(castling_rights_str, Player::Black);
        let en_passant_target = fen_iterator.next().map(|x| square_to_int(x).ok()).flatten();
        let halfmove_clock= fen_iterator.next().map(|x| x.parse::<u8>().unwrap_or(1)).unwrap_or(1);
        let fullmove = fen_iterator.next().map(|x| x.parse::<usize>().unwrap_or_default()).unwrap_or_default();
        // let fen = format!("{} {} {} {} {} {}", pieces.get_partial_fen(), active_color, castling_rights_str, en_passant_target, halfmove_clock, fullmove);
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

fn int_to_square(square:u8) -> Result<String, ()> {
    if square > 63 {return Err(())}
    Ok(format!("{}{}", (104 -square % 8) as char, (49 + square / 8) as char))
}

