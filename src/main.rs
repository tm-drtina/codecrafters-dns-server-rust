use anyhow::{Result, Context, bail, ensure};
use dns_starter_rust::message::*;
use dns_starter_rust::resolver::{DummyResolver, Resolver, ForwardingResolver};
use std::env;
use std::net::UdpSocket;

fn create_resolver() -> Result<Box<dyn Resolver>> {
    let mut args = env::args();
    args.next().context("Expected first arg (path of executable)")?;

    Ok(match args.len() {
        0 => Box::new(DummyResolver),
        2 => {
            let key = args.next().unwrap();
            let value = args.next().unwrap();
            ensure!(key == "--resolver", "Unrecognized argument. Expected '--resolver'");
            Box::new(ForwardingResolver::new(&value)?)
        }
        _ => {
            bail!("Invalid number of arguments")
        }
    })
}

fn main() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").context("Failed to bind to address")?;
    let mut buf = [0; 512];
    let resolver = create_resolver()?;

    loop {
        let (size, source) = udp_socket.recv_from(&mut buf)?;
        let msg_bytes = &buf[0..size];
        let request = Message::from_bytes(msg_bytes);

        let reply = match request.header.opcode {
            Opcode::Query => {
                let mut answers = Vec::new();
                for question in &request.questions {
                    answers.extend(resolver.resolve(question, msg_bytes)?)
                }
                request.reply(RCode::NoError, answers)
            }
            Opcode::IQuery | Opcode::Status | Opcode::Reserved(_) => {
                request.reply(RCode::NotImplemented, Vec::new())
            }
        };

        udp_socket
            .send_to(&reply.as_bytes(), source)
            .context("Failed to send response")?;
    }
}
