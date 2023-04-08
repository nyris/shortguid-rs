#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use shortguid::ShortGuid;

#[derive(Arbitrary, Debug)]
struct Input {
    bytes: [u8; 16],
    string: String,
}

fuzz_target!(|input: Input| {
    let id = ShortGuid::from_bytes_ref(&input.bytes);
    let _ = id.eq(&input.string);
});
