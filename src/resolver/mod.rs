pub mod constants;
pub mod header;
pub mod packet;
pub mod question;
pub mod record;

use std::io::Error;
use std::net::UdpSocket;

pub struct Resolver {}

impl Resolver {
    fn build_query(domain_name: &str, record_type: u16) -> Vec<u8> {
        let mut query = Vec::new();

        let encoded_name = Question::encode_name(&domain_name);
        let id: u16 = 1;
        let recursion_desired: u16 = 1 << 8;
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

    /// Resolves the given domain.
    fn send_query(ip: &str, domain_name: &str, record_type: u16) -> Result<Packet, Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        let query = Resolver::build_query(domain_name, record_type);

        let _ = socket.send_to(&query, ip.to_owned() + ":53");

        let mut response_buffer = [0; MAX_DNS_PACKET_BYTES];

        println!("{:?}", Packet::from_bytes(&response_buffer));

        let (_, _) = socket.recv_from(&mut response_buffer)?;

        let packet = Packet::from_bytes(&response_buffer)?;
        Ok(packet)
    }

    pub fn resolve(domain_name: String) -> Result<String, std::io::Error> {
        let hostname = domain_name.trim_end_matches(".");
        let mut ip: &str = "198.41.0.4";
        let record_type = DNS_RECORD_TYPE_A;

        loop {
            println!(">> asking {} for {}", ip, hostname);
            let previous_ip = ip;
            let packet = Resolver::send_query(&ip, &domain_name, record_type)?;

            // Check for a direct answer
            if let Some(_answer) = packet
                .answers
                .iter()
                .find(|ans| ans.record_type == DNS_RECORD_TYPE_A)
            {
                return Ok("ip".to_string());
            }

            // Check for a nameserver with an A record
            if let Some(_add) = packet
                .additionals
                .iter()
                .find(|add| add.record_type == DNS_RECORD_TYPE_A)
            {
                ip = "";
                continue;
            }

            // Get the nameserver's IP
            if let Some(auth) = packet
                .authorities
                .iter()
                .find(|auth| auth.record_type == DNS_RECORD_TYPE_NS && !auth.data.is_empty())
            {
                let ip = &Resolver::resolve(String::from_utf8_lossy(&auth.data).to_string())?;
                if ip == previous_ip {
                    println!(">> not found, ending!");
                    break;
                }
            }
        }

        Ok("not found".to_string())
    }
}

pub use constants::*;
pub use header::Header;
pub use packet::Packet;
pub use question::Question;
pub use record::Record;
