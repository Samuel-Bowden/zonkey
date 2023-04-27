use crate::{address::AddressType, Address};

pub enum PermissionLevel {
    All,
    NetworkOnly,
}

impl PermissionLevel {
    pub fn new(address: &Address) -> Self {
        match address.address_type {
            AddressType::HTTP { .. } => PermissionLevel::NetworkOnly,
            _ => PermissionLevel::All,
        }
    }
}
