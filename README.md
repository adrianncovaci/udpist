## udpist

The purpose of the project
------------

This is a semi-reliable ftp-like protocol, which supports encryption and encoding of packets using RSA, ftp-like file management commands, result codes based on the result of the command at the transport layer, custom errors


Implementation
------------

This project was implemented in rustlang.
I've used the tokio's asynchronous runtime to run asynchronous code, and tokio_utils crate to handle data encoding, also the Bytes crate as a wrapper for a vec<u8>.

##### handler.rs
Handles custom cmd struct

##### command.rs
parses bytes received from the client into a `Cmd` struct

##### ftp.rs
Result codes defined from the actual rfc of ftp.

##### error.rs
Custom error types (io, utf8 related) and `Error::Msg` which allows to pass a custom exception

##### server.rs
Receives packets from the client, decodes, decrypts the content and parses it to a `Cmd` struct, and then it *handles* the command. After receiving a Response Code, it sends it back to the client.

##### client.rs
encrypts, encodes data and sends it to the server

##### main.rs
    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {
        let addr = "127.0.0.1:8080".to_string();
        let socket = UdpSocket::bind(&addr).await?;
    
        let public_key_pem = "-----BEGIN PUBLIC KEY-----
                              -----END PUBLIC KEY-----";
    
        let private_key_pem = "-----BEGIN RSA PRIVATE KEY-----
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
