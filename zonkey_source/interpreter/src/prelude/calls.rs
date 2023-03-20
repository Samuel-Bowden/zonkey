use crate::expr::{Expr, FloatExpr, ObjectExpr, StringExpr};

// Prelude Calls (Separated by return types)
#[derive(Debug, Clone)]
pub enum NativeCallNone {
    Print(Box<Expr>, bool),
    AddButton(ObjectExpr, ObjectExpr),
    AddText(ObjectExpr, ObjectExpr),
    AddHyperlink(ObjectExpr, ObjectExpr),
    AddInput(ObjectExpr, ObjectExpr),
}

#[derive(Debug, Clone)]
pub enum NativeCallString {
    Prompt(Box<StringExpr>),
    GetInputText(ObjectExpr),
}

#[derive(Debug, Clone)]
pub enum NativeCallBoolean {
    WaitForEvent,
    ButtonClicked(ObjectExpr),
    InputConfirmed(ObjectExpr),
}

#[derive(Debug, Clone)]
pub enum NativeCallObject {
    ButtonConstructor(Box<StringExpr>),
    SetButtonText(Box<ObjectExpr>, Box<StringExpr>),
    SetButtonBackgroundColour(Box<ObjectExpr>, FloatExpr, FloatExpr, FloatExpr),

    TextConstructor(Box<StringExpr>),
    SetTextValue(Box<ObjectExpr>, Box<StringExpr>),
    SetTextSize(Box<ObjectExpr>, Box<FloatExpr>),
    SetTextColour(Box<ObjectExpr>, FloatExpr, FloatExpr, FloatExpr),

    HyperlinkConstructor(Box<StringExpr>, Box<StringExpr>),

    InputConstructor(Box<StringExpr>),

    PageConstructor,
}
