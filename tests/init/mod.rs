use once_cell::sync::OnceCell;
use std::net::{SocketAddr, TcpListener};

pub fn init_global_server() -> SocketAddr {
    static ADDR: OnceCell<SocketAddr> = OnceCell::new();
    ADDR.get_or_init(|| spawn_app()).clone()
}

fn spawn_app() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let addr = listener.local_addr().unwrap();

    let server = zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    addr
}
