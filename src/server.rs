#[path = "helpers/mod.rs"]
use crate::helpers;

use openssl::rsa::{Padding, Rsa};
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

type Handler = helpers::handler::Handler;
type Cmd = helpers::commands::Cmd;

pub struct Server<'a> {
    pub socket: UdpSocket,
    pub buf: Vec<u8>,
    pub to_send: Option<(usize, SocketAddr)>,
    pub public_key_pem: &'a str,
    pub private_key_pem: &'a str,
}

impl<'a> Server<'a> {
    pub async fn run(self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
            public_key_pem,
            private_key_pem,
        } = self;
        let mut handler = Handler::new();
        loop {
            if let Some((size, peer)) = to_send {
                let passphrase = "rust_by_example";
                let rsa = Rsa::private_key_from_pem_passphrase(
                    private_key_pem.as_bytes(),
                    passphrase.as_bytes(),
                )
                .unwrap();
                let mut dec_buf: Vec<u8> = vec![0; rsa.size() as usize];
                let _ = rsa
                    .private_decrypt(&buf[..size], &mut dec_buf, Padding::PKCS1)
                    .unwrap();
                dec_buf.retain(|x| *x != 0);
                dec_buf.retain(|x| *x != 10);
                if let Ok(command) = Cmd::new(dec_buf.to_vec()) {
                    let response = handler.handle_cmd(command).await;
                    let amount = socket.send_to(&[response as u32 as u8], &peer).await?;
                    println!("Sent {} bytes to {:?}", amount, peer);
                }
            }
            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}
