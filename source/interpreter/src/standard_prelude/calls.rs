use crate::expr::{Expr, FloatExpr, ObjectExpr, StringExpr};

// Standard prelude calls (Separated by return types)
#[derive(Debug, Clone)]
pub enum NativeCallNone {
    Print(Box<Expr>, bool),
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
    ButtonSetText(Box<ObjectExpr>, Box<StringExpr>),
    ButtonSetBackgroundColour(Box<ObjectExpr>, FloatExpr, FloatExpr, FloatExpr),
    ButtonSetPadding(Box<ObjectExpr>, FloatExpr, FloatExpr),
    ButtonSetWidthFill(Box<ObjectExpr>),

    TextConstructor(Box<StringExpr>),
    TextSetValue(Box<ObjectExpr>, Box<StringExpr>),
    TextSetSize(Box<ObjectExpr>, Box<FloatExpr>),
    TextSetColour(Box<ObjectExpr>, FloatExpr, FloatExpr, FloatExpr),

    HyperlinkConstructor(Box<StringExpr>, Box<StringExpr>),

    InputConstructor(Box<StringExpr>),

    PageConstructor,
    PageSetTitle(Box<ObjectExpr>, Box<StringExpr>),
    PageSetBackgroundColour(Box<ObjectExpr>, FloatExpr, FloatExpr, FloatExpr),
    PageAddElement(Box<ObjectExpr>, Box<ObjectExpr>),
    PageRemoveElement(Box<ObjectExpr>, Box<ObjectExpr>),

    RowConstructor,
    RowAddElement(Box<ObjectExpr>, Box<ObjectExpr>),
    RowCenter(Box<ObjectExpr>),

    ColumnConstructor,
    ColumnAddElement(Box<ObjectExpr>, Box<ObjectExpr>),
    ColumnSetMaxWidth(Box<ObjectExpr>, FloatExpr),

    ImageConstructor(Box<StringExpr>),
    ImageSetMaxWidth(Box<ObjectExpr>, FloatExpr),
}
