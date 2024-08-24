use super::constants::HEADER_SIZE_BYTES;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub flags: u16,
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additionals: u16,
}

impl Header {
    /// Converts the `Header` struct to a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(&self.id.to_be_bytes());
        bytes.extend(&self.flags.to_be_bytes());
        bytes.extend(&self.num_questions.to_be_bytes());
        bytes.extend(&self.num_answers.to_be_bytes());
        bytes.extend(&self.num_authorities.to_be_bytes());
        bytes.extend(&self.num_additionals.to_be_bytes());

        bytes
    }

    /// Constructs a `Header` struct from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < HEADER_SIZE_BYTES {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "not enough bytes for a dns header",
            ));
        }

        Ok(Header {
            id: u16::from_be_bytes([bytes[0], bytes[1]]),
            flags: u16::from_be_bytes([bytes[2], bytes[3]]),
            num_questions: u16::from_be_bytes([bytes[4], bytes[5]]),
            num_answers: u16::from_be_bytes([bytes[6], bytes[7]]),
            num_authorities: u16::from_be_bytes([bytes[8], bytes[9]]),
            num_additionals: u16::from_be_bytes([bytes[10], bytes[11]]),
        })
    }
}
