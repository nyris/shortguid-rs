//! Provides short, URL-safe UUID representations.
//!
//! ```
//! # use shortguid::ShortGuid;
//! let from_uuid = ShortGuid::try_parse("c9a646d3-9c61-4cb7-bfcd-ee2522c8f633").unwrap();
//! let from_short = ShortGuid::try_parse("yaZG05xhTLe_ze4lIsj2Mw").unwrap();
//! assert_eq!(from_uuid, "yaZG05xhTLe_ze4lIsj2Mw");
//! assert_eq!(from_uuid, from_short);
//!
//! let random = ShortGuid::new_random();
//! assert_ne!(from_uuid, random);
//! ```

// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "serde")]
mod serde;

use base64::{DecodeError, Engine};
use std::borrow::Borrow;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

/// A short, URL-safe UUID representation.
///
/// ## Example
///
/// The [`ShortGuid`] can be constructed from an existing [`Uuid`]:
///
/// ```
/// # use uuid::Uuid;
/// # use shortguid::ShortGuid;
/// let uuid = Uuid::try_parse("c9a646d3-9c61-4cb7-bfcd-ee2522c8f633").unwrap();
/// let short_guid = ShortGuid::from(uuid);
/// assert_eq!(short_guid, "yaZG05xhTLe_ze4lIsj2Mw");
/// assert_eq!(short_guid, uuid);
/// ```
///
/// Alternatively, it can be directly parsed from a UUID:
///
/// ```
/// # use shortguid::ShortGuid;
/// let short_guid_a = ShortGuid::try_parse("c9a646d3-9c61-4cb7-bfcd-ee2522c8f633").unwrap();
/// let short_guid_b = ShortGuid::try_parse("yaZG05xhTLe_ze4lIsj2Mw").unwrap();
/// assert_eq!(short_guid_a, "yaZG05xhTLe_ze4lIsj2Mw");
/// assert_eq!(short_guid_a, short_guid_b);
/// ```
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[repr(transparent)]
pub struct ShortGuid(Uuid);

/// A short UUID format.
impl ShortGuid {
    /// Generates a new [`ShortGuid`] based on a random UUID v4.
    #[cfg_attr(docsrs, doc(cfg(feature = "random")))]
    #[cfg(feature = "random")]
    #[inline(always)]
    pub fn new_random() -> Self {
        Self::new_from_uuid(Uuid::new_v4())
    }

    /// Creates a new [`ShortGuid`] based on the provided [`Uuid`].
    #[inline(always)]
    pub const fn new_from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Tries to parse the value as a [`ShortGuid`] or [`Uuid`] string, and outputs an actual
    /// [`ShortGuid`] instance.
    pub fn try_parse<S: AsRef<str>>(value: S) -> Result<Self, ParseError> {
        if let Ok(uuid) = Uuid::try_parse(value.as_ref()) {
            return Ok(Self(uuid));
        }

        let uuid = Self::try_decode(value)?;
        Ok(Self(uuid))
    }

    /// Creates a [`ShortGuid`] using the supplied bytes.
    #[inline]
    pub fn from_slice<B: AsRef<[u8]>>(bytes: B) -> Result<Self, ParseError> {
        let uuid = Uuid::from_slice(bytes.as_ref()).map_err(|e| ParseError::InvalidSlice(e))?;
        Ok(Self(uuid))
    }

    /// Constructs a [`ShortGuid`] instance based on a byte slice.
    ///
    /// ## Notes
    /// This will clone the underlying data. If you wish to return a
    /// transparent reference around the provided slice, use [`ShortGuid::from_bytes_ref`]
    /// instead.
    #[inline]
    pub fn from_bytes<B: Borrow<[u8; 16]>>(bytes: B) -> Self {
        Self(Uuid::from_bytes_ref(bytes.borrow()).clone())
    }

    /// Returns a slice of 16 octets containing the value.
    ///
    /// This method borrows the underlying byte value of the UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shortguid::ShortGuid;
    /// let bytes1 = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    /// let uuid1 = ShortGuid::from_bytes_ref(&bytes1);
    ///
    /// let bytes2 = uuid1.as_bytes();
    /// let uuid2 = ShortGuid::from_bytes_ref(bytes2);
    ///
    /// assert_eq!(uuid1, uuid2);
    ///
    /// assert!(std::ptr::eq(
    ///     uuid2 as *const ShortGuid as *const u8,
    ///     &bytes1 as *const [u8; 16] as *const u8,
    /// ));
    /// ```
    #[inline]
    pub const fn from_bytes_ref(bytes: &[u8; 16]) -> &Self {
        // SAFETY: `Bytes`, `Uuid` and `ShortGuid have the same ABI
        unsafe { &*(bytes as *const [u8; 16] as *const Uuid as *const ShortGuid) }
    }

    /// Tests if this [`ShortGuid`] is all zeros.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_nil()
    }

    /// Returns the underlying [`Uuid`] instance.
    #[inline]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Returns a slice of 16 octets containing the value.
    ///
    /// This method borrows the underlying byte value of the UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shortguid::ShortGuid;
    /// let bytes1 = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    /// let uuid1 = ShortGuid::from_bytes_ref(&bytes1);
    ///
    /// let bytes2 = uuid1.as_bytes();
    /// let uuid2 = ShortGuid::from_bytes_ref(bytes2);
    ///
    /// assert_eq!(uuid1, uuid2);
    ///
    /// assert!(std::ptr::eq(
    ///     uuid2 as *const ShortGuid as *const u8,
    ///     &bytes1 as *const [u8; 16] as *const u8,
    /// ));
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }

    /// Returns the bytes of the [`ShortGuid`] in little-endian order.
    ///
    /// The bytes will be flipped to convert into little-endian order. This is
    /// based on the endianness of the underlying UUID, rather than the target environment
    /// so bytes will be flipped on both big and little endian machines.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shortguid::ShortGuid;
    ///
    /// # fn main() -> Result<(), shortguid::ParseError> {
    /// let uuid = ShortGuid::try_parse("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
    ///
    /// assert_eq!(
    ///     uuid.to_bytes_le(),
    ///     ([
    ///         0xa4, 0xa3, 0xa2, 0xa1, 0xb2, 0xb1, 0xc2, 0xc1, 0xd1, 0xd2,
    ///         0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8
    ///     ])
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub const fn to_bytes_le(&self) -> [u8; 16] {
        self.0.to_bytes_le()
    }

    /// Decodes the given value to a [`Uuid`].
    ///
    /// ## Arguments
    /// * `value` - A 22 character ShortGuid URL-safe Base64 string.
    fn try_decode<S: AsRef<str>>(value: S) -> Result<Uuid, ParseError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Ok(Uuid::default());
        }

        if value.len() != 22 {
            return Err(ParseError::InvalidLength(value.len()));
        }

        // This particular alphabet replaces '/' with '_' and '+' with '-'.
        let engine = &base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let value = engine.decode(value)?;
        if value.len() != 16 {
            return Err(ParseError::InvalidLength(value.len()));
        }

        let bytes: [u8; 16] = value.try_into().expect("array has 16 elements");
        let uuid = Uuid::from_bytes(bytes);
        Ok(uuid)
    }

    /// Encodes the given [`Uuid`] value to an encoded [`ShortGuid`] string.
    /// The encoding is similar to base-64, with some non-URL safe characters replaced
    /// and padding removed.
    ///
    /// ## Returns
    /// A 22 character ShortGuid URL-safe Base64 string.
    fn encode<U: Borrow<Uuid>>(value: U) -> String {
        let bytes = value.borrow().as_bytes();

        // This particular alphabet replaces '/' with '_' and '+' with '-'.
        let engine = &base64::engine::general_purpose::URL_SAFE_NO_PAD;

        let mut buf = String::with_capacity(22);
        engine.encode_string(bytes, &mut buf);
        debug_assert_eq!(buf.len(), 22);
        buf
    }
}

impl Debug for ShortGuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{short} ({long})",
            short = Self::encode(&self.0),
            long = self.0
        )
    }
}

impl Display for ShortGuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{short}", short = Self::encode(&self.0))
    }
}

impl From<Uuid> for ShortGuid {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<ShortGuid> for Uuid {
    fn from(value: ShortGuid) -> Self {
        value.0
    }
}

impl TryFrom<String> for ShortGuid {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ShortGuid::try_parse(value)
    }
}

impl TryFrom<&str> for ShortGuid {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ShortGuid::try_parse(value)
    }
}

impl PartialEq<Uuid> for ShortGuid {
    fn eq(&self, other: &Uuid) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<String> for ShortGuid {
    fn eq(&self, other: &String) -> bool {
        if let Ok(uuid) = ShortGuid::try_decode(other) {
            return self.0.eq(&uuid);
        }

        if let Ok(uuid) = Uuid::try_parse(other) {
            return self.0.eq(&uuid);
        }

        false
    }
}

impl PartialEq<str> for ShortGuid {
    fn eq(&self, other: &str) -> bool {
        if let Ok(uuid) = ShortGuid::try_decode(other) {
            return self.0.eq(&uuid);
        }

        if let Ok(uuid) = Uuid::try_parse(other) {
            return self.0.eq(&uuid);
        }

        false
    }
}

impl PartialEq<&str> for ShortGuid {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<Vec<u8>> for ShortGuid {
    fn eq(&self, other: &Vec<u8>) -> bool {
        other.len() == 16 && self.as_bytes().eq(other.as_slice())
    }
}

impl PartialEq<&[u8]> for ShortGuid {
    fn eq(&self, other: &&[u8]) -> bool {
        other.len() == 16 && self.as_bytes().eq(other)
    }
}

impl PartialEq<&[u8; 16]> for ShortGuid {
    fn eq(&self, other: &&[u8; 16]) -> bool {
        self.as_bytes().eq(*other)
    }
}

impl PartialEq<[u8; 16]> for ShortGuid {
    fn eq(&self, other: &[u8; 16]) -> bool {
        self.as_bytes().eq(other)
    }
}

impl Borrow<Uuid> for ShortGuid {
    fn borrow(&self) -> &Uuid {
        self.as_uuid()
    }
}

impl AsRef<Uuid> for ShortGuid {
    fn as_ref(&self) -> &Uuid {
        self.as_uuid()
    }
}

impl AsRef<[u8]> for ShortGuid {
    fn as_ref(&self) -> &[u8] {
        self.as_uuid().as_ref()
    }
}

/// A parsing error.
#[derive(Eq, PartialEq)]
pub enum ParseError {
    /// The provided input had an invalid length.
    /// The contained value is the actual size.
    InvalidLength(usize),
    /// The provided input had an invalid format.
    /// The contained value is the underlying decoding error.
    InvalidFormat(DecodeError),
    /// The provided slice input was invalid.
    InvalidSlice(uuid::Error),
}

impl From<DecodeError> for ParseError {
    fn from(value: DecodeError) -> Self {
        Self::InvalidFormat(value)
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidLength(len) => write!(
                f,
                "Invalid ID length; expected 22 characters, but got {len}"
            ),
            ParseError::InvalidFormat(err) => write!(f, "Invalid ID format: {err}"),
            ParseError::InvalidSlice(err) => write!(f, "Invalid slice: {err}"),
        }
    }
}

impl FromStr for ShortGuid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ShortGuid::try_parse(s)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn debug_works() {
        assert_eq!(
            format!("{:?}", ShortGuid::default()),
            "AAAAAAAAAAAAAAAAAAAAAA (00000000-0000-0000-0000-000000000000)".to_string()
        );
    }

    #[test]
    fn display_works() {
        assert_eq!(
            format!("{}", ShortGuid::default()),
            "AAAAAAAAAAAAAAAAAAAAAA".to_string()
        );
    }

    #[test]
    fn is_empty_works() {
        assert!(ShortGuid::default().is_empty());
    }

    #[test]
    fn new_random_works() {
        let a = ShortGuid::new_random();
        let b = ShortGuid::new_random();
        assert_ne!(a, b);
        assert_ne!(a, ShortGuid::default());
    }

    #[test]
    fn try_parse_works() {
        assert_eq!(
            ShortGuid::try_parse("AAAAAAAAAAAAAAAAAAAAAA").unwrap(),
            "00000000-0000-0000-0000-000000000000"
        );
        assert_eq!(
            ShortGuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap(),
            ShortGuid::default()
        );

        assert_eq!(
            ShortGuid::try_parse("yaZG05xhTLe_ze4lIsj2Mw").unwrap(),
            "c9a646d3-9c61-4cb7-bfcd-ee2522c8f633"
        );
        assert_eq!(
            ShortGuid::try_parse("c9a646d3-9c61-4cb7-bfcd-ee2522c8f633").unwrap(),
            "c9a646d3-9c61-4cb7-bfcd-ee2522c8f633"
        );

        assert_eq!(
            ShortGuid::try_parse("ELina62d0RGAtADAT9QwyA").unwrap(),
            "10b8a76b-ad9d-d111-80b4-00c04fd430c8"
        );
        assert_eq!(
            ShortGuid::try_parse("10b8a76b-ad9d-d111-80b4-00c04fd430c8").unwrap(),
            "10b8a76b-ad9d-d111-80b4-00c04fd430c8"
        );

        assert_eq!(
            ShortGuid::try_parse("4ZOgWsqcM1iE3YmYWinsBA").unwrap(),
            Uuid::from_str("e193a05a-ca9c-3358-84dd-89985a29ec04").unwrap()
        );
        assert_eq!(
            ShortGuid::try_parse("e193a05a-ca9c-3358-84dd-89985a29ec04").unwrap(),
            Uuid::from_str("e193a05a-ca9c-3358-84dd-89985a29ec04").unwrap()
        );
    }

    #[test]
    fn try_decode_works() {
        assert_eq!(
            ShortGuid::try_decode("yaZG05xhTLe_ze4lIsj2Mw").unwrap(),
            Uuid::from_str("c9a646d3-9c61-4cb7-bfcd-ee2522c8f633").unwrap()
        );
        assert_eq!(
            ShortGuid::try_decode("ELina62d0RGAtADAT9QwyA").unwrap(),
            Uuid::from_str("10b8a76b-ad9d-d111-80b4-00c04fd430c8").unwrap()
        );
        assert_eq!(
            ShortGuid::try_decode("4ZOgWsqcM1iE3YmYWinsBA").unwrap(),
            Uuid::from_str("e193a05a-ca9c-3358-84dd-89985a29ec04").unwrap()
        );
        assert_eq!(
            ShortGuid::try_decode("AAAAAAAAAAAAAAAAAAAAAA").unwrap(),
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()
        );
    }

    #[test]
    fn try_decode_with_invalid_input_of_correct_length_fails() {
        assert!(matches!(
            ShortGuid::try_decode("Nothing to see here...").unwrap_err(),
            ParseError::InvalidFormat(..)
        ));
    }

    #[test]
    fn try_decode_with_invalid_input_fails() {
        assert!(matches!(
            ShortGuid::try_decode("Nothing to see here").unwrap_err(),
            ParseError::InvalidLength(..)
        ));
    }

    #[test]
    fn encode_works() {
        assert_eq!(
            ShortGuid::encode(Uuid::from_str("c9a646d3-9c61-4cb7-bfcd-ee2522c8f633").unwrap()),
            "yaZG05xhTLe_ze4lIsj2Mw"
        );
        assert_eq!(
            ShortGuid::encode(Uuid::from_str("10b8a76b-ad9d-d111-80b4-00c04fd430c8").unwrap()),
            "ELina62d0RGAtADAT9QwyA"
        );
        assert_eq!(
            ShortGuid::encode(Uuid::from_str("e193a05a-ca9c-3358-84dd-89985a29ec04").unwrap()),
            "4ZOgWsqcM1iE3YmYWinsBA"
        );
        assert_eq!(
            ShortGuid::encode(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()),
            "AAAAAAAAAAAAAAAAAAAAAA"
        );
    }

    #[test]
    fn eq_array_works() {
        let id = ShortGuid::try_parse("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap();
        let array: [u8; 16] = [
            0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6,
            0xd7, 0xd8,
        ];
        assert_eq!(id, array);
    }

    #[test]
    fn eq_slice_works() {
        let id = ShortGuid::try_parse("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap();
        let slice: &[u8] = &[
            0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6,
            0xd7, 0xd8,
        ];
        assert_eq!(id, slice);
    }
}
