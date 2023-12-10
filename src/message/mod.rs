mod answer;
mod header;
mod question;
mod rclass;
mod rtype;

pub use answer::*;
pub use header::*;
pub use question::*;
pub use rclass::*;
pub use rtype::*;

pub struct Message {
    pub header: header::Header,
    pub questions: Vec<question::Question>,
    pub answers: Vec<answer::Answer>,
}

impl Message {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut write_buf = Vec::with_capacity(512);
        self.header.write(&mut write_buf);
        for q in &self.questions {
            q.write(&mut write_buf);
        }
        for a in &self.answers {
            a.write(&mut write_buf);
        }
        write_buf
    }
}
