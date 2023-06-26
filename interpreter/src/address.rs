use directories_next::ProjectDirs;
use include_dir::{include_dir, Dir};
use reqwest::blocking::Response;
use std::{
    fmt,
    fs::{self, read_to_string, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

pub static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../assets");

fn http_name(secure: bool) -> &'static str {
    if secure {
        "https"
    } else {
        "http"
    }
}

#[derive(Clone)]
pub struct Address {
    pub address_type: AddressType,
    pub arguments: Vec<String>,
    pub location: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AddressType {
    Zonkey,
    File,
    Installed,
    HTTP { secure: bool },
}

#[derive(Debug)]
pub enum AddressErr {
    FileSystemFailure(std::io::Error),
    NetworkFailure(reqwest::Error),
    InvalidAddress(String),
    ZonkeyAssetError(String),
}

impl fmt::Display for AddressErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddressErr::FileSystemFailure(e) => write!(f, "Failed to read file - {e}"),
            AddressErr::NetworkFailure(e) => write!(f, "Failed to read over network - {e}"),
            AddressErr::InvalidAddress(e) => {
                write!(f, "Could not load as the address is invalid - {e}")
            }
            AddressErr::ZonkeyAssetError(e) => write!(f, "Failed to read zonkey asset - {e}"),
        }
    }
}

impl Address {
    pub fn new(string: &str, arguments: Vec<String>) -> Self {
        let mut it = string.splitn(2, ":");

        let first_section = match it.next() {
            Some(sec) => sec,
            None => return Self::invalid_address(vec!["The provided address is empty".into()]),
        };

        let mut second_section = match it.next() {
            Some(sec) => sec.to_string(),
            None => {
                // Assume to be a file if the address is not split into sections
                return Self {
                    address_type: AddressType::File,
                    location: first_section.to_string(),
                    arguments,
                };
            }
        };

        let address_type = match first_section {
            "zonkey" => AddressType::Zonkey,
            "file" => AddressType::File,
            "installed" => {
                let proj_dirs = ProjectDirs::from("rocks.sambowden", "", "zonkey").unwrap();
                let mut path = PathBuf::from(proj_dirs.data_dir());
                path.push(second_section);
                second_section = path.as_path().display().to_string();
                AddressType::File
            }
            "http" => AddressType::HTTP { secure: false },
            "https" => AddressType::HTTP { secure: true },
            invalid_section => {
                return Self::invalid_address(vec![format!(
                    "'{}' is not a valid first section of an address",
                    invalid_section
                )])
            }
        };

        Self {
            address_type,
            arguments,
            location: second_section,
        }
    }

    fn invalid_address(arguments: Vec<String>) -> Self {
        Self {
            address_type: AddressType::Zonkey,
            arguments,
            location: "invalid_address.zonk".into(),
        }
    }

    pub fn read_string(&self) -> Result<String, AddressErr> {
        match &self.address_type {
            AddressType::Zonkey => zonkey_asset_read(&self.location),
            AddressType::File | AddressType::Installed => {
                file_system_read(Path::new(&self.location))
            }
            AddressType::HTTP { secure } => {
                match network_read(http_name(*secure), &self.location)?.text() {
                    Ok(script) => Ok(script),
                    Err(e) => Err(AddressErr::NetworkFailure(e)),
                }
            }
        }
    }

    pub fn write_string(&self, string: String) -> Result<String, AddressErr> {
        match &self.address_type {
            AddressType::File | AddressType::Installed => {
                match file_system_write(Path::new(&self.location), string) {
                    Ok(()) => Ok("OK".to_string()),
                    Err(e) => Err(e),
                }
            }
            AddressType::HTTP { secure } => {
                network_write(http_name(*secure), &self.location, string)
            }
            AddressType::Zonkey => Err(AddressErr::ZonkeyAssetError(
                "Cannot overwrite internal zonkey assets".into(),
            )),
        }
    }

    pub fn load_bytes(&self) -> Result<Vec<u8>, String> {
        match &self.address_type {
            AddressType::Zonkey => match zonkey_asset_read_bytes(&self.location) {
                Ok(bytes) => Ok(bytes),
                Err(e) => Err(e.to_string()),
            },
            AddressType::File | AddressType::Installed => match file_read_bytes(&self.location) {
                Ok(bytes) => Ok(bytes),
                Err(e) => Err(e.to_string()),
            },
            AddressType::HTTP { secure } => {
                match network_read(http_name(*secure), &self.location) {
                    Ok(response) => match response.bytes() {
                        Ok(data) => Ok(data.to_vec()),
                        Err(e) => Err(e.to_string()),
                    },
                    Err(e) => Err(e.to_string()),
                }
            }
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match &self.address_type {
            AddressType::File => "file",
            AddressType::Installed => "installed",
            AddressType::Zonkey => "zonkey",
            AddressType::HTTP { secure } => http_name(*secure),
        };
        write!(f, "{}:{}", name, self.location)
    }
}

fn zonkey_asset_read(path: &str) -> Result<String, AddressErr> {
    match PROJECT_DIR.get_file(path) {
        Some(file) => match file.contents_utf8() {
            Some(contents) => Ok(contents.into()),
            None => Err(AddressErr::ZonkeyAssetError(
                "File is not valid UTF8".into(),
            )),
        },
        None => Err(AddressErr::InvalidAddress(format!(
            "Internal Zonkey asset does not exist - {}",
            path
        ))),
    }
}

fn zonkey_asset_read_bytes(path: &str) -> Result<Vec<u8>, AddressErr> {
    match PROJECT_DIR.get_file(path) {
        Some(file) => Ok(file.contents().to_vec()),
        None => Err(AddressErr::InvalidAddress(format!(
            "Internal Zonkey asset does not exist - {}",
            path
        ))),
    }
}

fn file_system_read(path: &Path) -> Result<String, AddressErr> {
    match read_to_string(path) {
        Ok(source) => Ok(source),
        Err(e) => Err(AddressErr::FileSystemFailure(e)),
    }
}

fn file_system_write(path: &Path, string: String) -> Result<(), AddressErr> {
    match fs::write(path, string) {
        Ok(_) => Ok(()),
        Err(e) => Err(AddressErr::FileSystemFailure(e)),
    }
}

fn file_read_bytes(location: &str) -> Result<Vec<u8>, AddressErr> {
    let mut reader = match File::open(Path::new(location)) {
        Ok(file) => BufReader::new(file),
        Err(e) => return Err(AddressErr::FileSystemFailure(e)),
    };
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    Ok(buffer)
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

#[cfg(test)]
mod tests {
    use crate::{address::AddressType, Address};

    #[test]
    fn zonkey_address_ok() {
        let address = Address::new("zonkey:home.zonk", vec![]);
        assert_eq!(address.location, "home.zonk");
        assert_eq!(address.address_type, AddressType::Zonkey);
    }

    #[test]
    fn installed_address_ok() {
        let address = Address::new("installed:Calculator/app.zonk", vec![]);
        assert!(address.location.contains("Calculator/app.zonk"));
        assert_eq!(address.address_type, AddressType::File);
    }

    #[test]
    fn http_address_ok() {
        let address = Address::new("http://somewhere.com/app.zonk", vec![]);
        assert_eq!(address.location, "//somewhere.com/app.zonk");
        assert_eq!(address.address_type, AddressType::HTTP { secure: false });
    }

    #[test]
    fn https_address_ok() {
        let address = Address::new("https://localhost:8000/documents/test.zonk", vec![]);
        assert_eq!(address.location, "//localhost:8000/documents/test.zonk");
        assert_eq!(address.address_type, AddressType::HTTP { secure: true });
    }

    #[test]
    fn file_addresses_ok() {
        let address = Address::new("file:/home/user/documents/scripts/test.zonk", vec![]);
        assert_eq!(address.location, "/home/user/documents/scripts/test.zonk");
        assert_eq!(address.address_type, AddressType::File);

        let address = Address::new(
            "file:D:\\Documents\\Pictures\\Holiday\\First Picture.png",
            vec![],
        );
        assert_eq!(
            address.location,
            "D:\\Documents\\Pictures\\Holiday\\First Picture.png"
        );
        assert_eq!(address.address_type, AddressType::File);

        let address = Address::new("/home/user/documents/scripts/test.zonk", vec![]);
        assert_eq!(address.location, "/home/user/documents/scripts/test.zonk");
        assert_eq!(address.address_type, AddressType::File);
    }
}
