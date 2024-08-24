use std::net::UdpSocket;

const DNS_RECORD_TYPE_A: u16 = 0;
const DNS_RECORD_TYPE_AAAA: u16 = 16;
const DNS_RECORD_TYPE_CNAME: u16 = 5;
const DNS_RECORD_TYPE_MX: u16 = 15;
const DNS_RECORD_TYPE_NS: u16 = 2;
const DNS_RECORD_TYPE_PTR: u16 = 12;
const DNS_RECORD_TYPE_SOA: u16 = 6;
const DNS_RECORD_TYPE_TXT: u16 = 16;
const DNS_RECORD_TYPE_SRV: u16 = 33;

const DNS_CLASS_IN: u16 = 1;
const DNS_CLASS_CH: u16 = 3;
const DNS_CLASS_HS: u16 = 4;

#[derive(Debug)]
struct DnsHeader {
    id: u16,
    flags: u16,
    num_questions: u16,
    num_answers: u16,
    num_authorities: u16,
    num_additionals: u16,
}

impl DnsHeader {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.id.to_be_bytes());
        bytes.extend(&self.flags.to_be_bytes());
        bytes.extend(&self.num_questions.to_be_bytes());
        bytes.extend(&self.num_answers.to_be_bytes());
        bytes.extend(&self.num_authorities.to_be_bytes());
        bytes.extend(&self.num_additionals.to_be_bytes());
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 12 {
            // Not enough bytes for a DNS header
            return None;
        }

        Some(DnsHeader {
            id: u16::from_be_bytes([bytes[0], bytes[1]]),
            flags: u16::from_be_bytes([bytes[2], bytes[3]]),
            num_questions: u16::from_be_bytes([bytes[4], bytes[5]]),
            num_answers: u16::from_be_bytes([bytes[6], bytes[7]]),
            num_authorities: u16::from_be_bytes([bytes[8], bytes[9]]),
            num_additionals: u16::from_be_bytes([bytes[10], bytes[11]]),
        })
    }
}

#[derive(Debug)]
struct DnsQuestion {
    qname: Vec<u8>,
    qtype: u16,
    qclass: u16,
}

#[derive(Debug)]
struct DnsRecord {
    name: Vec<u8>,
    record_type: u16,
    class_type: u16,
    ttl: u32,
    data: Vec<u8>,
}

impl DnsRecord {
    fn from_bytes(bytes: &[u8], mut cursor: usize) -> Option<(DnsRecord, usize)> {
        // Parse the name
        let mut name = Vec::new();
        while cursor < bytes.len() {
            let len = bytes[cursor] as usize;
            cursor += 1;

            if len == 0 {
                break; // End of name
            }

            if cursor + len > bytes.len() {
                return None; // Out of bounds
            }

            name.extend_from_slice(&bytes[cursor..cursor + len]);
            cursor += len;
        }

        // Ensure name parsing was successful
        if name.is_empty() {
            return None;
        }

        // Parse record_type (2 bytes)
        if cursor + 2 > bytes.len() {
            return None; // Out of bounds
        }
        let record_type = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;

        // Parse class_type (2 bytes)
        if cursor + 2 > bytes.len() {
            return None; // Out of bounds
        }
        let class_type = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;

        // Parse ttl (4 bytes)
        if cursor + 4 > bytes.len() {
            return None; // Out of bounds
        }
        let ttl = u32::from_be_bytes([bytes[cursor], bytes[cursor + 1], bytes[cursor + 2], bytes[cursor + 3]]);
        cursor += 4;

        // Parse data length (2 bytes)
        if cursor + 2 > bytes.len() {
            return None; // Out of bounds
        }
        let data_len = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;

        // Parse data
        if cursor + data_len as usize > bytes.len() {
            return None; // Out of bounds
        }
        let data = bytes[cursor..cursor + data_len as usize].to_vec();
        cursor += data_len as usize;

        Some((DnsRecord {
            name,
            record_type,
            class_type,
            ttl,
            data,
        }, cursor))
    }

    fn next_n_from_bytes(n: u16, bytes: &[u8], mut cursor: usize) -> (Option<Vec<DnsRecord>>, usize) {
        let mut records = Vec::new();
        for _i in 0..n {
            let (record, updated_cursor) = DnsRecord::from_bytes(bytes, cursor).unwrap();
            records.push(record);
            cursor = updated_cursor;
        }
        (Some(records), cursor)
    }
}

impl DnsQuestion {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.qname);
        bytes.extend(&self.qtype.to_be_bytes());
        bytes.extend(&self.qclass.to_be_bytes());
        bytes
    }

    fn encode_name(domain_name: String) -> Vec<u8> {
        let mut bytes = Vec::new();
        let parts: Vec<&str> = domain_name.split('.').collect();
        for part in parts {
            bytes.push(part.len() as u8);
            bytes.extend(part.as_bytes());
        }
        bytes.push(0);
        bytes
    }

    fn from_bytes(bytes: &[u8], mut cursor: usize) -> (Option<DnsQuestion>, usize) {
        // Parse the domain name
        let mut qname = Vec::new();
        while cursor < bytes.len() {
            let len = bytes[cursor] as usize;
            cursor += 1;

            if len == 0 {
                break;
            }

            // Read the label
            if cursor + len > bytes.len() {
                return (None, cursor); // Not enough bytes for the label
            }
            qname.extend_from_slice(&bytes[cursor..cursor + len]);
            cursor += len;
        }

        // Ensure the domain name parsing was successful
        if qname.is_empty() {
            return (None, cursor); // Domain name should not be empty
        }

        // Parse qtype and qclass
        if cursor + 4 > bytes.len() {
            return (None, cursor); // Not enough bytes for qtype and qclass
        }

        let qtype = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;

        let qclass = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;

        (Some(DnsQuestion {
            qname,
            qtype,
            qclass,
        }), cursor)
    }
}

#[derive(Debug)]
struct Packet {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsRecord>,
    authorities: Vec<DnsRecord>,
    additionals: Vec<DnsRecord>,
}

impl Packet {
    fn from_bytes(bytes: &[u8]) -> Packet {
        let header = DnsHeader::from_bytes(&bytes).unwrap();
        let mut cursor = 12;
        let mut questions = Vec::new();
        for _i in 0..header.num_questions {
            let (question, updated_cursor) = DnsQuestion::from_bytes(&bytes, cursor);
            questions.push(question.unwrap());
            cursor = updated_cursor;
        }
        let (answers, cursor) = DnsRecord::next_n_from_bytes(header.num_answers, &bytes, cursor);
        let (authorities, cursor) = DnsRecord::next_n_from_bytes(header.num_authorities, &bytes, cursor);
        let (additionals, _cursor) = DnsRecord::next_n_from_bytes(header.num_additionals, &bytes, cursor);

        Packet {
            header,
            questions: questions,
            answers: answers.unwrap(),
            authorities: authorities.unwrap(),
            additionals: additionals.unwrap(),
        }
    }
}

fn build_query(domain_name: String, record_type: u16) -> Vec<u8> {
    let mut query = Vec::new();

    let encoded_name = DnsQuestion::encode_name(domain_name);
    let id: u16 = 1;
    let recursion_desired: u16 = 1 << 8;
    let header = DnsHeader {
        id,
        flags: recursion_desired,
        num_questions: 1,
        num_additionals: 0,
        num_authorities: 0,
        num_answers: 0,
    };

    let question = DnsQuestion {
        qname: encoded_name,
        qtype: record_type,
        qclass: DNS_CLASS_IN,
    };

    query.extend(header.to_bytes());
    query.extend(question.to_bytes());

    query
}

fn run(domain_name: String) -> Result<(), std::io::Error> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(sock) => sock,
        Err(err) => {
            eprintln!("failed to bind udp socket: {}", err);
            return Err(err);
        }
    };

    let query = build_query(domain_name, DNS_RECORD_TYPE_A);

    let _ = match socket.send_to(&query, "8.8.8.8:53") {
        Ok(_res) => {},
        Err(err) => {
            eprintln!("failed to send udp data: {}", err);
            return Err(err);
        }
    };

    let mut response_buffer = [0; 512];
    let _response = match socket.recv_from(&mut response_buffer) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("failed to receive udp data: {}", err);
            return Err(err);
        }
    };

    println!("{:?}", Packet::from_bytes(&response_buffer));

    Ok(())
}

fn main() {
    run("vivekn.dev".to_string()).unwrap();
}
