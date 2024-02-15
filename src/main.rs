pub mod bitboard;
pub mod square;
pub mod position;

use bitboard::Bitboard;
use square::Square;

fn main() {
    let a1 = Square::new(0);
    let b1 = Square::new(1);
    let a2 = Square::new(8);
    let e4 = Square::new(28);
    let h8 = Square::new(63);
    let bfile = Bitboard::get_file(b1.clone());
    let fourth_rank = Bitboard::get_rank(e4.clone());
    println!("a1:\n{:?}\n\n", a1.into_bitmap());
    println!("b1:\n{:?}\n\n", b1.into_bitmap());
    println!("a2:\n{:?}\n\n", a2.into_bitmap());
    println!("e4:\n{:?}\n\n", e4.into_bitmap());
    println!("h8:\n{:?}\n\n", h8.into_bitmap());
    println!("{:064b}\n\n", 1u64);
    let hfile = Bitboard::get_file(h8.clone());
    let eigth_rank = Bitboard::get_rank(h8.clone());
    println!("B file:\n{:?}\n\n", bfile);
    println!("4th rank:\n{:?}\n\n", fourth_rank);
    println!("H file:\n{:?}\n\n", hfile);
    println!("8th rank:\n{:?}\n\n", eigth_rank);

    println!("a1 antidiagonal:\n{:?}\n\n", Bitboard::get_anti_diagonal(a1.clone()));
    println!("b1 antidiagonal:\n{:?}\n\n", Bitboard::get_anti_diagonal(b1.clone()));
    println!("e4 antidiagonal:\n{:?}\n\n", Bitboard::get_anti_diagonal(e4.clone()));
    println!("h8 antidiagonal:\n{:?}\n\n", Bitboard::get_anti_diagonal(h8.clone()));

    println!("a1 diagonal:\n{:?}\n\n", Bitboard::get_diagonal(a1.clone()));
    println!("b1 diagonal:\n{:?}\n\n", Bitboard::get_diagonal(b1.clone()));
    println!("e4 diagonal:\n{:?}\n\n", Bitboard::get_diagonal(e4.clone()));
    println!("h8 diagonal:\n{:?}\n\n", Bitboard::get_diagonal(h8.clone()));
    println!("h1 diagonal:\n{:?}\n\n", Bitboard::get_diagonal(Square::new(7)));
    println!("a8 diagonal:\n{:?}\n\n", Bitboard::get_diagonal(Square::new(56)));
}
