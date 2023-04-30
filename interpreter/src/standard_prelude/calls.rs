use crate::{
    expr::{BooleanExpr, Expr, FloatExpr, IntegerExpr, ObjectExpr, StringExpr},
    token::Token,
};

// Standard prelude calls (Separated by return types)
#[derive(Debug, Clone)]
pub enum NativeCallNone {
    Print(Box<Expr>, bool),
    Sleep(IntegerExpr),
    SetPage(ObjectExpr),
    CloseTab,
    OpenLink(Box<StringExpr>, ObjectExpr),
    InstallApplication(ObjectExpr, BooleanExpr),
    RemoveApplication(StringExpr),
}

#[derive(Debug, Clone)]
pub enum NativeCallInteger {
    IntegerArrayGet(Box<ObjectExpr>, Box<IntegerExpr>, Token),
    IntegerArrayRemove(Box<ObjectExpr>, Box<IntegerExpr>, Token),
    ArrayLength(Box<ObjectExpr>),
    FromString(Box<StringExpr>, Token),
    FromFloat(Box<FloatExpr>),
    Power(Box<IntegerExpr>, Box<IntegerExpr>),
}

#[derive(Debug, Clone)]
pub enum NativeCallFloat {
    FloatArrayGet(Box<ObjectExpr>, Box<IntegerExpr>, Token),
    FloatArrayRemove(Box<ObjectExpr>, Box<IntegerExpr>, Token),
    FromString(Box<StringExpr>, Token),
    FromInteger(Box<IntegerExpr>),
    PowerF(Box<FloatExpr>, Box<FloatExpr>),
}

#[derive(Debug, Clone)]
pub enum NativeCallString {
    Prompt(Box<StringExpr>),
    FromInteger(IntegerExpr),
    FromFloat(FloatExpr),
    GetInputText(ObjectExpr),
    GetButtonText(ObjectExpr),
    ReadString(Box<StringExpr>),
    WriteString(Box<StringExpr>, Box<StringExpr>),
    StringArrayGet(Box<ObjectExpr>, IntegerExpr, Token),
    StringArrayRemove(Box<ObjectExpr>, IntegerExpr, Token),
}

#[derive(Debug, Clone)]
pub enum NativeCallBoolean {
    WaitForEvent,
    ButtonClicked(ObjectExpr),
    InputConfirmed(ObjectExpr),
    BooleanArrayGet(Box<ObjectExpr>, IntegerExpr, Token),
    BooleanArrayRemove(Box<ObjectExpr>, IntegerExpr, Token),
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
    HyperlinkAddArg(Box<ObjectExpr>, Box<StringExpr>),

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
    ObjectArrayGet(Box<ObjectExpr>, IntegerExpr, Token),
    ObjectArrayRemove(Box<ObjectExpr>, IntegerExpr, Token),

    Args,

    InstalledApplications,
}
