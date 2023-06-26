use super::{message::Message, Window};
use interpreter::iced::{
    self,
    widget::{
        container::{Appearance, StyleSheet},
        Button, TextInput,
    },
    widget::{svg::Handle, Container, Row, Svg},
    Alignment, Background, Length, Theme,
};
use interpreter::iced_native::color;

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

impl Window {
    pub fn control_bar_build(&self) -> Container<Message> {
        let search_box = TextInput::new("Enter address", &self.current_tab().address_field)
            .on_input(Message::AddressChanged)
            .on_submit(Message::AddressConfirmed);

        let home_svg = Svg::new(Handle::from_memory(include_str!("remix-icons/home.svg").as_bytes()));
        let home_button = Button::new(home_svg).on_press(Message::HomePressed);

        let reload_svg = Svg::new(Handle::from_memory(include_str!("remix-icons/reload.svg").as_bytes()));
        let reload_button = Button::new(reload_svg).on_press(Message::ReloadPressed);

        let back_svg = Svg::new(Handle::from_memory(include_str!("remix-icons/back.svg").as_bytes()));
        let back_button = Button::new(back_svg).on_press(Message::BackPressed);

        let add_tab_svg = Svg::new(Handle::from_memory(include_str!("remix-icons/add.svg").as_bytes()));
        let mut add_tab_button = Button::new(add_tab_svg);

        if self.tabs.len() < 8 {
            add_tab_button = add_tab_button.on_press(Message::NewTab);
        }

        let zoom_in_svg = Svg::new(Handle::from_memory(include_str!("remix-icons/zoom_in.svg").as_bytes()));
        let mut zoom_in_button = Button::new(zoom_in_svg);

        if self.zoom_level < 2.0 {
            zoom_in_button = zoom_in_button.on_press(Message::ZoomIn);
        }

        let zoom_out_svg = Svg::new(Handle::from_memory(include_str!("remix-icons/zoom_out.svg").as_bytes()));
        let mut zoom_out_button = Button::new(zoom_out_svg);

        if self.zoom_level > 0.5 {
            zoom_out_button = zoom_out_button.on_press(Message::ZoomOut);
        }

        let settings_svg = Svg::new(Handle::from_memory(include_str!("remix-icons/settings.svg").as_bytes()));
        let settings_button = Button::new(settings_svg).on_press(Message::SettingsPressed);

        let content = Row::new()
            .push(home_button)
            .push(reload_button)
            .push(back_button)
            .push(add_tab_button)
            .push(search_box)
            .push(zoom_in_button)
            .push(zoom_out_button)
            .push(settings_button)
            .align_items(Alignment::Center)
            .spacing(30);

        Container::new(content)
            .width(Length::Fill)
            .padding(10)
            .center_x()
            .style(TopBarStyle)
    }
}
