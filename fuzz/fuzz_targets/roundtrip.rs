#![no_main]

use libfuzzer_sys::fuzz_target;
use shortguid::ShortGuid;

fuzz_target!(|bytes: [u8; 16]| {
    let id = ShortGuid::from_bytes_ref(&bytes);
    assert_eq!(&bytes, id.as_bytes());
});
