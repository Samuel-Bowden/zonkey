#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single grapheme tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma,
    Dot,
    SemiColon,
    QuestionMark,

    // One or more grapheme tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Less, LessEqual,
    More, MoreEqual,
    Plus, PlusEqual,
    Minus, MinusEqual, Arrow,
    Star, StarEqual,
    Slash, SlashEqual,

    // Literals
    String,
    Integer,
    Float,
    Identifier,

    // Keywords
    Start,
    Function,
    Loop,
    Infinite,
    If,
    Else,
    For,
    While,
    Break,
    Continue,
    Return,
    IntegerType,
    FloatType,
    StringType,
    BooleanType,
}
