#[derive(Debug, Clone)]
pub enum Message {
    ZoomIn,
    ZoomOut,
    AddressConfirmed,
    AddressChanged(String),
    HomePressed,
    ReloadPressed,
    BackPressed,
    Tab(crate::tab::MessagePointer),
    TabPressed(usize),
    TabClosePressed(usize),
    NewTab,
}
