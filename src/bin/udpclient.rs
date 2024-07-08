use std::net::{UdpSocket, SocketAddr};
use std::io::{Result, Error, ErrorKind};

// 封装UDP套接字
struct UDPClient {
    socket: UdpSocket,
}

impl UDPClient {
    // 创建一个新的UDP套接字
    fn new(addr: &str) -> Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self { socket })
    }

    // 发送数据
    #[allow(non_snake_case)]
    fn SendTo(&self, buf: &[u8], addr: &str) -> Result<usize> {
        let dest_addr: SocketAddr = addr.parse().map_err(|_| Error::new(ErrorKind::Other, "Invalid socket address"))?;
        self.socket.send_to(buf, &dest_addr)
    }

    // 接收数据
    #[allow(non_snake_case)]
    fn RecvFrom(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        self.socket.recv_from(buf)
    }
}

// 示例用法
fn main() -> Result<()> {
// 创建一个封装了UDP套接字的对象
    let udpclient = UDPClient::new("127.0.0.1:8080")?;
    println!("UDP socket bound to {}", "127.0.0.1:8080");

// 发送UDP数据
    let udp_buf = b"Hello, UDP!";
    udpclient.SendTo(udp_buf, "127.0.0.1:8888")?;
    println!("UDP message sent");

// 接收UDP数据
    let mut udp_recv_buf = [0u8; 1024];
    let (udp_recv_len, udp_src_addr) = udpclient.RecvFrom(&mut udp_recv_buf)?;
    println!("Received {} bytes from {}: {:?}", udp_recv_len, udp_src_addr, &udp_recv_buf[..udp_recv_len]);

    Ok(())
}