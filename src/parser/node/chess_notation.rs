#[derive(Debug)]
pub struct ChessNotation<'a> {
    notation: &'a str,
}

impl<'a> ChessNotation<'a> {
    pub fn new(s: &'a str) -> Self {
        ChessNotation { notation: s }
    }
}
