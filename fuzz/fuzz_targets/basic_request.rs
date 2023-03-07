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
extern crate arbitrary;
extern crate memgraph;

use std::ops::{Bound, RangeBounds};

use arbitrary::Arbitrary;

#[derive(Debug, Clone, Copy)]
struct B {
    start: Bound<[u8; 11]>,
    end: Bound<[u8; 11]>,
}

impl RangeBounds<[u8; 11]> for B {
    fn start_bound(&self) -> Bound<&[u8; 11]> {
        ref_bound(&self.start)
    }

    fn end_bound(&self) -> Bound<&[u8; 11]> {
        ref_bound(&self.end)
    }
}

fn ref_bound<T>(bound: &Bound<T>) -> Bound<&T> {
    match bound {
        Bound::Unbounded => Bound::Unbounded,
        Bound::Included(x) => Bound::Included(&x),
        Bound::Excluded(x) => Bound::Excluded(&x),
    }
}

impl<'a> Arbitrary<'a> for B {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let information: u8 = Arbitrary::arbitrary(u)?;

        let a: [u8; 11] = Arbitrary::arbitrary(u)?;
        let add: [u8; 11] = Arbitrary::arbitrary(u)?;
        let mut b: [u8; 11] = [0; 11];

        for i in 0..11 {
            b[i] = a[i].saturating_add(add[i]);
        }

        let (start, end) = match information % 8 {
            0 => (Bound::Included(a), Bound::Included(b)),
            1 => (Bound::Included(a), Bound::Excluded(b)),
            2 => (Bound::Included(a), Bound::Unbounded),
            3 => (Bound::Excluded(a), Bound::Included(b)),
            // Excluded..Excluded is skipped because it's not valid
            4 => (Bound::Excluded(a), Bound::Unbounded),
            5 => (Bound::Unbounded, Bound::Included(b)),
            6 => (Bound::Unbounded, Bound::Excluded(b)),
            7 => (Bound::Unbounded, Bound::Unbounded),
            _ => unreachable!(),
        };

        Ok(B { start, end })
    }
}

#[derive(Debug, Arbitrary)]
enum Op {
    Insert([u8; 4], u8),
    Remove([u8; 4]),
    Get([u8; 4]),
    Range(B, bool),
}

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
});
