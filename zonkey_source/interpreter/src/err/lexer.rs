#[derive(Debug)]
pub enum LexerErr {
    UnexpectedGrapheme(usize),
    UnterminatedString(usize),
    FloatMoreThanOneDecimalPoint(usize),
}
