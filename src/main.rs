use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

const UNCONNECTED_PING: &'static [u8] = &[
    0x1u8, // UNCONNECTED_PING ID
    0x0, 0x0, 0x0, 0x0, 0x0, 0xD, 0x8, 0x8B, // ping-time
    0x0, 0xFF, 0xFF, 0x0, 0xFE, 0xFE, 0xFE, 0xFE, 0xFD, 0xFD, 0xFD, 0xFD, 0x12, 0x34, 0x56, 0x78, // magic
//    0xAB, 0x93, 0x18, 0x92, 0x1F, 0xEA, 0xC5, 0x5E // unknown
];

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:21119").expect("couldn't bind to address");
    println!("Bound to 0.0.0.0:21119");
    println!("UDP Sending: {:X?}", UNCONNECTED_PING);
    let server_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 19132);

    socket.send_to(UNCONNECTED_PING, server_address);

    loop {
        let mut buf = [0; 10];
        let (length, address) = socket.recv_from(&mut buf).expect("Didn't receive data");
        if length > 0 {
            let buf = &mut buf[..length];
            println!("UDP Received {} bytes: {:X?}", length, buf);
        }
    }
}
