use std::net::UdpSocket;

use dns_starter_rust::message::*;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let request = Message::from_bytes(&buf[0..size]);

                let questions = request.questions;
                let answers: Vec<_> = questions.iter().map(|q| {
                    Answer {
                        name: q.qname.clone(),
                        rtype: q.qtype,
                        rclass: q.qclass,
                        ttl: 60,
                        rdlength: 4,
                        rdata: vec![0x08, 0x08, 0x08, 0x08],
                    }
                }).collect();

                let message = Message {
                    header: Header {
                        id: request.header.id,
                        is_reply: true,
                        opcode: request.header.opcode,
                        authoritative: false,
                        truncation: false,
                        recursion_desired: request.header.recursion_desired,
                        recursion_available: false,
                        rcode: if request.header.opcode == Opcode::Query { RCode::NoError } else { RCode::NotImplemented },
                        question_count: questions.len() as u16,
                        answer_count: answers.len() as u16,
                        authority_count: 0,
                        additional_count: 0,
                    },
                    questions,
                    answers,
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
