#[derive(Debug, Clone)]
pub enum InterpreterEvent {
    AddHeading(String, i64),

    AddParagraph(String, i64),

    AddButton(String, i64),
    ChangeButtonText(String, i64),

    AddHyperlink(String, String, i64),

    AddInput(String, i64),
}

#[derive(Debug, Clone)]
pub enum BrowserEvent {
    ButtonPress(i64),
    InputConfirmed(String, i64),
}
