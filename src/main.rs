use std::net::UdpSocket;

use dns_starter_rust::message::{Header, Opcode, RCode};

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                
                let header = Header {
                    id: 1234,
                    is_reply: true,
                    opcode: Opcode::QUERY,
                    authoritative: false,
                    truncation: false,
                    recursion_desired: false,
                    recursion_available: false,
                    rcode: RCode::NoError,
                    question_count: 0,
                    answer_count: 0,
                    authority_count: 0,
                    additional_count: 0,
                };

                udp_socket
                    .send_to(&header.bytes(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
