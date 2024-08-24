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
}
