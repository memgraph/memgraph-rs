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

use crc32fast::Hasher;

use memgraph::serialization::*;

fn serialize_u64_varint(x: u64) -> Vec<u8> {
    if x <= 1 {
        [0x1].to_vec()
    } else {
        [0x1, 0x2].to_vec()
    }
}

#[test]
fn basic_serialization() {
    let x = Basic { a: 42, b: 1023 };
    let serialized = bincode::serialize(&x).unwrap();
    let len = serialized.len();
    println!("DATA: {:x?} LENGTH: {}", serialized, len);

    let mut hasher = Hasher::new();
    hasher.update(&serialized);
    let checksum = hasher.finalize();
    println!("HASH: {}", checksum);

    println!("{:x?}", serialize_u64_varint(len as u64));
}
