use std::fmt;
use super::{
    Move,
    BitBoard,
    CastlingRights,
    Square,
    Player,
};

pub struct Board {
    fen:String,
    pieces:BitBoard,
    active_color:Player,
    white_castling_rights:CastlingRights,
    black_castling_rights:CastlingRights,
    en_passant_target:Option<Square>,
    halfmove_clock:u8,
    fullmove:usize
}

pub enum BoardParseError {
    LengthError,
    InvalidFENError,
}

impl Board {
    pub fn new(fen:&str) -> Self {
        Board::try_from(fen).expect("FEN is formatted improperly")
    } 

    pub fn make_move(&mut self, m:Move) {
        match m {
            Move::Normal { from, to } => {
                todo!()
            }
            _ => (),
        }
        if let Player::Black = self.active_color {
            self.fullmove += 1;
        }
        self.halfmove_clock += 1;
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();
        legal_moves
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
        let en_passant_target = fen_iterator.next().map(|x| Square::try_from(x).ok()).flatten();
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
