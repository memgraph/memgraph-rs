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
use std::time::Duration;

use futures::executor::block_on;

use memgraph::*;

#[test]
fn rsm() {
    let sim = memgraph::simulator::Simulator::default();
    sim.start_ticker_thread();
    let handle: Arc<dyn memgraph::io::Handle> = Arc::new(sim);

    let cli_addr = Address {
        id: 0,
        ip_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
    };

    let cli_io = memgraph::io::Io {
        timeout: std::time::Duration::from_millis(100),
        handle: handle.clone(),
        address: cli_addr,
    };

    let srv_address = Address {
        id: 1,
        ip_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
    };

    let srv_io = memgraph::io::Io {
        address: srv_address,
        timeout: std::time::Duration::from_millis(100),
        handle: handle.clone(),
    };

    let mut mm = MachineManager::recover("test_mm_1", srv_io).unwrap();

    std::thread::spawn(move || mm.run());

    let shard_addr = Address {
        id: 42,
        ip_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
    };

    let mut coord_client = RsmClient::<Coordinator>::new(vec![srv_address], cli_io.clone());
    let mut shard_client = RsmClient::<Shard>::new(vec![shard_addr], cli_io);

    loop {
        let shard_map_res = block_on(coord_client.read(CoordinatorReadReq::GetShardMap));
        if shard_map_res.is_err() {
            println!("retrying request");
            continue;
        }
        println!("got shard map");
        return;
    }
}
