use crate::bitboard::Bitboard;

#[derive(Clone, Copy)]
pub struct Square(pub(crate) u8);

impl Square {
    pub fn new(index:u8) -> Self {
        Square(index.clamp(0,63))
    }

    pub fn into_bitmap(&self) -> Bitboard {
        return Bitboard::new((1u64 << self.0) as u64);
    }
}
