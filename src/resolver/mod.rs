pub mod constants;
pub mod header;
pub mod packet;
pub mod question;
pub mod record;

pub struct Resolver {}

impl Resolver {
    fn build_query(domain_name: String, record_type: u16) -> Vec<u8> {
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

    pub fn run(domain_name: String) -> Result<(), std::io::Error> {
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(sock) => sock,
            Err(err) => {
                eprintln!("failed to bind udp socket: {}", err);
                return Err(err);
            }
        };

        let query = Resolver::build_query(domain_name, DNS_RECORD_TYPE_A);

        let _ = match socket.send_to(&query, "8.8.8.8:53") {
            Ok(_res) => {}
            Err(err) => {
                eprintln!("failed to send udp data: {}", err);
                return Err(err);
            }
        };

        let mut response_buffer = [0; 512];
        let _response = match socket.recv_from(&mut response_buffer) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("failed to receive udp data: {}", err);
                return Err(err);
            }
        };

        println!("{:?}", Packet::from_bytes(&response_buffer));

        Ok(())
    }
}

use std::net::UdpSocket;

pub use constants::*;
pub use header::Header;
pub use packet::Packet;
pub use question::Question;
pub use record::Record;
