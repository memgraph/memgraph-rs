// Copyright 2023 Memgraph Ltd.
//
// Use of this software is governed by the Business Source License
// included in the file licenses/BSL.txt; by using this file, you agree to be bound by the terms of the Business Source
// License, and you may not use this file except in compliance with the Business Source License.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0, included in the file
// licenses/APL.txt.

#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate memgraph;

fuzz_target!(|input: u64| {
    let serialized: Vec<u8> = memgraph::varint::from_int(input);
    let deserialized: u64 = memgraph::varint::from_bytes(&serialized);
    assert_eq!(input, deserialized);
});

