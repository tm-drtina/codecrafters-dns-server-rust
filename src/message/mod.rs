mod header;
mod question;

pub use header::*;
pub use question::*;

pub struct Message {
    pub header: header::Header,
    pub questions: Vec<question::Question>,
}

impl Message {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut write_buf = Vec::with_capacity(512);
        self.header.write(&mut write_buf);
        for q in &self.questions {
            q.write(&mut write_buf);
        }
        write_buf
    }
}
