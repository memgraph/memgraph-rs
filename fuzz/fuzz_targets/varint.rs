#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate memgraph;

fuzz_target!(|input: u64| {
    let serialized: Vec<u8> = memgraph::varint::from_int(input);
    let deserialized: u64 = memgraph::varint::from_bytes(&serialized);
    assert_eq!(input, deserialized);
});

