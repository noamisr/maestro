use rosc::{OscMessage, OscPacket, OscType};
use std::net::UdpSocket;
use std::sync::Mutex;

pub struct OscClient {
    socket: Mutex<UdpSocket>,
    target: String,
}

impl OscClient {
    pub fn new(target_port: u16) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_nonblocking(false)?;
        Ok(Self {
            socket: Mutex::new(socket),
            target: format!("127.0.0.1:{}", target_port),
        })
    }

    pub fn send(
        &self,
        address: &str,
        args: Vec<OscType>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let msg = OscMessage {
            addr: address.to_string(),
            args,
        };
        let packet = OscPacket::Message(msg);
        let buf = rosc::encoder::encode(&packet)?;
        let socket = self.socket.lock().map_err(|e| e.to_string())?;
        socket.send_to(&buf, &self.target)?;
        log::debug!("OSC sent: {} -> {}", address, self.target);
        Ok(())
    }
}
