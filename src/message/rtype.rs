use crate::int_enum;


int_enum! {
    /// TYPE fields are used in resource records.
    /// Note that these types are a subset of QTYPEs.
    ///
    /// QTYPE fields appear in the question part of a query.
    /// QTYPES are a superset of TYPEs, hence all TYPEs are valid QTYPEs.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    QType(u16) {
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
}
