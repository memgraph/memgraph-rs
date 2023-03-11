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

use futures::executor::block_on;

use memgraph::*;

#[test]
fn basic_request() {
    let handle: Arc<dyn memgraph::io::Handle> = Arc::new(memgraph::simulator::Simulator::default());

    let cli_addr = Address {
        id: 0,
        port: 1,
        ip_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
    };

    let cli_io = memgraph::io::Io {
        timeout: std::time::Duration::from_millis(100),
        handle: handle.clone(),
        address: cli_addr,
    };

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

    let _srv_thread = std::thread::spawn(move || {
        println!("server receiving");
        let req = block_on(srv_io.receive()).expect("request should not time out");
        dbg!(&req);

        let res = Message::Coordinator(RsmMessage::VoteRes {
            success: true,
            term: Term(0),
            committed_log_size: 0,
        });

        srv_io.send(req.from, req.request_id, res);
    });

    let req = Message::Coordinator(RsmMessage::VoteReq {
        proposed_leadership_term: Term(0),
        last_log_term: None,
        committed_log_size: 0,
    });

    block_on(cli_io.request(srv_address, req)).unwrap();

    println!("client got response");
}
