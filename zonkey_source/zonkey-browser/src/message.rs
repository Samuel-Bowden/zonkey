#[derive(Debug, Clone)]
pub enum Message {
    AddressChanged(String),
    AddressConfirmed,
    SettingsPressed,
    HomePressed,
}
