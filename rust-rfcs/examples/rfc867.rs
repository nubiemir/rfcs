fn main_tcp(args: &[String]) {
    let is_client = args.len() > 2 && args[2] == "client";
    if is_client {
        let _ = rust_rfcs::rfc867::tcp_client();
        return;
    }
    let _ = rust_rfcs::rfc867::tcp_server();
}

fn main_udp(args: &[String]) {
    let is_client = args.len() > 2 && args[2] == "client";
    if is_client {
        let _ = rust_rfcs::rfc867::upd_client();
        return;
    }
    let _ = rust_rfcs::rfc867::udp_server();
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
// cargo run --example rfc867 -- upd client;
//
// this is how to execute tcp clinet.
// cargo run --example rfc867 -- tcp client;
//
// for server start remove client from the respective protocol run.
