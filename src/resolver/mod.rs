pub mod constants;
pub mod header;
pub mod packet;
pub mod question;
pub mod record;

use std::fmt::Write;
use std::io::{Error, ErrorKind};
use std::net::UdpSocket;

pub struct Resolver {}

impl Resolver {
    fn build_query(domain_name: &str, record_type: u16) -> Vec<u8> {
        let mut query = Vec::new();

        let encoded_name = Question::encode_name(&domain_name);
        let id: u16 = 1;
        let recursion_desired: u16 = 0;
        let header = Header {
            id,
            flags: recursion_desired,
            num_questions: 1,
            num_additionals: 0,
            num_authorities: 0,
            num_answers: 0,
        };

        let question = Question {
            qname: encoded_name,
            qtype: record_type,
            qclass: DNS_CLASS_IN,
        };

        query.extend(header.to_bytes());
        query.extend(question.to_bytes());

        query
    }

    fn parse_ip(data: &[u8]) -> Option<String> {
        let mut ip = String::new();

        for b in &data[0..3] {
            if write!(&mut ip, "{}.", b).is_err() {
                return None;
            }
        }

        if write!(&mut ip, "{}", data[3]).is_err() {
            return None;
        }

        Some(ip)
    }

    /// Resolves the given domain.
    fn send_query(ip: &str, domain_name: &str, record_type: u16) -> Result<Packet, Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        let query = Resolver::build_query(domain_name, record_type);

        let _ = socket.send_to(&query, ip.to_owned() + ":53");

        let mut response_buffer = [0; MAX_DNS_PACKET_BYTES];

        let (_, _) = socket.recv_from(&mut response_buffer)?;

        let packet = Packet::from_bytes(&response_buffer)?;

        Ok(packet)
    }

    pub fn resolve_dbg(domain_name: String) -> Result<(), std::io::Error> {
        let packet = Resolver::send_query("8.8.8.8", &domain_name, DNS_RECORD_TYPE_A)?;
        packet.print();
        Ok(())
    }

    pub fn resolve(domain_name: &str, record_type: u16) -> Result<String, Error> {
        let mut nameserver = "198.41.0.4".to_string();
        let domain_name = domain_name.trim_end_matches('.');
        let record_type = record_type;

        loop {
            println!(">> asking {} for {}", nameserver, domain_name);
            let packet = Resolver::send_query(&nameserver, domain_name, record_type)?;

            if let Some(ip) = Resolver::get_answer(&packet) {
                return Ok(ip);
            } else if let Some(ns_ip) = Resolver::get_nameserver_ip(&packet) {
                nameserver = ns_ip;
            } else if let Some(ns_domain) = Resolver::get_nameserver(&packet) {
                nameserver = Resolver::resolve(&ns_domain, DNS_RECORD_TYPE_A)?;
            } else {
                println!(">> not found, ending!");
                return Err(Error::new(ErrorKind::NotFound, "domain not found"));
            }
        }
    }

    fn get_answer(packet: &Packet) -> Option<String> {
        packet
            .answers
            .iter()
            .find(|ans| ans.record_type == DNS_RECORD_TYPE_A)
            .and_then(|answer| Resolver::parse_ip(&answer.data))
    }

    fn get_nameserver_ip(packet: &Packet) -> Option<String> {
        packet
            .additionals
            .iter()
            .find(|add| add.record_type == DNS_RECORD_TYPE_A)
            .and_then(|additional| Resolver::parse_ip(&additional.data))
    }

    fn get_nameserver(packet: &Packet) -> Option<String> {
        packet
            .authorities
            .iter()
            .find(|auth| auth.record_type == DNS_RECORD_TYPE_NS && !auth.data.is_empty())
            .map(|authority| String::from_utf8_lossy(&authority.data).to_string())
    }
}

pub use constants::*;
pub use header::Header;
pub use packet::Packet;
pub use question::Question;
pub use record::Record;
