use crate::resolver::Resolver;

use super::{constants::HEADER_SIZE_BYTES, Header, Question, Record};
use std::io::Error;

#[derive(Debug)]
pub struct Packet {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Record>,
    pub authorities: Vec<Record>,
    pub additionals: Vec<Record>,
}

impl Packet {
    /// Constructs a `Packet` from the raw DNS query response.
    pub fn from_bytes(bytes: &[u8]) -> Result<Packet, Error> {
        let header = Header::from_bytes(&bytes)?;
        let mut cursor = HEADER_SIZE_BYTES;

        let mut questions = Vec::new();
        for _i in 0..header.num_questions {
            let (question, updated_cursor) = Question::from_bytes(&bytes, cursor)?;
            questions.push(question);
            cursor = updated_cursor;
        }

        let (answers, cursor) = Record::next_n_from_bytes(header.num_answers, &bytes, cursor)?;
        let (authorities, cursor) =
            Record::next_n_from_bytes(header.num_authorities, &bytes, cursor)?;
        let (additionals, _) = Record::next_n_from_bytes(header.num_additionals, &bytes, cursor)?;

        Ok(Packet {
            header,
            questions,
            answers,
            authorities,
            additionals,
        })
    }

    pub fn print(&self) {
        println!("=== START OF DNS PACKET ===");
        println!("== HEADER ==");
        println!("id = {:?}", self.header.id);
        println!("flags = {:?}", self.header.flags);
        println!("num_questions = {:?}", self.header.num_questions);
        println!("num_answers = {:?}", self.header.num_answers);
        println!("num_authorities = {:?}", self.header.num_authorities);
        println!("num_additionals = {:?}", self.header.num_additionals);
        println!("== QUESTIONS ==");
        for question in &self.questions {
            println!("qname = {:?}", String::from_utf8_lossy(&question.qname));
            println!("qtype = {:?}", question.qtype);
            println!("qclass = {:?}", question.qclass);
        }
        println!("== ANSWERS ==");
        for record in &self.answers {
            println!("name = {:?}", String::from_utf8_lossy(&record.name));
            println!("record type = {:?}", record.record_type);
            println!("class type = {:?}", record.class_type);
            println!("ttl = {:?}", record.ttl);
            println!("data = {:?}", Resolver::parse_ip(&record.data).unwrap());
        }
        println!("== AUTHORITIES ==");
        for record in &self.authorities {
            println!("name = {:?}", String::from_utf8_lossy(&record.name));
            println!("record type = {:?}", record.record_type);
            println!("class type = {:?}", record.class_type);
            println!("ttl = {:?}", record.ttl);
            println!("data = {:?}", Resolver::parse_ip(&record.data).unwrap());
        }
        println!("== ADDITIONALS ==");
        for record in &self.additionals {
            println!("name = {:?}", String::from_utf8_lossy(&record.name));
            println!("record type = {:?}", record.record_type);
            println!("class type = {:?}", record.class_type);
            println!("ttl = {:?}", record.ttl);
            println!("data = {:?}", Resolver::parse_ip(&record.data).unwrap());
        }
        println!("=== END OF DNS PACKET ===");
    }
}
