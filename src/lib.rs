//! A crate providing the [`EtherType`] type for representing IEEE 802 EtherType values.
//!
//! [`EtherType`] has associated constants for the most common EtherType values, the descriptions of
//! which can be obtained using [`EtherType::description`].
//!
//! Additionally, descriptions from
//! [IANA](https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml) can be obtained
//! with [`EtherType::iana_description`]. Protocol descriptions and the names of the organizations
//! that registered them sourced from the
//! [IEEE Registration Authority](http://standards.ieee.org/develop/regauth) can be obtained with
//! [`EtherType::ieee_description`] and [`EtherType::ieee_organization`] respectively.

#![deny(clippy::pedantic)]
#![allow(clippy::doc_markdown, clippy::must_use_candidate)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use core::{fmt, str::FromStr};

use error::ParseEtherTypeError;

mod consts;
pub mod error;

#[cfg(feature = "ieee")]
mod ieee;

#[cfg(feature = "iana")]
mod iana;

#[cfg(feature = "desc")]
mod desc;

#[cfg(feature = "serde")]
mod serde;

/// The EtherType field in an Ethernet header, as specified in IEEE 802.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EtherType(pub u16);

impl From<u16> for EtherType {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<EtherType> for u16 {
    fn from(value: EtherType) -> Self {
        value.0
    }
}

impl fmt::Display for EtherType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:04x}", self.0)
    }
}

impl FromStr for EtherType {
    type Err = ParseEtherTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(ParseEtherTypeError::InvalidLength);
        }

        u16::from_str_radix(s, 16)
            .map(EtherType)
            .map_err(ParseEtherTypeError::ParseInt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "std")]
    fn display() {
        let ether_type = EtherType(0x0123);
        assert_eq!("0123", ether_type.to_string())
    }

    #[test]
    fn from_str() {
        assert_eq!(EtherType(0x0123), "0123".parse::<EtherType>().unwrap());
    }

    #[test]
    fn from_str_length_error() {
        assert_eq!(
            Err(ParseEtherTypeError::InvalidLength),
            "123".parse::<EtherType>()
        );
    }
}
