use directories::ProjectDirs;
use error::load_address::LoadAddressErr;
use error::new_address::NewAddressErr;
use iced::widget::Image;
use iced_native::image::Handle;
use reqwest::blocking::Response;
use std::{fmt, fs::read_to_string, path::PathBuf};

macro_rules! data_dir {
    () => {
        ProjectDirs::from("rocks.sambowden", "", "zonkey-browser")
            .unwrap()
            .data_dir()
    };
}

fn http_name(secure: bool) -> &'static str {
    if secure {
        "https"
    } else {
        "http"
    }
}

mod error;

pub enum Address {
    Zonkey(String),
    File(String),
    HTTP(bool, String), // True if using HTTPS
    InvalidAddress(NewAddressErr),
    FailedToLoadAddress(LoadAddressErr),
    ScriptError,
}

impl Address {
    pub fn new(string: &str) -> Result<Self, NewAddressErr> {
        let mut it = string.splitn(2, ":");

        let first_section = match it.next() {
            Some(sec) => Ok(sec),
            None => Err(NewAddressErr::Empty),
        }?;

        let second_section = match it.next() {
            Some(sec) => Ok(sec),
            None => {
                // Assume to be a file if the address is not split into sections
                return Ok(Address::File(first_section.to_string()));
            }
        }?;

        match first_section {
            "zonkey" => Ok(Address::Zonkey(second_section.into())),
            "file" => Ok(Address::File(second_section.into())),
            "http" => Ok(Address::HTTP(false, second_section.into())),
            "https" => Ok(Address::HTTP(true, second_section.into())),
            _ => Err(NewAddressErr::InvalidFirstSection),
        }
    }

    pub fn load_script(&self) -> Result<String, LoadAddressErr> {
        match self {
            Self::Zonkey(location) => file_system_load(data_dir!().join(location)),
            Self::File(location) => file_system_load(location.into()),
            Self::HTTP(secure, location) => {
                match network_load(http_name(*secure), location)?.text() {
                    Ok(script) => Ok(script),
                    Err(e) => Err(LoadAddressErr::NetworkFailure(e)),
                }
            }
            Self::InvalidAddress(error) => Ok(invalid_script(error)),
            Self::FailedToLoadAddress(error) => Ok(failed_to_load_address_script(error)),
            Self::ScriptError => Ok(script_error()),
        }
    }

    pub fn load_image(&self) -> Result<Image, LoadAddressErr> {
        match self {
            Self::Zonkey(_) => Ok(Image::new(self.get_path())),
            Self::File(location) => Ok(Image::new(location)),
            Self::HTTP(secure, location) => {
                match network_load(http_name(*secure), location)?.bytes() {
                    Ok(data) => Ok(Image::new(Handle::from_memory(data))),
                    Err(e) => Err(LoadAddressErr::NetworkFailure(e)),
                }
            }
            _ => panic!("Not the address of an image"),
        }
    }

    pub fn get_path(&self) -> PathBuf {
        match self {
            Self::File(location) => PathBuf::from(location),
            Self::Zonkey(location) => data_dir!().join(location),
            _ => panic!("Cannot get path of non filesystem types"),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::File(location) => write!(f, "file:{}", location),
            Self::Zonkey(location) => write!(f, "zonkey:{}", location),
            Self::HTTP(secure, location) => write!(f, "{}:{}", http_name(*secure), location),
            Self::InvalidAddress(_) => write!(f, "error:invalid_address"),
            Self::ScriptError => write!(f, "error:script_failed"),
            Self::FailedToLoadAddress(_) => write!(f, "error:failed_to_load_address"),
        }
    }
}

fn file_system_load(path: PathBuf) -> Result<String, LoadAddressErr> {
    match read_to_string(path) {
        Ok(source) => Ok(source),
        Err(e) => Err(LoadAddressErr::FileSystemFailure(e)),
    }
}

pub fn network_load(protocol: &str, location: &str) -> Result<Response, LoadAddressErr> {
    match reqwest::blocking::get(protocol.to_string() + ":" + location) {
        Ok(response) => Ok(response),
        Err(e) => Err(LoadAddressErr::NetworkFailure(e)),
    }
}

fn invalid_script(error: &NewAddressErr) -> String {
    format!(
        "
        start {{
            Page()
                .set_title(\"Invalid address\")
                .add(Text(\"Invalid address\").set_size(100.))
                .add(Text(\"{error}\"));
        }}
    "
    )
}

fn failed_to_load_address_script(error: &LoadAddressErr) -> String {
    format!(
        "
        start {{
            Page()
                .set_title(\"Failed to load page\")
                .add(Text(\"Failed to load page\").set_size(100.))
                .add(Text(\"{error}\"));
        }}
    "
    )
}

fn script_error() -> String {
    format!("
        start {{
            let further_information_button = Button(\"Further information for developers\");
            let further_information_open = false;

            let further_information = Text(\"Detailed error information has been printed to the standard error stream. Please use the Zonkey command line interface to see and diagnose these errors.\");

            let page = Page()
                .set_title(\"Failed to load page\")
                .add(Text(\"Failed to load page\").set_size(100.))
                .add(Text(\"Execution of the script for the requested page failed.\"))
                .add(Text(\"You can contact the developer of the application to notify them of this error on their page.\"))
                .add(further_information_button);

            while (wait_for_event()) {{
                if (further_information_button.clicked() & !(further_information_open)) {{
                    page
                        .add(further_information);
                    further_information_open = true;
                }} else {{
                    page
                        .remove(further_information);
                    further_information_open = false;
                }}
            }}
        }}
    ")
}
