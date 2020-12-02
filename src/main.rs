mod helpers;
mod server;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080".to_string();
    let socket = UdpSocket::bind(&addr).await?;

    let public_key_pem = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDC+Jx89MjzbWw9PPh0dffD+i2c
J7XMioLndImQvQiNJjZ00zyxjgt4+wkual+ZHhH94HIjRIeLI+ncBEjFMa1xIzHT
exz/pvJUCsHNxNK9958zR0E997xxSf3C2Lu8BWtJG348xd5QNzb+R+i963PtcAsQ
fCu+q5gbqqtQEIjlMwIDAQAB
-----END PUBLIC KEY-----";

    let private_key_pem = "-----BEGIN RSA PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: AES-128-CBC,43371B6CECDB096AC2A362FD33BF4B07

aIs3x9UBN95VJJFsd1ddYxmwAKQdFE5BJwZVYtidV+cZ4Qpmg9tdBLm8AhF5bVGR
FzAVMxTEFQgwT4o2jH2UxRkRmChwNy6aqdGteDIK6yXQK7//GMmxhbvqMmFzwdof
2E7Jkq3BQQEqMFu2CxRUPUFYRIebEIZSDCD3PoJ6p7a77qwm/KCXCbad/DqtOGkJ
wOkPH5AXLIu02MJfs+vcLswXFMlq7aaUrAv5WGt1SpKz9Co6bplSYDG7JE+906Uw
MIg4XDJTJDKCKyDaPkMydw6StvyNuZfIYUNIofulLci7yoNEGvwQHsHCaHr6n4bt
I4iC9CbkEcPbf06HAWGFfsexeLGf9mU0HVsZi83QdMhWMbOREakFU755AMvTeB8w
IMCNn55nzJlSHooKuvJAmbqBBb4+wqgwnoYQEVZmTDZxqT/eR08Zl9d1QeKB+1fw
gjZmY/10kFLnTKlWGIaLIu60ehbXxZeFbW+m1pF9uHEiIkWgkrHNjKfzWh5EyfhY
vXxWuZH92ZP/nioGzVQr00oSEPLwW1RSoAx3jPuu1EILNu7lFL896CsDZpa1Oigf
OMxk0GhMuKs4H6TlHmx5a0TOAcGYWEbnqXi+KUw7pMPFiEs1/2crFI6QfQx8R7dL
/ohKFvksPExsB196RZ1PFyMdryOr/mCqI4nBT+KzPz4zJF2iTMGq3NFQI2MvW/4g
WMwsyQtIJQviFJpYlQpOVBFaeB69oHJMxfauM8OdEU8yomFl3sAVagNxPfiWsGt4
LRsReK2BDT/pnhhZG96qSsNPwQlrwffBleTy9BGSuHHox6A7GKyVAAOMND/TY1ak
-----END RSA PRIVATE KEY-----";
    println!("Listening on {}", socket.local_addr()?);
    let server = server::Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
        public_key_pem,
        private_key_pem,
    };
    server.run().await?;
    Ok(())
}
