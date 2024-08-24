/// Ref: https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml

pub const DNS_RECORD_TYPE_RESERVED_0: u16 = 0;
pub const DNS_RECORD_TYPE_A: u16 = 1;
pub const DNS_RECORD_TYPE_NS: u16 = 2;
pub const DNS_RECORD_TYPE_MD: u16 = 3;
pub const DNS_RECORD_TYPE_MF: u16 = 4;
pub const DNS_RECORD_TYPE_CNAME: u16 = 5;
pub const DNS_RECORD_TYPE_SOA: u16 = 6;
pub const DNS_RECORD_TYPE_MB: u16 = 7;
pub const DNS_RECORD_TYPE_MG: u16 = 8;
pub const DNS_RECORD_TYPE_MR: u16 = 9;
pub const DNS_RECORD_TYPE_NULL: u16 = 10;
pub const DNS_RECORD_TYPE_WKS: u16 = 11;
pub const DNS_RECORD_TYPE_PTR: u16 = 12;
pub const DNS_RECORD_TYPE_HINFO: u16 = 13;
pub const DNS_RECORD_TYPE_MINFO: u16 = 14;
pub const DNS_RECORD_TYPE_MX: u16 = 15;
pub const DNS_RECORD_TYPE_TXT: u16 = 16;
pub const DNS_RECORD_TYPE_RP: u16 = 17;
pub const DNS_RECORD_TYPE_AFSDB: u16 = 18;
pub const DNS_RECORD_TYPE_X25: u16 = 19;
pub const DNS_RECORD_TYPE_ISDN: u16 = 20;

pub const DNS_CLASS_RESERVED_0: u16 = 0;
pub const DNS_CLASS_IN: u16 = 1;
pub const DNS_CLASS_UNASSIGNED_2: u16 = 2;
pub const DNS_CLASS_CH: u16 = 3;
pub const DNS_CLASS_HS: u16 = 4;
pub const DNS_CLASS_QCLASS_NONE: u16 = 254;
pub const DNS_CLASS_QCLASS_ANY: u16 = 255;
pub const DNS_CLASS_RESERVED_PRIVATE_USE_START: u16 = 65280;
pub const DNS_CLASS_RESERVED_PRIVATE_USE_END: u16 = 65534;
pub const DNS_CLASS_RESERVED_65535: u16 = 65535;

pub const HEADER_SIZE_BYTES: usize = 12;
pub const MIN_QUESTION_SIZE_BYTES: usize = 4;
