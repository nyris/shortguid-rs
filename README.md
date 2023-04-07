# ShortGuid

Short URL-safe Base64 encoded UUIDs.

```rust
fn it_works() {
    let uuid = Uuid::try_parse("c9a646d3-9c61-4cb7-bfcd-ee2522c8f633").unwrap();
    let short_guid_1 = ShortGuid::from(uuid);
    let short_guid_2 = ShortGuid::try_parse("c9a646d3-9c61-4cb7-bfcd-ee2522c8f633").unwrap();
    let short_guid_3 = ShortGuid::try_parse("yaZG05xhTLe_ze4lIsj2Mw").unwrap();

    assert_eq!(short_guid_1, "yaZG05xhTLe_ze4lIsj2Mw");
    assert_eq!(short_guid_2, short_guid_1);
    assert_eq!(short_guid_3, uuid);
}
```

## Fuzzing

This uses `cargo-fuzz`. See `fuzz/fuzzers` for the available fuzzing scripts. To run, use an invocation like these:

```
cargo +nightly fuzz run roundtrip
cargo +nightly fuzz run decode_random
cargo +nightly fuzz run encode_random
```
