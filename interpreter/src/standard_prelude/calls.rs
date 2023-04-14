use crate::expr::{Expr, FloatExpr, IntegerExpr, ObjectExpr, StringExpr, BooleanExpr};

// Standard prelude calls (Separated by return types)
#[derive(Debug, Clone)]
pub enum NativeCallNone {
    Print(Box<Expr>, bool),
    Sleep(IntegerExpr),
    SetPage(ObjectExpr),
    CloseTab,
    WriteString(Box<StringExpr>, Box<StringExpr>),
    OpenLink(Box<StringExpr>),
}

#[derive(Debug, Clone)]
pub enum NativeCallInteger {
    IntegerArrayGet(Box<ObjectExpr>, Box<IntegerExpr>),
    IntegerArrayRemove(Box<ObjectExpr>, Box<IntegerExpr>),
    ArrayLength(Box<ObjectExpr>),
}

#[derive(Debug, Clone)]
pub enum NativeCallFloat {
    FloatArrayGet(Box<ObjectExpr>, Box<IntegerExpr>),
    FloatArrayRemove(Box<ObjectExpr>, Box<IntegerExpr>),
}

#[derive(Debug, Clone)]
pub enum NativeCallString {
    Prompt(Box<StringExpr>),
    FromInteger(IntegerExpr),
    FromFloat(FloatExpr),
    GetInputText(ObjectExpr),
    ReadString(Box<StringExpr>),
    StringArrayGet(Box<ObjectExpr>, IntegerExpr),
    StringArrayRemove(Box<ObjectExpr>, IntegerExpr),
}

#[derive(Debug, Clone)]
pub enum NativeCallBoolean {
    WaitForEvent,
    ButtonClicked(ObjectExpr),
    InputConfirmed(ObjectExpr),
    BooleanArrayGet(Box<ObjectExpr>, IntegerExpr),
    BooleanArrayRemove(Box<ObjectExpr>, IntegerExpr),
}

#[derive(Debug, Clone)]
pub enum NativeCallObject {
    ButtonConstructor(Box<StringExpr>),
    ButtonSetText(Box<ObjectExpr>, Box<StringExpr>),
    ButtonSetBackgroundColour(Box<ObjectExpr>, Box<StringExpr>),
    ButtonSetTextColour(Box<ObjectExpr>, Box<StringExpr>),
    ButtonSetPadding(Box<ObjectExpr>, FloatExpr, FloatExpr),
    ButtonSetWidthFill(Box<ObjectExpr>),

    TextConstructor(Box<StringExpr>),
    TextSetValue(Box<ObjectExpr>, Box<StringExpr>),
    TextSetSize(Box<ObjectExpr>, Box<FloatExpr>),
    TextSetColour(Box<ObjectExpr>, Box<StringExpr>),

    HyperlinkConstructor(Box<StringExpr>, Box<StringExpr>),

    InputConstructor(Box<StringExpr>),
    InputSetText(Box<ObjectExpr>, Box<StringExpr>),

    PageConstructor,
    PageSetTitle(Box<ObjectExpr>, Box<StringExpr>),
    PageSetBackgroundColour(Box<ObjectExpr>, Box<StringExpr>),
    PageSetTextColour(Box<ObjectExpr>, Box<StringExpr>),
    PageCenter(Box<ObjectExpr>),
    PageAddElement(Box<ObjectExpr>, Box<ObjectExpr>),
    PageRemoveElement(Box<ObjectExpr>, Box<ObjectExpr>),
    PageSetMaxWidth(Box<ObjectExpr>, FloatExpr),

    RowConstructor,
    RowAddElement(Box<ObjectExpr>, Box<ObjectExpr>),
    RowRemoveElement(Box<ObjectExpr>, Box<ObjectExpr>),
    RowCenter(Box<ObjectExpr>),

    ColumnConstructor,
    ColumnAddElement(Box<ObjectExpr>, Box<ObjectExpr>),
    ColumnRemoveElement(Box<ObjectExpr>, Box<ObjectExpr>),
    ColumnSetMaxWidth(Box<ObjectExpr>, FloatExpr),

    ImageConstructor(Box<StringExpr>),
    ImageSetMaxWidth(Box<ObjectExpr>, FloatExpr),

    IntegerArrayConstructor(Vec<Expr>),
    IntegerArrayPush(Box<ObjectExpr>, IntegerExpr),

    FloatArrayConstructor(Vec<Expr>),
    FloatArrayPush(Box<ObjectExpr>, FloatExpr),

    StringArrayConstructor(Vec<Expr>),
    StringArrayPush(Box<ObjectExpr>, Box<StringExpr>),

    BooleanArrayConstructor(Vec<Expr>),
    BooleanArrayPush(Box<ObjectExpr>, Box<BooleanExpr>),

    ObjectArrayConstructor(Vec<Expr>),
    ObjectArrayPush(Box<ObjectExpr>, Box<ObjectExpr>),
    ObjectArrayGet(Box<ObjectExpr>, IntegerExpr),
    ObjectArrayRemove(Box<ObjectExpr>, IntegerExpr),
}
