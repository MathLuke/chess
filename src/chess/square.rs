pub struct Square
{
    pub square:u8,
}

impl Square {
    fn get_file(&self) -> u8 {
        todo!()
    }

    fn get_rank(&self) -> u8 {
        todo!()
    }
}

impl TryFrom<&str> for Square {
    type Error = ();
    fn try_from(square: &str) -> Result<Self, Self::Error> {
        if square.len() != 2 {return Err(())}
        let mut chars = square.chars();
        let file = chars.next().ok_or(())? as u8; 
        let rank = chars.next().ok_or(())? as u8;
        if file < 97 || file > 104 || rank < 49 || rank > 56 {return Err(())}
        Ok(Square{
            square:8* (56 - rank) + file - 97
        }) 
    }
}

impl TryFrom<u8> for Square {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 63 {
            return Err(());
        }
        Ok(Square{
            square:value
        })
    }
}

impl From<Square> for u8 {
    fn from(value: Square) -> Self {
        value.square
    }
}

impl From<Square> for String {
    fn from(value: Square) -> Self {
        format!("{}{}", (104 - value.square % 8) as char, (49 + value.square / 8) as char)
    }
}

