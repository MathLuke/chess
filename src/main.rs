pub mod bitboard;
pub mod square;
pub mod position;

use bitboard::Bitboard;
use square::Square;

fn main() {
    let a1 = Square::new(0);
    let b1 = Square::new(1);
    let a2 = Square::new(8);
    println!("a1:\n{:?}\n\n", a1.into_bitmap());
    println!("b1:\n{:?}\n\n", b1.into_bitmap());
    println!("a2:\n{:?}\n\n", a2.into_bitmap());
    println!("{:064b}\n\n", 1u64);

}
