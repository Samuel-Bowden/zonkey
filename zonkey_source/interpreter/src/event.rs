#[derive(Debug, Clone)]
pub enum InterpreterEvent {
    AddText(Text),
    AddButton(Button),
    AddHyperlink(String, String, i64),
    AddInput(String, i64),
    SetButtonText(String, i64),
    SetTextValue(String, i64),
    SetTextSize(f32, i64),
    SetTextColour(f32, f32, f32, i64),
    SetButtonBackgroundColour(f32, f32, f32, i64),
}

#[derive(Debug, Clone)]
pub enum BrowserEvent {
    ButtonPress(i64),
    InputConfirmed(String, i64),
}

#[derive(Debug, Clone)]
pub struct Text {
    pub id: i64,
    pub value: String,
    pub size: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub id: i64,
    pub text: String,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
