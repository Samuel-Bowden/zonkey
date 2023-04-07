use std::{fmt, fs::read_to_string, path::PathBuf};

use directories::ProjectDirs;
use load_address_err::LoadAddressErr;
use new_address_err::NewAddressErr;

macro_rules! data_dir {
    () => {
        ProjectDirs::from("rocks.sambowden", "", "zonkey-browser")
            .unwrap()
            .data_dir()
    };
}

mod load_address_err;
mod new_address_err;

pub enum Address {
    Zonkey(String),
    File(String),
    InvalidAddress,
    ScriptError,
}

impl Address {
    pub fn new(string: &str) -> Result<Self, NewAddressErr> {
        let mut it = string.split(":");

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

        if let Some(_) = it.next() {
            return Err(NewAddressErr::TooManySections);
        }

        match first_section {
            "zonkey" => Ok(Address::Zonkey(second_section.into())),
            "file" => Ok(Address::File(second_section.into())),
            "error" => match second_section {
                "invalid_address" => Ok(Address::InvalidAddress),
                "script_failed" => Ok(Address::ScriptError),
                _ => Err(NewAddressErr::InvalidErrType),
            },
            _ => Err(NewAddressErr::InvalidFirstSection),
        }
    }

    pub fn load_script(&self) -> Result<String, LoadAddressErr> {
        match self {
            Self::Zonkey(location) => load_file(data_dir!().join(location)),
            Self::File(location) => load_file(location.into()),
            Self::InvalidAddress => Ok(invalid_script()),
            Self::ScriptError => Ok(script_error()),
        }
    }

    pub fn get_path(&self) -> PathBuf {
        match self {
            Self::File(location) => PathBuf::from(location),
            Self::Zonkey(location) => data_dir!().join(location),
            _ => panic!("Cannot get the path of an inbuilt script"),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::File(location) => write!(f, "file:{}", location),
            Self::Zonkey(location) => write!(f, "zonkey:{}", location),
            Self::InvalidAddress => write!(f, "error:invalid_address"),
            Self::ScriptError => write!(f, "error:script_failed"),
        }
    }
}

fn load_file(path: PathBuf) -> Result<String, LoadAddressErr> {
    match read_to_string(path) {
        Ok(source) => Ok(source),
        Err(e) => Err(LoadAddressErr::FailedToLoadFile(e)),
    }
}

fn invalid_script() -> String {
    stringify!(
        start {
            Page()
                .set_title("Invalid Page")
                .add(Text("Invalid page").set_size(100.))
                .add(Text("The entered page could not be found."));
        }
    )
    .to_string()
}

fn script_error() -> String {
    stringify!(
        start {
            let further_information_button = Button("Further information for developers");
            let further_information_open = false;

            let further_information = Text("Detailed error information has been printed to the standard error stream. Please use the Zonkey command line interface to see and diagnose these errors.");

            let page = Page()
                .set_title("Failed to load page")
                .add(Text("Failed to load page").set_size(100.))
                .add(Text("Execution of the script for the requested page failed."))
                .add(Text("You can contact the developer of the application to notify them of this error on their page."))
                .add(further_information_button);

            while (wait_for_event()) {
                if (further_information_button.clicked() & !(further_information_open)) {
                    page
                        .add(further_information);
                    further_information_open = true;
                } else {
                    page
                        .remove(further_information);
                    further_information_open = false;
                }
            }
        }
    )
    .to_string()
}
