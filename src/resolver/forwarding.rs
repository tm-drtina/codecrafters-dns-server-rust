use anyhow::{Context, Result};
use std::net::UdpSocket;

use crate::message::{Answer, Header, Message, Opcode, Question, RCode};

use super::Resolver;

pub struct ForwardingResolver(UdpSocket);

impl ForwardingResolver {
    pub fn new(addr: &str) -> Result<Self> {
        let socket = UdpSocket::bind("127.0.0.1:0").context("Cannot bind socket for forwarding")?;
        socket
            .connect(addr)
            .context("Failed to connect socket to given addr")?;
        Ok(Self(socket))
    }
}

impl Resolver for ForwardingResolver {
    fn resolve(&self, question: &Question, msg: &[u8]) -> Result<Vec<Answer>> {
        let query = Message {
            header: Header {
                id: 0,
                is_reply: false,
                opcode: Opcode::Query,
                authoritative: false,
                truncation: false,
                recursion_desired: false,
                recursion_available: false,
                rcode: RCode::NoError,
                question_count: 1,
                answer_count: 0,
                authority_count: 0,
                additional_count: 0,
            },
            questions: vec![question.with_resolved_name(msg)],
            answers: Vec::new(),
        };
        self.0
            .send(&query.as_bytes())
            .context("Failed to send forwarding query")?;

        let mut buf = [0; 512];
        let size = self
            .0
            .recv(&mut buf)
            .context("Failed to receive data from forwading server")?;

        let response = Message::from_bytes(&buf[0..size]);
        Ok(response.answers)
    }
}
