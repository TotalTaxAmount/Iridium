use std::fmt;

#[derive(Debug)]
pub struct Fen {
    pub pos: Vec<Vec<String>>,
    pub active: String,
    pub castling: String,
    pub en_passant: String,
    pub halfmoves: u32,
    pub fullmoves: u32
}

impl fmt::Display for Fen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
