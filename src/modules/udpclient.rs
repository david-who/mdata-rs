use std::net::{UdpSocket, SocketAddr};
use std::io::{Result, Error, ErrorKind};

// 封装UDP套接字
pub struct UDPClient {
    socket: UdpSocket,
}

impl UDPClient {
    // 创建一个新的UDP套接字
    pub fn new(addr: &str) -> Result<Self> {
        let socket: UdpSocket = UdpSocket::bind(addr)?;
        Ok(Self { socket })
    }

    // 发送数据
    #[allow(non_snake_case)]
    pub fn SendTo(&self, buf: &[u8], addr: &str) -> Result<usize> {
        let dest_addr: SocketAddr = addr.parse().map_err(|_| Error::new(ErrorKind::Other, "Invalid socket address"))?;
        self.socket.send_to(buf, &dest_addr)
    }

    // 接收数据
    #[allow(non_snake_case)]
    #[warn(dead_code)]
    pub fn RecvFrom(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        self.socket.recv_from(buf)
    }
}