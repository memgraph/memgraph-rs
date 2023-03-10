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

use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use memgraph::*;

fn main() {
    let handle: Arc<dyn memgraph::io::Handle> = Arc::new(memgraph::simulator::Simulator::default());

    let srv_address = Address {
        id: 1,
        port: 1,
        ip_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
    };

    let srv_io = memgraph::io::Io {
        address: srv_address,
        timeout: std::time::Duration::from_millis(100),
        handle: handle.clone(),
    };

    let mut mm = MachineManager::recover("test_mm_1", srv_io).unwrap();

    mm.run();
}
