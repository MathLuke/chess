pub struct FENString {
    fen_data:[String; 8],
}

impl FENString {
    pub fn new() -> Self {
        todo!();
    }

    pub fn fen(&self) -> String {
        let mut output:String = String::new();
        for s in &self.fen_data {
            output.push_str(&s);
            output.push(' ');
        }
        output.pop();
        return output;
    }

    pub fn pieces(&self) -> Bitboard {
    }
}
