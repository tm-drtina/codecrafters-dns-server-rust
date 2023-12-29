mod answer;
mod header;
mod name;
mod question;
mod rclass;
mod rtype;

pub use answer::*;
pub use header::*;
pub use question::*;
pub use name::*;
pub use rclass::*;
pub use rtype::*;

#[derive(Debug)]
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

    pub fn from_bytes(buf: &[u8]) -> Self {
        let (header, mut body) = buf.split_at(12);
        let header = Header::from_bytes(header);

        let mut questions = Vec::with_capacity(header.question_count as usize);
        for _ in 0..header.question_count {
            questions.push(Question::read(&mut body));
        }

        let mut answers = Vec::with_capacity(header.answer_count as usize);
        for _ in 0..header.answer_count {
            answers.push(Answer::read(&mut body));
        }

        Self { header, questions, answers }
    }
}
