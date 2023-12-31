use crate::message::{Question, Answer};

mod dummy;
pub use dummy::DummyResolver;

pub trait Resolver {
    fn resolve(&self, question: &Question) -> Answer;
}
