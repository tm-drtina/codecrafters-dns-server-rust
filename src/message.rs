/// A four bit field that specifies kind of query in this message.
/// This value is set by the originator of a query and copied into the response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /// a standard query
    QUERY = 0,
    /// an inverse query
    IQUERY = 1,
    /// a server status request
    STATUS = 2,
    //3-15 reserved for future use
}

/// Response code - this 4 bit field is set as part of responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RCode {
    /// No error condition
    NoError = 0,
    /// The name server was unable to interpret the query.
    FormatError = 1,
    /// The name server was unable to process this query due to a problem with the name server.
    ServerFailure = 2,
    /// Meaningful only for responses from an authoritative name server, this code signifies that the domain name referenced in the query does not exist.
    NameError = 3,
    /// The name server does not support the requested kind of query.
    NotImplemented = 4,
    /// The name server refuses to perform the specified operation for policy reasons.
    /// For example, a name server may not wish to provide the information to the particular requester, or a name server may not wish to perform a particular operation (e.g., zone transfer) for particular data.
    Refused = 5,
    // 6-15 Reserved for future use.
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// A 16 bit identifier assigned by the program that generates any kind of query.
    /// This identifier is copied the corresponding reply and can be used by the requester to match up replies to outstanding queries.
    pub id: u16,
    /// A one bit field that specifies whether this message is a query (0), or a response (1).
    pub is_reply: bool,
    /// A four bit field that specifies kind of query in this message.
    /// This value is set by the originator of a query and copied into the response.
    pub opcode: Opcode,
    /// Authoritative Answer - this bit is valid in responses, and specifies that the responding name server is an authority for the domain name in question section.
    /// 
    /// Note that the contents of the answer section may have multiple owner names because of aliases.
    /// The AA bit corresponds to the name which matches the query name, or the first owner name in the answer section.
    pub authoritative: bool,
    /// Truncation - specifies that this message was truncated due to length greater than that permitted on the transmission channel.
    /// 1 if the message is larger than 512 bytes. Always 0 in UDP responses.
    pub truncation: bool,
    /// Recursion Desired - this bit may be set in a query an is copied into the response.
    /// If RD is set, it directs the name server to pursue the query recursively.
    /// Recursive query support is optional.
    pub recursion_desired: bool,
    /// Recursion Available - this be is set or cleared in a response, and denotes whether recursive query support is available in the name server.
    pub recursion_available: bool,
    // Reserved (Z) 	3 bits 	Used by DNSSEC queries. At inception, it was reserved for future use.
    /// Response code - this 4 bit field is set as part of responses. 
    pub rcode: RCode,
    /// an unsigned 16 bit integer specifying the number of entries in the question section.
    pub question_count: u16,
    /// an unsigned 16 bit integer specifying the number of resource records in the answer section.
    pub answer_count: u16,
    /// an unsigned 16 bit integer specifying the number of name server resource records in the authority records section
    pub authority_count: u16,
    /// an unsigned 16 bit integer specifying the number of resource records in the additional records section.
    pub additional_count: u16,
}

impl Header {
    pub fn bytes(&self) -> [u8; 12] {
        //                                  1  1  1  1  1  1
        //    0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                      ID                       |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                    QDCOUNT                    |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                    ANCOUNT                    |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                    NSCOUNT                    |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        // |                    ARCOUNT                    |
        // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        let mut res = [0; 12];
        res[0..2].clone_from_slice(&self.id.to_be_bytes());

        if self.is_reply {
            res[2] |= 0b1000_0000;
        }
        res[2] |= (self.opcode as u8) << 3;
        if self.authoritative {
            res[2] |= 0b0000_0100;
        }
        if self.truncation {
            res[2] |= 0b0000_0010;
        }
        if self.recursion_desired {
            res[2] |= 0b0000_0001;
        }

        if self.recursion_available {
            res[3] |= 0b1000_0000;
        }
        res[3] |= self.rcode as u8;

        res[4..6].clone_from_slice(&self.question_count.to_be_bytes());
        res[6..8].clone_from_slice(&self.answer_count.to_be_bytes());
        res[8..10].clone_from_slice(&self.authority_count.to_be_bytes());
        res[10..12].clone_from_slice(&self.additional_count.to_be_bytes());
        
        res
    }
}
