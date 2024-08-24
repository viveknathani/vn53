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
}

struct DnsQuestion {
    qname: Vec<u8>,
    qtype: u16,
    qclass: u16,
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
        qtype: DNS_RECORD_TYPE_A,
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

    println!("{:x?}", response_buffer);

    Ok(())
}

fn main() {
    run("vivekn.dev".to_string()).unwrap();
}
