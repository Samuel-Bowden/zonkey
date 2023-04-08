use crate::{message::Message, ZonkeyBrowser};
use iced::{
    widget::{
        container::{Appearance, StyleSheet},
        Button, TextInput,
    },
    widget::{svg::Handle, Container, Row, Svg},
    Alignment, Background, Color, Length, Theme,
};

// Icons from remixicon.com
pub const SETTINGS: &'static str = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' width='24' height='24'><path fill='none' d='M0 0h24v24H0z'/><path d='M8.686 4l2.607-2.607a1 1 0 0 1 1.414 0L15.314 4H19a1 1 0 0 1 1 1v3.686l2.607 2.607a1 1 0 0 1 0 1.414L20 15.314V19a1 1 0 0 1-1 1h-3.686l-2.607 2.607a1 1 0 0 1-1.414 0L8.686 20H5a1 1 0 0 1-1-1v-3.686l-2.607-2.607a1 1 0 0 1 0-1.414L4 8.686V5a1 1 0 0 1 1-1h3.686zM6 6v3.515L3.515 12 6 14.485V18h3.515L12 20.485 14.485 18H18v-3.515L20.485 12 18 9.515V6h-3.515L12 3.515 9.515 6H6zm6 10a4 4 0 1 1 0-8 4 4 0 0 1 0 8zm0-2a2 2 0 1 0 0-4 2 2 0 0 0 0 4z' fill='rgba(255,255,255,1)'/></svg>";
pub const HOME: &'static str = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' width='24' height='24'><path fill='none' d='M0 0h24v24H0z'/><path d='M21 20a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V9.49a1 1 0 0 1 .386-.79l8-6.222a1 1 0 0 1 1.228 0l8 6.222a1 1 0 0 1 .386.79V20zm-2-1V9.978l-7-5.444-7 5.444V19h14z' fill='rgba(255,255,255,1)'/></svg>";
pub const RELOAD: &'static str = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' width='24' height='24'><path fill='none' d='M0 0h24v24H0z'/><path d='M18.537 19.567A9.961 9.961 0 0 1 12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10c0 2.136-.67 4.116-1.81 5.74L17 12h3a8 8 0 1 0-2.46 5.772l.997 1.795z' fill='rgba(255,255,255,1)'/></svg>";
pub const BACK: &'static str = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' width='24' height='24'><path fill='none' d='M0 0h24v24H0z'/><path d='M5.828 7l2.536 2.536L6.95 10.95 2 6l4.95-4.95 1.414 1.414L5.828 5H13a8 8 0 1 1 0 16H4v-2h9a6 6 0 1 0 0-12H5.828z' fill='rgba(255,255,255,1)'/></svg>";

pub struct TopBarStyle;

impl StyleSheet for TopBarStyle {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Some(Background::Color(Color::from_rgb8(100, 100, 100))),
            ..Default::default()
        }
    }
}

impl From<TopBarStyle> for iced::theme::Container {
    fn from(style: TopBarStyle) -> Self {
        iced::theme::Container::Custom(Box::new(style))
    }
}

pub fn build(browser: &ZonkeyBrowser) -> Container<Message> {
    let search_box = TextInput::new(
        "Enter address",
        &browser.current_tab().address_field,
        Message::AddressChanged,
    )
    .on_submit(Message::AddressConfirmed);

    let settings_svg = Svg::new(Handle::from_memory(SETTINGS.as_bytes()));
    let settings_button = Button::new(settings_svg).on_press(Message::SettingsPressed);

    let home_svg = Svg::new(Handle::from_memory(HOME.as_bytes()));
    let home_button = Button::new(home_svg).on_press(Message::HomePressed);

    let reload_svg = Svg::new(Handle::from_memory(RELOAD.as_bytes()));
    let reload_button = Button::new(reload_svg).on_press(Message::ReloadPressed);

    let back_svg = Svg::new(Handle::from_memory(BACK.as_bytes()));
    let back_button = Button::new(back_svg).on_press(Message::BackPressed);

    let content = Row::new()
        .push(home_button)
        .push(back_button)
        .push(reload_button)
        .push(search_box)
        .push(settings_button)
        .align_items(Alignment::Center)
        .spacing(30);

    Container::new(content)
        .width(Length::Fill)
        .padding(10)
        .center_x()
        .style(TopBarStyle)
}
