use crate::message::{Answer, Question};
use anyhow::Result;

mod dummy;
pub use dummy::DummyResolver;

mod forwarding;
pub use forwarding::ForwardingResolver;

pub trait Resolver {
    fn resolve(&self, question: &Question, msg: &[u8]) -> Result<Vec<Answer>>;
}
