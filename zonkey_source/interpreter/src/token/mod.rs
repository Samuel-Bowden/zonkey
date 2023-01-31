use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single grapheme tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    SemiColon,
    Colon,
    QuestionMark,

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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftParen => f.write_str("("),
            Self::RightParen => f.write_str(")"),
            Self::LeftBrace => f.write_str("{"),
            Self::RightBrace => f.write_str("}"),
            Self::Comma => f.write_str(","),
            Self::Dot => f.write_str("."),
            Self::SemiColon => f.write_str(";"),
            Self::QuestionMark => f.write_str("?"),
            Self::Colon => f.write_str(":"),
            Self::Bang => f.write_str("!"),
            Self::BangEqual => f.write_str("!="),
            Self::Equal => f.write_str("="),
            Self::EqualEqual => f.write_str("=="),
            Self::Less => f.write_str("<"),
            Self::LessEqual => f.write_str("<="),
            Self::More => f.write_str(">"),
            Self::MoreEqual => f.write_str(">="),
            Self::Plus => f.write_str("+"),
            Self::PlusEqual => f.write_str("+="),
            Self::Minus => f.write_str("-"),
            Self::MinusEqual => f.write_str("-="),
            Self::Arrow => f.write_str("->"),
            Self::Star => f.write_str("*"),
            Self::StarEqual => f.write_str("*="),
            Self::Slash => f.write_str("/"),
            Self::SlashEqual => f.write_str("/="),
            Self::String(val) => write!(f, "{val}"),
            Self::Integer(val) => write!(f, "{val}"),
            Self::Float(val) => write!(f, "{val}"),
            Self::Boolean(val) => write!(f, "{val}"),
            Self::Identifier(val) => write!(f, "{val}"),
            Self::Start => f.write_str("start"),
            Self::Function => f.write_str("function"),
            Self::Loop => f.write_str("loop"),
            Self::If => f.write_str("if"),
            Self::Else => f.write_str("else"),
            Self::For => f.write_str("for"),
            Self::While => f.write_str("while"),
            Self::Break => f.write_str("break"),
            Self::Continue => f.write_str("continue"),
            Self::Return => f.write_str("return"),
            Self::IntegerType => f.write_str("Integer"),
            Self::FloatType => f.write_str("Float"),
            Self::StringType => f.write_str("String"),
            Self::BooleanType => f.write_str("Boolean"),
            Self::Let => f.write_str("let"),
        }
    }
}
