extern crate serde_cr as serde;

use serde::{de::Error, Deserialize, Serialize};

use crate::EtherType;

impl Serialize for EtherType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format_args!("{:04x}", self.0))
    }
}

impl<'de> Deserialize<'de> for EtherType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let hex = <&str>::deserialize(deserializer)?;

        if hex.len() != 4 {
            return Err(Error::custom("expected a 4-digit hexadecimal string"));
        }

        u16::from_str_radix(hex, 16)
            .map(EtherType)
            .map_err(Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_de_tokens_error, assert_tokens, Token};

    use super::*;

    #[test]
    fn serde() {
        assert_tokens(&EtherType(0x123), &[Token::BorrowedStr("0123")])
    }

    #[test]
    fn deserialize_invalid_length() {
        assert_de_tokens_error::<EtherType>(
            &[Token::BorrowedStr("123")],
            "expected a 4-digit hexadecimal string",
        );
    }
}
