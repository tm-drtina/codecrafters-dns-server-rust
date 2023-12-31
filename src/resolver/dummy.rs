use crate::message::{Answer, Question};
use anyhow::Result;

use super::Resolver;

pub struct DummyResolver;

impl Resolver for DummyResolver {
    fn resolve(&self, question: &Question, _msg: &[u8]) -> Result<Vec<Answer>> {
        Ok(vec![Answer {
            name: question.qname.clone(),
            rtype: question.qtype,
            rclass: question.qclass,
            ttl: 60,
            rdlength: 4,
            rdata: vec![0x08, 0x08, 0x08, 0x08],
        }])
    }
}
