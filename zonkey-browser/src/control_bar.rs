use crate::{message::Message, ZonkeyBrowser};
use tab::{iced, iced_native::color};
use iced::{
    widget::{
        container::{Appearance, StyleSheet},
        Button, TextInput,
    },
    widget::{svg::Handle, Container, Row, Svg},
    Alignment, Background, Length, Theme,
};

// Icons from remixicon.com
pub const HOME: &'static str = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' width='24' height='24'><path fill='none' d='M0 0h24v24H0z'/><path d='M21 20a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V9.49a1 1 0 0 1 .386-.79l8-6.222a1 1 0 0 1 1.228 0l8 6.222a1 1 0 0 1 .386.79V20zm-2-1V9.978l-7-5.444-7 5.444V19h14z'/></svg>";
pub const RELOAD: &'static str = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' width='24' height='24'><path fill='none' d='M0 0h24v24H0z'/><path d='M18.537 19.567A9.961 9.961 0 0 1 12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10c0 2.136-.67 4.116-1.81 5.74L17 12h3a8 8 0 1 0-2.46 5.772l.997 1.795z'/></svg>";
pub const BACK: &'static str = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' width='24' height='24'><path fill='none' d='M0 0h24v24H0z'/><path d='M5.828 7l2.536 2.536L6.95 10.95 2 6l4.95-4.95 1.414 1.414L5.828 5H13a8 8 0 1 1 0 16H4v-2h9a6 6 0 1 0 0-12H5.828z'/></svg>";
pub const ZOOM_IN: &'static str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path d="M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748ZM10 10V7H12V10H15V12H12V15H10V12H7V10H10Z"></path></svg>"#;
pub const ZOOM_OUT: &'static str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path d="M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748ZM7 10H15V12H7V10Z"></path></svg>"#;
pub const ADD: &'static str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path d="M11 11V5H13V11H19V13H13V19H11V13H5V11H11Z" fill="\#000"></path></svg>"#;

pub struct TopBarStyle;

impl StyleSheet for TopBarStyle {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Some(Background::Color(color!(0xe1e2e2))),
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
    )
    .on_input(Message::AddressChanged)
    .on_submit(Message::AddressConfirmed);

    let home_svg = Svg::new(Handle::from_memory(HOME.as_bytes()));
    let home_button = Button::new(home_svg).on_press(Message::HomePressed);

    let reload_svg = Svg::new(Handle::from_memory(RELOAD.as_bytes()));
    let reload_button = Button::new(reload_svg).on_press(Message::ReloadPressed);

    let back_svg = Svg::new(Handle::from_memory(BACK.as_bytes()));
    let back_button = Button::new(back_svg).on_press(Message::BackPressed);

    let add_tab_svg = Svg::new(Handle::from_memory(ADD.as_bytes()));
    let add_tab_button = Button::new(add_tab_svg).on_press(Message::NewTab);

    let zoom_in_svg = Svg::new(Handle::from_memory(ZOOM_IN.as_bytes()));
    let zoom_in_button = Button::new(zoom_in_svg).on_press(Message::ZoomIn);

    let zoom_out_svg = Svg::new(Handle::from_memory(ZOOM_OUT.as_bytes()));
    let zoom_out_button = Button::new(zoom_out_svg).on_press(Message::ZoomOut);

    let content = Row::new()
        .push(home_button)
        .push(reload_button)
        .push(back_button)
        .push(add_tab_button)
        .push(search_box)
        .push(zoom_in_button)
        .push(zoom_out_button)
        .align_items(Alignment::Center)
        .spacing(30);

    Container::new(content)
        .width(Length::Fill)
        .padding(10)
        .center_x()
        .style(TopBarStyle)
}
