pub struct Bitboard(u64);

use std::fmt;

use crate::square::Square;

impl Bitboard {
    pub fn new(bitmap:u64) -> Self {
        Bitboard(bitmap)
    }

    pub fn occupied(&self, square:Square) -> bool {
        return self.0 & square.into_bitmap().0 != 0;
    }

    pub fn population(&self) -> u32 {
        return self.0.count_ones();
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.0.to_le_bytes();
        bytes.iter().map(
            );
        todo!();
    }
}
