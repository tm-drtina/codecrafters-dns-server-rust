use std::net::UdpSocket;

use dns_starter_rust::message::*;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);

                let message = Message {
                    header: Header {
                        id: 1234,
                        is_reply: true,
                        opcode: Opcode::QUERY,
                        authoritative: false,
                        truncation: false,
                        recursion_desired: false,
                        recursion_available: false,
                        rcode: RCode::NoError,
                        question_count: 1,
                        answer_count: 1,
                        authority_count: 0,
                        additional_count: 0,
                    },
                    questions: vec![Question {
                        qname: vec![b"codecrafters".to_vec(), b"io".to_vec()],
                        qtype: QType::A,
                        qclass: QClass::IN,
                    }],
                    answers: vec![Answer {
                        name: vec![b"codecrafters".to_vec(), b"io".to_vec()],
                        rtype: QType::A,
                        rclass: QClass::IN,
                        ttl: 60,
                        rdlength: 4,
                        rdata: vec![0x08, 0x08, 0x08, 0x08],
                    }],
                };

                udp_socket
                    .send_to(&message.as_bytes(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
