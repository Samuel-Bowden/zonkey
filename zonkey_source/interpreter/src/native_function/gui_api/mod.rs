use crate::expr::StringExpr;

#[derive(Debug)]
pub enum GuiFunctionNone {
    AddHeading(Box<StringExpr>),
    AddParagraph(Box<StringExpr>),
}
