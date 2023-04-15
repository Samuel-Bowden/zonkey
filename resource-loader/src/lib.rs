use directories::ProjectDirs;
use reqwest::blocking::Response;
use std::{
    fmt,
    fs::{self, read_to_string, File},
    io::{BufReader, Read},
    path::PathBuf,
};

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

#[derive(Clone)]
pub enum Address {
    Zonkey(String),
    File(String),
    HTTP(bool, String),      // True if using HTTPS
    Invalid(String, String), // The bad address, then the error string
}

#[derive(Debug)]
pub enum AddressErr {
    FileSystemFailure(std::io::Error),
    NetworkFailure(reqwest::Error),
    InvalidAddress(String),
}

impl fmt::Display for AddressErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileSystemFailure(e) => write!(f, "Failed to read file - {e}"),
            Self::NetworkFailure(e) => write!(f, "Failed to read over network - {e}"),
            Self::InvalidAddress(e) => write!(f, "Could not load as the address is invalid - {e}"),
        }
    }
}

impl Address {
    pub fn new(string: &str) -> Self {
        let mut it = string.splitn(2, ":");

        let first_section = match it.next() {
            Some(sec) => sec,
            None => return Address::Invalid(string.into(), "The provided address is empty".into()),
        };

        let second_section = match it.next() {
            Some(sec) => sec,
            None => {
                // Assume to be a file if the address is not split into sections
                return Address::File(first_section.to_string());
            }
        };

        match first_section {
            "zonkey" => Address::Zonkey(second_section.into()),
            "file" => Address::File(second_section.into()),
            "http" => Address::HTTP(false, second_section.into()),
            "https" => Address::HTTP(true, second_section.into()),
            invalid_section => Address::Invalid(
                string.into(),
                format!(
                    "'{}' is not a valid first section of an address",
                    invalid_section
                ),
            ),
        }
    }

    pub fn load_script(&self) -> Result<String, AddressErr> {
        match self {
            Self::Zonkey(location) => file_system_read(data_dir!().join(location)),
            Self::File(location) => file_system_read(location.into()),
            Self::HTTP(secure, location) => {
                match network_read(http_name(*secure), location)?.text() {
                    Ok(script) => Ok(script),
                    Err(e) => Err(AddressErr::NetworkFailure(e)),
                }
            }
            Self::Invalid(bad_address, error) => Ok(invalid_address_script(&bad_address, error)),
        }
    }

    pub fn read_string(&self) -> Result<String, AddressErr> {
        match self {
            Self::Zonkey(location) => file_system_read(data_dir!().join(location)),
            Self::File(location) => file_system_read(location.into()),
            Self::HTTP(secure, location) => {
                match network_read(http_name(*secure), location)?.text() {
                    Ok(script) => Ok(script),
                    Err(e) => Err(AddressErr::NetworkFailure(e)),
                }
            }
            Self::Invalid(_, error) => Err(AddressErr::InvalidAddress(error.into())),
        }
    }

    pub fn write_string(&self, string: String) -> Result<String, AddressErr> {
        match self {
            Self::Zonkey(location) | Self::File(location) => {
                let dir = if let Self::Zonkey(_) = self {
                    data_dir!().join(location)
                } else {
                    location.into()
                };

                match file_system_write(dir, string) {
                    Ok(()) => Ok("OK".to_string()),
                    Err(e) => Err(e),
                }
            }
            Self::HTTP(secure, location) => network_write(http_name(*secure), location, string),
            Self::Invalid(_, error) => Err(AddressErr::InvalidAddress(error.into())),
        }
    }

    pub fn load_image(&self) -> Vec<u8> {
        match self {
            Self::Zonkey(_) => self.load_bytes(),
            Self::File(_) => self.load_bytes(),
            Self::HTTP(secure, location) => {
                if let Ok(response) = network_read(http_name(*secure), location) {
                    if let Ok(data) = response.bytes() {
                        return data.to_vec();
                    }
                }

                include_bytes!("image_load_failed.png").to_vec()
            }
            Self::Invalid(..) => include_bytes!("image_load_failed.png").to_vec(),
        }
    }

    fn load_bytes(&self) -> Vec<u8> {
        let mut reader = match File::open(self.get_path()) {
            Ok(file) => BufReader::new(file),
            Err(_) => return vec![],
        };
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();
        buffer
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
            Self::Invalid(bad_address, _) => write!(f, "{}", bad_address),
        }
    }
}

fn file_system_read(path: PathBuf) -> Result<String, AddressErr> {
    match read_to_string(path) {
        Ok(source) => Ok(source),
        Err(e) => Err(AddressErr::FileSystemFailure(e)),
    }
}

fn file_system_write(path: PathBuf, string: String) -> Result<(), AddressErr> {
    match fs::write(path, string) {
        Ok(_) => Ok(()),
        Err(e) => Err(AddressErr::FileSystemFailure(e)),
    }
}

pub fn network_read(protocol: &str, location: &str) -> Result<Response, AddressErr> {
    match reqwest::blocking::get(protocol.to_string() + ":" + location) {
        Ok(response) => match response.error_for_status() {
            Ok(response) => Ok(response),
            Err(e) => Err(AddressErr::NetworkFailure(e)),
        },
        Err(e) => Err(AddressErr::NetworkFailure(e)),
    }
}

pub fn network_write(protocol: &str, location: &str, string: String) -> Result<String, AddressErr> {
    let client = reqwest::blocking::Client::new();
    match client
        .post(protocol.to_string() + ":" + location)
        .body(string)
        .send()
    {
        Ok(response) => match response.error_for_status() {
            Ok(response) => match response.text() {
                Ok(text) => Ok(text),
                Err(e) => Err(AddressErr::NetworkFailure(e)),
            },
            Err(e) => Err(AddressErr::NetworkFailure(e)),
        },
        Err(e) => Err(AddressErr::NetworkFailure(e)),
    }
}

fn invalid_address_script(bad_address: &str, error: &str) -> String {
    format!(
        "
        start {{
            Page()
                .set_title(\"Invalid address\")
                .add(Text(\"Invalid address\").set_size(100.))
                .add(Text(\"The entered address '{bad_address}' is not valid.\"))
                .add(Text(\"{error}.\"));
        }}
        "
    )
}
