use std::net::UdpSocket;

use dns_starter_rust::message::*;
use dns_starter_rust::resolver::{DummyResolver, Resolver};

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    let resolver: Box<dyn Resolver> = Box::new(DummyResolver);

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let request = Message::from_bytes(&buf[0..size]);

                let reply = match request.header.opcode {
                    Opcode::Query => {
                        let answers: Vec<_> = request
                            .questions
                            .iter()
                            .map(|q| resolver.resolve(q))
                            .collect();
                        request.reply(RCode::NoError, answers)
                    }
                    Opcode::IQuery | Opcode::Status | Opcode::Reserved(_) => {
                        request.reply(RCode::NotImplemented, Vec::new())
                    }
                };

                udp_socket
                    .send_to(&reply.as_bytes(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
