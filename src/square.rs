use crate::bitboard::Bitboard;

pub struct Square(u8);

impl Square {
    pub fn new(index:u8) -> Self {
        Square(index.clamp(0,63))
    }

    pub fn into_bitmap(&self) -> Bitboard {
        return Bitboard::new((1 << self.0) as u64);
    }
}
