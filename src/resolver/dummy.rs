use crate::message::{Question, Answer};

use super::Resolver;

pub struct DummyResolver;

impl Resolver for DummyResolver {
    fn resolve(&self, question: &Question) -> Answer {
        Answer {
            name: question.qname.clone(),
            rtype: question.qtype,
            rclass: question.qclass,
            ttl: 60,
            rdlength: 4,
            rdata: vec![0x08, 0x08, 0x08, 0x08],
        }
    }
}
