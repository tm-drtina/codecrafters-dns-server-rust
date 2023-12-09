/// TYPE fields are used in resource records.
/// Note that these types are a subset of QTYPEs.
///
/// QTYPE fields appear in the question part of a query.
/// QTYPES are a superset of TYPEs, hence all TYPEs are valid QTYPEs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QType {
    /// a host address
    A = 1,
    /// an authoritative name server
    NS = 2,
    /// a mail destination (Obsolete - use MX)
    MD = 3,
    /// a mail forwarder (Obsolete - use MX)
    MF = 4,
    /// the canonical name for an alias
    CNAME = 5,
    /// marks the start of a zone of authority
    SOA = 6,
    /// a mailbox domain name (EXPERIMENTAL)
    MB = 7,
    /// a mail group member (EXPERIMENTAL)
    MG = 8,
    /// a mail rename domain name (EXPERIMENTAL)
    MR = 9,
    /// a null RR (EXPERIMENTAL)
    NULL = 10,
    /// a well known service description
    WKS = 11,
    /// a domain name pointer
    PTR = 12,
    /// host information
    HINFO = 13,
    /// mailbox or mail list information
    MINFO = 14,
    /// mail exchange
    MX = 15,
    /// text strings
    TXT = 16,

    // QTYPE specific
    /// A request for a transfer of an entire zone
    AXFR = 252,
    /// A request for mailbox-related records (MB, MG or MR)
    MAILB = 253,
    ///  A request for mail agent RRs (Obsolete - see MX)
    MAILA = 254,
    /// A request for all records
    ANY = 255,
}

/// CLASS fields appear in resource records.
///
/// QCLASS fields appear in the question section of a query.
/// QCLASS values are a superset of CLASS values; every CLASS is a valid QCLASS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QClass {
    /// the Internet
    IN = 1,
    /// the CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS = 2,
    /// the CHAOS class
    CH = 3,
    ///  Hesiod [Dyer 87]
    HS = 4,

    // QCLASS specific
    /// any class
    Any = 255,
}

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

        buf.extend_from_slice(&(self.qtype as u16).to_be_bytes());
        buf.extend_from_slice(&(self.qclass as u16).to_be_bytes());
    }
}
