pub enum Move {
    Normal{from:u8, to:u8},
    Castle (),
    Promotion (crate::Piece),
}
