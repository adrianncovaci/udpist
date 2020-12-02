use openssl::rsa::{Padding, Rsa};
use std::io::Write;
use std::io::{stdin, Read};
use std::net::SocketAddr;
use std::{error::Error, path::PathBuf};
use tokio::net::UdpSocket;

fn get_stdin_data() -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.into_bytes())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let remote_address: SocketAddr = "127.0.0.1:8080".parse()?;
    let local_address: SocketAddr = "0.0.0.0:0".parse()?;
    let socket = UdpSocket::bind(local_address).await?;
    let public_key_pem = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDC+Jx89MjzbWw9PPh0dffD+i2c
J7XMioLndImQvQiNJjZ00zyxjgt4+wkual+ZHhH94HIjRIeLI+ncBEjFMa1xIzHT
exz/pvJUCsHNxNK9958zR0E997xxSf3C2Lu8BWtJG348xd5QNzb+R+i963PtcAsQ
fCu+q5gbqqtQEIjlMwIDAQAB
-----END PUBLIC KEY-----";
    const MAX_DATAGRAM_SIZE: usize = 65_507;
    socket.connect(&remote_address).await?;
    loop {
        let data = get_stdin_data()?;
        let rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes()).unwrap();
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let _ = rsa.public_encrypt(&data, &mut buf, Padding::PKCS1).unwrap();
        println!("data: {:?}", data);
        socket.send(&buf).await?;
        println!("sent {:?}", data);
        let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
        let len = socket.recv(&mut data).await?;
        println!(
            "Received {} bytes\n{}",
            len,
            String::from_utf8_lossy(&data[..len])
        );
    }
}
