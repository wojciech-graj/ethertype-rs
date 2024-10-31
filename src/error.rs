use core::{fmt, num::ParseIntError};

/// An error which can be returned when parsing an [`EtherType`](`crate::EtherType`)
#[derive(Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum ParseEtherTypeError {
    InvalidLength,
    ParseInt(ParseIntError),
}

impl fmt::Display for ParseEtherTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseEtherTypeError::InvalidLength => {
                write!(f, "expected a 4-digit hexadecimal string")
            }
            ParseEtherTypeError::ParseInt(err) => write!(f, "{}", err),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
mod std {
    use ::std::error::Error;

    use super::*;

    impl Error for ParseEtherTypeError {}
}
