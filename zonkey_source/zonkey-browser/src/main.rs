mod app;

use self::app::ZonkeyBrowser;
use iced::{Application, Settings};

pub fn main() -> iced::Result {
    ZonkeyBrowser::run(Settings::default())
}
