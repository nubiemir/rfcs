use std::time::{SystemTime, UNIX_EPOCH};

fn main_tcp(args: &[String]) {
    let is_client = args.len() > 2 && args[2] == "client";
    if is_client {
        let _ = rust_rfcs::rfc862::tcp::client();
        return;
    }
    let _ = rust_rfcs::rfc862::tcp::serve();
}

fn main_udp(args: &[String]) {
    let is_client = args.len() > 2 && args[2] == "client";
    if is_client {
        let num = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            % 10;

        println!("{}", num);

        let _ = rust_rfcs::rfc862::udp::client(num as u8);
        return;
    }
    let _ = rust_rfcs::rfc862::udp::serve();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "udp" {
        main_udp(&args);
        return;
    }
    main_tcp(&args);
}

// this is how to execute udp client.
// cargo run --example rfc862 -- upd client;
//
// this is how to execute tcp clinet.
// cargo run --example rfc862 -- tcp client;
//
// for server start remove client from the respective protocol run.
