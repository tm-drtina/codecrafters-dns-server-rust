use crate::message::{QType, QClass};

/// The question section is used to carry the "question" in most queries, i.e., the parameters that define what is being asked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    /// a domain name represented as a sequence of labels, where each label consists of a length octet followed by that number of octets.
    /// The domain name terminates with the zero length octet for the null label of the root.
    /// Note that this field may be an odd number of octets; no padding is used.
    pub qname: Vec<Vec<u8>>,
    /// a two octet code which specifies the type of the query.
    /// The values for this field include all codes valid for a TYPE field, together with some more general codes which can match more than one type of RR.
    pub qtype: QType,
    /// a two octet code that specifies the class of the query.
    /// For example, the QCLASS field is IN for the Internet.
    pub qclass: QClass,
}

impl Question {
    pub fn write(&self, buf: &mut Vec<u8>) {
        //                                 1  1  1  1  1  1
        //   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                                               |
        // /                     QNAME                     /
        // /                                               /
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                     QTYPE                     |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                     QCLASS                    |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        for name_part in &self.qname {
            buf.push(name_part.len() as u8);
            buf.extend(name_part);
        }
        buf.push(0);

        buf.extend_from_slice(&self.qtype.value().to_be_bytes());
        buf.extend_from_slice(&self.qclass.value().to_be_bytes());
    }

    pub fn read(buf: &mut &[u8]) -> Self {
        let mut qname = Vec::new();
        loop {
            let len = buf[0] as usize;
            *buf = &buf[1..];
            if len == 0 {
                break;
            }
            qname.push(buf[0..len].to_vec());
            *buf = &buf[len..];
        }

        let qtype = QType::from_value(u16::from_be_bytes([buf[0], buf[1]]));
        let qclass = QClass::from_value(u16::from_be_bytes([buf[2], buf[3]]));
        *buf = &buf[4..];

        Self { qname, qtype, qclass }
    }
}
