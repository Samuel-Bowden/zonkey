use crate::expr::{Expr, ObjectExpr, StringExpr};

#[derive(Debug)]
pub enum NativeFunctionNone {
    Print(Box<Expr>, bool),
    AddButton(ObjectExpr),
    AddHeading(ObjectExpr),
    AddParagraph(ObjectExpr),
    AddHyperlink(ObjectExpr),
    AddInput(ObjectExpr),
    ButtonText(ObjectExpr, Box<StringExpr>),
}

#[derive(Debug)]
pub enum NativeFunctionString {
    Prompt(Box<StringExpr>),
}

#[derive(Debug)]
pub enum NativeFunctionBoolean {
    WaitForEvent,
    ButtonClicked(ObjectExpr),
}

#[derive(Debug)]
pub enum NativeFunctionObject {
    ButtonConstructor(Box<StringExpr>),
    HeadingConstructor(Box<StringExpr>),
    ParagraphConstructor(Box<StringExpr>),
    HyperlinkConstructor(Box<StringExpr>, Box<StringExpr>),
    InputConstructor(Box<StringExpr>),
}
