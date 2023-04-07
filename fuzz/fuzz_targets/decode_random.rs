#![no_main]

use libfuzzer_sys::fuzz_target;
use shortguid::ShortGuid;

fuzz_target!(|id: ShortGuid| {
    let str = id.to_string();
    let id2 = ShortGuid::try_parse(str).expect("decoding should work");
    assert_eq!(id, id2);
});
