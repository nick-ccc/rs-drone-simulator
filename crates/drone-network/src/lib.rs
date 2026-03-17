use std::io;
use std::net::UdpSocket;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Serialize, Deserialize};

// uses derive to generate code at compile time
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heartbeat {
    pub id: u32,
    pub timestamp: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub battery: f32,
}

pub struct HeartbeatSender {
    socket: UdpSocket,
    target_addr: String,
}

impl HeartbeatSender {
    pub fn new(target_addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_nonblocking(false)?;

        Ok(Self {
            socket,
            target_addr: target_addr.to_string(),
        })
    }

    pub fn send() {}
}
