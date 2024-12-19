#![allow(unreachable_patterns)]

use crate::EtherType;

impl EtherType {
    /// A brief textual description of the [`EtherType`] sourced from the [IEEE Registration Authority](http://standards.ieee.org/develop/regauth).
    pub const fn ieee_description(self) -> Option<&'static str> {
        Some(match self.0 {
            _ => return None,
        })
    }

    /// The organization that registered the [`EtherType`] sourced from the [IEEE Registration Authority](http://standards.ieee.org/develop/regauth).
    pub const fn ieee_organization(self) -> Option<&'static str> {
        Some(match self.0 {
            _ => return None,
        })
    }
}
