use std::net::TcpStream;
use crate::util::env::_get_gateway_url;
use tungstenite::{
    connect,
    stream::MaybeTlsStream,
    WebSocket
};

pub use tungstenite::{Message, Error};
pub type WebsocketStream = WebSocket<MaybeTlsStream<TcpStream>>; 

pub struct Socket {
    socket: WebsocketStream
}

impl Socket {
    pub fn new(host: &str) -> Self {
        let (mut socket, _) = connect(host)
            .expect("Could not connect to the gateway api socket");

        // Set the stream to be non-blocking
        // This is critical to the functioning of the crate
        match socket.get_mut() {
            MaybeTlsStream::NativeTls(tls_stream) => {
                let tcp_stream = tls_stream.get_mut();
                let _ = tcp_stream.set_nonblocking(true);
            },
            MaybeTlsStream::Plain(tcp_stream) => {
                let _ = tcp_stream.set_nonblocking(true);
            },
            _ => panic!("Stream was not MaybeTlsStream variant")
        }

        Self { socket }
    }

    /// Connect to Discord's gateway socket
    pub fn gateway() -> Self {
        let host = _get_gateway_url().unwrap();
        Self::new(&host)
    }

    pub fn read(&mut self) -> Result<Message, Error> {
        self.socket.read()
    }

    pub fn send(&mut self, message: Message) -> Result<(), Error> {
        self.socket.send(message)
    }
}