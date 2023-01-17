#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Single grapheme tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    SemiColon,
    QuestionMark,

    // Double grapheme
    ColonColon,

    // One or more grapheme tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    More,
    MoreEqual,
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Arrow,
    Star,
    StarEqual,
    Slash,
    SlashEqual,

    // Literals
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Identifier(String),

    // Keywords
    Start,
    Function,
    Loop,
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
    Let,
}
