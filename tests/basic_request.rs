use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use memgraph::*;

#[test]
fn basic_request() {
    let handle: Arc<dyn memgraph::io::Handle> = Arc::new(memgraph::simulator::Simulator::default());

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

    let srv_thread = std::thread::spawn(move || {
        println!("server receiving");
        let req = extreme::run(srv_io.receive()).expect("request should not time out");
        dbg!(&req);

        let res = Message::VoteRes(memgraph::VoteRes {
            success: true,
            last_log_term: Term(0),
            log_size: 0,
            term: Term(0),
        });

        srv_io.send(req.from, req.request_id, res);
    });

    let req = Message::VoteReq(memgraph::VoteReq {
        last_log_term: Term(0),
        log_size: 0,
        term: Term(0),
    });
    let res = extreme::run(cli_io.request(srv_address, req));
    dbg!(res);
    println!("client got response");
}
