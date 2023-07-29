pub mod chess {
    pub mod board;
    pub mod bitboard;
    pub mod movement;
}

use chess::board::Board;

enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

fn main() {
    let board = Board::default();
    println!("{}", board);
}

#[cfg(test)]
mod tests {

    fn get_all_squares() -> Vec<String> {
        let files: Vec<char> = vec!['h','g','f','e','d','c','b','a'];
        let ranks:Vec<char> = ('1'..='8').collect();
        let mut all_squares = Vec::new();
        for rank in ranks {
            for file in &files {
                all_squares.push(format!("{}{}", file, rank))
            }
        } 
        all_squares
    }
    #[test]
    fn square_int_conversions() {
    let all_squares:Vec<String> = get_all_squares();
        for i in 0..63 {
            // assert_eq!(int_to_square(i).map(|x| , all_squares.get(i as usize).ok_or(()));
        }
    }
}
