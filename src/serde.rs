// The original serde code is taken from the uuid crate at https://github.com/uuid-rs/uuid,
// licensed under an MIT or Apache-2.0 license and copyrighted as follows:
//
// Copyright (c) 2014 The Rust Project Developers
// Copyright (c) 2018 Ashley Mannix, Christopher Armstrong, Dylan DPC, Hunar Roop Kahlon
// Copyright (c) 2023 Markus Mayer
//
// SPDX-License-Identifier: EUPL-1.2 or MIT or Apache-2.0

use crate::{ParseError, ShortGuid};
use std::fmt::Formatter;
use uuid::Uuid;

#[cfg(feature = "serde")]
impl serde::Serialize for ShortGuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(Self::encode(self.0).as_str())
        } else {
            serializer.serialize_bytes(self.as_bytes())
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ShortGuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        fn de_error<E: serde::de::Error>(e: ParseError) -> E {
            E::custom(format_args!("ShortGuid parsing failed: {}", e))
        }

        if deserializer.is_human_readable() {
            struct ShortGuidVisitor;

            impl<'vi> serde::de::Visitor<'vi> for ShortGuidVisitor {
                type Value = ShortGuid;

                fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(formatter, "a ShortGuid string")
                }

                fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<ShortGuid, E> {
                    value.parse::<ShortGuid>().map_err(de_error)
                }

                fn visit_bytes<E: serde::de::Error>(self, value: &[u8]) -> Result<ShortGuid, E> {
                    ShortGuid::from_slice(value).map_err(de_error)
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<ShortGuid, A::Error>
                where
                    A: serde::de::SeqAccess<'vi>,
                {
                    use serde::de::Error;
                    #[rustfmt::skip]
                        let bytes: [u8; 16] = [
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                        match seq.next_element()? { Some(e) => e, None => return Err(Error::invalid_length(16, &self)) },
                    ];

                    Ok(ShortGuid::from_bytes(&bytes))
                }
            }

            deserializer.deserialize_str(ShortGuidVisitor)
        } else {
            let uuid = Uuid::deserialize(deserializer)?;
            Ok(ShortGuid::from(uuid))
        }
    }
}
