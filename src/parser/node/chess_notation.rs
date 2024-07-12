#[derive(Debug)]
pub struct ChessNotation {
    notation: String,
}

impl ChessNotation {
    pub fn new(s: String) -> Self {
        ChessNotation { notation: s }
    }
}
