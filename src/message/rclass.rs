use crate::int_enum;


int_enum! {
    /// CLASS fields appear in resource records.
    ///
    /// QCLASS fields appear in the question section of a query.
    /// QCLASS values are a superset of CLASS values; every CLASS is a valid QCLASS.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    QClass(u16) {
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
}
