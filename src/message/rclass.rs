/// CLASS fields appear in resource records.
///
/// QCLASS fields appear in the question section of a query.
/// QCLASS values are a superset of CLASS values; every CLASS is a valid QCLASS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
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

impl QClass {
    pub const fn to_bytes(&self) -> [u8; 2] {
        (*self as u16).to_be_bytes()
    }
}
