const A_FILE:u64 = 0x0101010101010101;
const FIRST_RANK:u64 = 0x00000000000000ff;
const DIAGONALS:[Bitboard;15] = [
    Bitboard(0x0100000000000000),
    Bitboard(0x0201000000000000),
    Bitboard(0x0402010000000000),
    Bitboard(0x0804020100000000),
    Bitboard(0x1008040201000000),
    Bitboard(0x2010080402010000),
    Bitboard(0x4020100804020100),
    Bitboard(0x8040201008040201),
    Bitboard(0x0080402010080402),
    Bitboard(0x0000804020100804),
    Bitboard(0x0000008040201008),
    Bitboard(0x0000000080402010),
    Bitboard(0x0000000000804020),
    Bitboard(0x0000000000008040),
    Bitboard(0x0000000000000080),
];
const ANTI_DIAGONALS:[Bitboard;15] = [
    Bitboard(0x0000000000000001),
    Bitboard(0x0000000000000102),
    Bitboard(0x0000000000010204),
    Bitboard(0x0000000001020408),
    Bitboard(0x0000000102040810),
    Bitboard(0x0000010204081020),
    Bitboard(0x0001020408102040),
    Bitboard(0x0102040810204080),
    Bitboard(0x0204081020408000),
    Bitboard(0x0408102040800000),
    Bitboard(0x0810204080000000),
    Bitboard(0x1020408000000000),
    Bitboard(0x2040800000000000),
    Bitboard(0x4080000000000000),
    Bitboard(0x8000000000000000),
];

#[derive(Clone, Copy)]
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

    pub const fn get_rank(square:Square) -> Self {
        let Square(index) = square;
        return Bitboard(FIRST_RANK << (index / 8) * 8);
    }

    pub const fn get_file(square:Square) -> Self {
        let Square(index) = square;
        return Bitboard(A_FILE << (index % 8));
    }

    pub const fn get_diagonal(square:Square) -> Self {
        return DIAGONALS[(square.0 % 8 + (7-square.0 / 8)) as usize];
    }

    pub const fn get_anti_diagonal(square:Square) -> Self {
        return ANTI_DIAGONALS[(square.0 % 8 + square.0 / 8) as usize];
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0.to_le_bytes().iter().rev() {
            write!(f, "{:08b}\n", byte.reverse_bits())?;
        }
        Ok(())
    }
}
