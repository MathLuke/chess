use std::fmt;

#[derive(Default)]
pub struct BitBoard {
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
    pub fn new (fen:&str) -> Self {
        BitBoard::try_from(fen).expect("Invalid FEN provided")
    } 

    pub fn get_partial_fen (&self) -> String {
        let mut output = String::new();
        let mut empty_counter:u8 = 0;
        for i in (0..=63).rev() {
            if 1<<i & self.white_pieces != 0 {
                if empty_counter != 0 {
                    output.push((empty_counter + 48) as char);
                    empty_counter = 0;
                }
                if 1<<i & self.white_pawns != 0 {output.push('P');}
                else if 1<<i & self.white_rooks != 0 {output.push('R');}
                else if 1<<i & self.white_knights != 0 {output.push('N');}
                else if 1<<i & self.white_bishops != 0 {output.push('B');}
                else if 1<<i & self.white_queens != 0 {output.push('Q');}
                else if 1<<i & self.white_kings != 0 {output.push('K');}
            } else if 1<<i & self.black_pieces != 0 {
                if empty_counter != 0 {
                    output.push((empty_counter + 48) as char);
                    empty_counter = 0;
                }
                if 1<<i & self.black_pawns != 0 {output.push('p');}
                else if 1<<i & self.black_rooks != 0 {output.push('r');}
                else if 1<<i & self.black_knights != 0 {output.push('n');}
                else if 1<<i & self.black_bishops != 0 {output.push('b');}
                else if 1<<i & self.black_queens != 0 {output.push('q');}
                else if 1<<i & self.black_kings != 0 {output.push('k');}
            } else {
                empty_counter += 1;
            }
            if i > 0 && i % 8 == 0 {
                if empty_counter != 0 {
                    output.push((empty_counter + 48) as char);
                    empty_counter = 0;
                }
                output.push('/');
            }
        }
        output
    }
}

impl TryFrom<&str> for BitBoard {
    type Error = &'static str;
    fn try_from(fen: &str) -> Result<Self, Self::Error> {
        let mut bitboard = BitBoard{..Default::default()};
        let fen_iterator = fen.chars();
        let mut i = 63;
        for c in fen_iterator {
            match c {
                'p' => bitboard.black_pawns += 1<<i,
                'r' => bitboard.black_rooks += 1<<i,
                'n' => bitboard.black_knights += 1<<i, 
                'b' => bitboard.black_bishops += 1<<i, 
                'q' => bitboard.black_queens += 1<<i, 
                'k' => bitboard.black_kings += 1<<i, 

                'P' => bitboard.white_pawns += 1<<i,
                'R' => bitboard.white_rooks += 1<<i,
                'N' => bitboard.white_knights += 1<<i, 
                'B' => bitboard.white_bishops += 1<<i, 
                'Q' => bitboard.white_queens += 1<<i, 
                'K' => bitboard.white_kings += 1<<i, 
                '1'..='8' => i -= c as u64 - 49,
                _ => i+= 1,
            }
            if i == 0 {break;}
            i-= 1;
        }
        bitboard.white_pieces |= bitboard.white_pawns | bitboard.white_rooks | bitboard.white_knights | bitboard.white_bishops | bitboard.white_queens | bitboard.white_kings;
        bitboard.black_pieces |= bitboard.black_pawns | bitboard.black_rooks | bitboard.black_knights | bitboard.black_bishops | bitboard.black_queens | bitboard.black_kings;
        bitboard.all_pieces = bitboard.white_pieces | bitboard.black_pieces;
        if bitboard.all_pieces == 0 {return Err("Empty BitBoard was constructed based on input FEN")}
        if bitboard.white_kings.count_ones() != 1 || bitboard.black_kings.count_ones() != 1 {return Err("FEN must contain 1 king of each color")}
        if bitboard.white_pawns & bitboard.white_rooks & bitboard.white_knights & bitboard.white_bishops & bitboard.white_queens & bitboard.white_kings & bitboard.black_pawns & bitboard.black_rooks & bitboard.black_knights & bitboard.black_bishops & bitboard.black_queens & bitboard.black_kings != 0 {
            return Err("Piece collision detected. If you see this, let a developer know.");
        }
        Ok(bitboard)
    }
}

impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output:String = String::new();
        output.push_str(&format!("all_pieces:    {:0>64b}\n\n", self.all_pieces));
        output.push_str(&format!("white_pieces:  {:0>64b}\n", self.white_pieces));
        output.push_str(&format!("white_pawns:   {:0>64b}\n", self.white_pawns));
        output.push_str(&format!("white_rooks:   {:0>64b}\n", self.white_rooks));
        output.push_str(&format!("white_knights: {:0>64b}\n", self.white_knights));
        output.push_str(&format!("white_bishops: {:0>64b}\n", self.white_bishops));
        output.push_str(&format!("white_queens:  {:0>64b}\n", self.white_queens));
        output.push_str(&format!("white_kings:   {:0>64b}\n\n", self.white_kings));

        output.push_str(&format!("black_pieces:  {:0>64b}\n", self.black_pieces));
        output.push_str(&format!("black_pawns:   {:0>64b}\n", self.black_pawns));
        output.push_str(&format!("black_rooks:   {:0>64b}\n", self.black_rooks));
        output.push_str(&format!("black_knights: {:0>64b}\n", self.black_knights));
        output.push_str(&format!("black_bishops: {:0>64b}\n", self.black_bishops));
        output.push_str(&format!("black_queens:  {:0>64b}\n", self.black_queens));
        output.push_str(&format!("black_kings:   {:0>64b}\n", self.black_kings));
        write!(f, "{output}")
    } 
}

