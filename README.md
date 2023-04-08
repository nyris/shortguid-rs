# ShortGuid

Short URL-safe Base64 encoded UUIDs. 

---

ShortGuids transparently represent UUID types but use only 22 characters 
in their string representation, as opposed to 36 characters for a dashed
UUID or 32 without dashes.

```rust
#[test]
fn it_works() {
    let uuid = Uuid::try_parse("c9a646d3-9c61-4cb7-bfcd-ee2522c8f633").unwrap();
    let from_uuid = ShortGuid::from(uuid);
    let parsed_uuid = ShortGuid::try_parse("c9a646d3-9c61-4cb7-bfcd-ee2522c8f633").unwrap();
    let parsed_short = ShortGuid::try_parse("yaZG05xhTLe_ze4lIsj2Mw").unwrap();

    assert_eq!(from_uuid, uuid);
    assert_eq!(from_uuid, "yaZG05xhTLe_ze4lIsj2Mw");
    assert_eq!(from_uuid, "c9a646d3-9c61-4cb7-bfcd-ee2522c8f633");
    assert_eq!(from_uuid, parsed_uuid);
    assert_eq!(from_uuid, parsed_short);

    let random = ShortGuid::new_random();
    assert_ne!(from_uuid, random);
}
```

## Fuzzing

This uses `cargo-fuzz`. See `fuzz/fuzzers` for the available fuzzing scripts. To run, use an invocation like these:

```
cargo +nightly fuzz run roundtrip
cargo +nightly fuzz run decode_random
cargo +nightly fuzz run encode_random
```
