// The original test code is taken from the uuid crate at https://github.com/uuid-rs/uuid,
// licensed under an MIT or Apache-2.0 license and copyrighted as follows:
//
// Copyright (c) 2014 The Rust Project Developers
// Copyright (c) 2018 Ashley Mannix, Christopher Armstrong, Dylan DPC, Hunar Roop Kahlon
// Copyright (c) 2023 Markus Mayer
//
// SPDX-License-Identifier: EUPL-1.2 or MIT or Apache-2.0

use serde_test::{Compact, Configure, Readable, Token};
use shortguid::ShortGuid;

#[test]
fn test_serialize_readable_string() {
    let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
    let shortguid_str = "-RaMXs6yT6q2vzKb85-h5A";
    let u = ShortGuid::try_parse(uuid_str).unwrap();
    serde_test::assert_tokens(&u.readable(), &[Token::Str(shortguid_str)]);
}

#[test]
fn test_deserialize_readable_compact() {
    let uuid_bytes = b"F9168C5E-CEB2-4F";
    let u = ShortGuid::from_slice(uuid_bytes).unwrap();

    serde_test::assert_de_tokens(
        &u.readable(),
        &[
            Token::Tuple { len: 16 },
            Token::U8(uuid_bytes[0]),
            Token::U8(uuid_bytes[1]),
            Token::U8(uuid_bytes[2]),
            Token::U8(uuid_bytes[3]),
            Token::U8(uuid_bytes[4]),
            Token::U8(uuid_bytes[5]),
            Token::U8(uuid_bytes[6]),
            Token::U8(uuid_bytes[7]),
            Token::U8(uuid_bytes[8]),
            Token::U8(uuid_bytes[9]),
            Token::U8(uuid_bytes[10]),
            Token::U8(uuid_bytes[11]),
            Token::U8(uuid_bytes[12]),
            Token::U8(uuid_bytes[13]),
            Token::U8(uuid_bytes[14]),
            Token::U8(uuid_bytes[15]),
            Token::TupleEnd,
        ],
    );
}

#[test]
fn test_deserialize_readable_bytes() {
    let uuid_bytes = b"F9168C5E-CEB2-4F";
    let u = ShortGuid::from_slice(uuid_bytes).unwrap();
    serde_test::assert_de_tokens(&u.readable(), &[Token::Bytes(uuid_bytes)]);
}

#[test]
fn test_serialize_non_human_readable() {
    let uuid_bytes = b"F9168C5E-CEB2-4F";
    let u = ShortGuid::from_slice(uuid_bytes).unwrap();
    serde_test::assert_tokens(
        &u.compact(),
        &[Token::Bytes(&[
            70, 57, 49, 54, 56, 67, 53, 69, 45, 67, 69, 66, 50, 45, 52, 70,
        ])],
    );
}

#[test]
fn test_de_failure() {
    serde_test::assert_de_tokens_error::<Readable<ShortGuid>>(
        &[Token::Str("hello_world")],
        "ShortGuid parsing failed: Invalid ID length; expected 22 characters, but got 11",
    );

    serde_test::assert_de_tokens_error::<Compact<ShortGuid>>(
        &[Token::Bytes(b"hello_world")],
        "UUID parsing failed: invalid length: expected 16 bytes, found 11",
    );
}
