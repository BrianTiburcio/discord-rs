use std::net::TcpStream;
use crate::util::env::_get_gateway_url;
use tungstenite::{
    connect,
    stream::MaybeTlsStream,
    WebSocket,
    protocol::{
        frame::coding::CloseCode,
        CloseFrame
    }
};

pub use tungstenite::{Message, Error};
pub type WebsocketStream = WebSocket<MaybeTlsStream<TcpStream>>; 

pub struct Socket {
    pub connection: WebsocketStream,
    host: String
}

impl Socket {
    pub fn new(host: &str) -> Self {
        let (mut connection, _) = connect(host)
            .expect("Could not connect to the gateway api socket");

        // Set the stream to be non-blocking
        // This is critical to the functioning of the crate
        match connection.get_mut() {
            MaybeTlsStream::NativeTls(tls_stream) => {
                let tcp_stream = tls_stream.get_mut();
                let _ = tcp_stream.set_nonblocking(true);
            },
            MaybeTlsStream::Plain(tcp_stream) => {
                let _ = tcp_stream.set_nonblocking(true);
            },
            _ => panic!("Stream was not MaybeTlsStream variant")
        }

        Self { connection, host: host.to_string() }
    }

    /// Connect to Discord's gateway socket
    pub fn gateway() -> Self {
        let host = _get_gateway_url().unwrap();
        Self::new(&host)
    }

    // What do you guys do when the unrelenting depression sets in?

    pub fn reconnect(&mut self) {
        // Properly close the connection
        self.connection.close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "Reconnecting".to_string().into(),
        })).ok();
        
        self.connection = connect(&self.host)
            .expect("Could not reconnect to the gateway api socket")
            .0;
    }

    pub fn read(&mut self) -> Result<Message, Error> {
        self.connection.read()
    }

    pub fn send(&mut self, message: Message) -> Result<(), Error> {
        self.connection.send(message)
    }
}