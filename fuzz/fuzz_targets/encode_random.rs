#![no_main]

use libfuzzer_sys::fuzz_target;
use shortguid::ShortGuid;

fuzz_target!(|str: String| {
    // Since the input data is random, this likely won't
    // parse correctly. As long as it returns an Err()
    // instead of panicking, it's considered correct behavior.
    let _ = ShortGuid::try_parse(&str);
});
