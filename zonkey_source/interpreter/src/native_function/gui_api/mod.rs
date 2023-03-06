use crate::expr::StringExpr;

#[derive(Debug)]
pub enum GuiFunctionNone {
    AddHeading(Box<StringExpr>),
    AddParagraph(Box<StringExpr>),
    AddHyperlink(Box<StringExpr>),
    AddButton(Box<StringExpr>),
    AddImage(Box<StringExpr>),
}
