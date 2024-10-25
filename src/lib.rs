//! A crate providing the [`EtherType`] type for representing IEEE 802 EtherType values.
//!
//! The [`consts`] module contains the most common EtherTypes, the descriptions of which can be
//! obtained using [`EtherType::description`].
//!
//! Additionally, descriptions from
//! [IANA](https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml) can be obtained
//! with [`EtherType::iana_description`]. Protocol descriptions and the names of the organizations
//! that registered them sourced from the
//! [IEEE Registration Authority](http://standards.ieee.org/develop/regauth) can be obtained with
//! [`EtherType::ieee_description`] and [`EtherType::ieee_organization`] respectively.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod consts;

#[cfg_attr(docsrs, doc(cfg(feature = "ieee")))]
#[cfg(feature = "ieee")]
mod ieee;

#[cfg_attr(docsrs, doc(cfg(feature = "iana")))]
#[cfg(feature = "iana")]
mod iana;

#[cfg_attr(docsrs, doc(cfg(feature = "desc")))]
#[cfg(feature = "desc")]
mod desc;

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
