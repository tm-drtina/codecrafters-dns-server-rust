use crate::message::{QType, QClass, Name};

/// The question section is used to carry the "question" in most queries, i.e., the parameters that define what is being asked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    /// an owner name, i.e., the name of the node to which this resource record pertains.
    pub name: Name,
    /// two octets containing one of the RR TYPE codes
    pub rtype: QType,
    /// two octets containing one of the RR CLASS codes.
    pub rclass: QClass,
    /// a 32 bit signed integer that specifies the time interval that the resource record may be cached before the source of the information should again be consulted.
    /// Zero values are interpreted to mean that the RR can only be used for the transaction in progress, and should not be cached.
    /// For example, SOA records are always distributed with a zero TTL to prohibit caching.
    /// Zero values can also be used for extremely volatile data.
    pub ttl: i32,
    /// an unsigned 16 bit integer that specifies the length in octets of the RDATA field.
    pub rdlength: u16,
    /// a variable length string of octets that describes the resource.
    /// The format of this information varies according to the TYPE and CLASS of the resource record.
    pub rdata: Vec<u8>,
}

impl Answer {
    pub fn write(&self, buf: &mut Vec<u8>) {
        //   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                                               |
        // /                                               /
        // /                      NAME                     /
        // |                                               |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                      TYPE                     |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                     CLASS                     |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                      TTL                      |
        // |                                               |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                   RDLENGTH                    |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
        // /                     RDATA                     /
        // /                                               /
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        self.name.write(buf);

        buf.extend_from_slice(&self.rtype.value().to_be_bytes());
        buf.extend_from_slice(&self.rclass.value().to_be_bytes());
        buf.extend_from_slice(&self.ttl.to_be_bytes());
        buf.extend_from_slice(&self.rdlength.to_be_bytes());
        buf.extend(self.rdata.iter());
    }

    pub fn read(buf: &mut &[u8]) -> Self {
        let name = Name::read(buf);

        let rtype = QType::from_value(u16::from_be_bytes([buf[0], buf[1]]));
        let rclass = QClass::from_value(u16::from_be_bytes([buf[2], buf[3]]));
        let ttl = i32::from_be_bytes([buf[4], buf[5], buf[6], buf[6]]);
        let rdlength = u16::from_be_bytes([buf[7], buf[8]]);
        let rdata = buf[9..9 + (rdlength as usize)].to_vec();

        *buf = &buf[9 + (rdlength as usize)..];

        Self { name, rtype, rclass, ttl, rdlength, rdata }
    }
}
