#[derive(Debug, Clone)]
pub enum Message {
    AddressConfirmed,
    AddressChanged(String),
    HomePressed,
    SettingsPressed,
    ReloadPressed,
    BackPressed,
    Tab(tab::MessagePointer),
    TabPressed(usize),
    TabClosePressed(usize),
    NewTab,
}
