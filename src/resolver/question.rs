use std::io::{Error, ErrorKind};

use super::constants::MIN_QUESTION_SIZE_BYTES;

#[derive(Debug)]
pub struct Question {
    pub qname: Vec<u8>,
    pub qtype: u16,
    pub qclass: u16,
}

impl Question {
    /// Converts the struct into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.qname.len() + 2 * std::mem::size_of::<u16>());

        bytes.extend(&self.qname);
        bytes.extend(&self.qtype.to_be_bytes());
        bytes.extend(&self.qclass.to_be_bytes());

        bytes
    }

    /// Encodes a domain name into its DNS representation format.
    pub fn encode_name(domain_name: &str) -> Vec<u8> {
        let mut bytes = Vec::new();
        let parts: Vec<&str> = domain_name.split('.').collect();
        for part in parts {
            bytes.push(part.len() as u8);
            bytes.extend(part.as_bytes());
        }
        bytes.push(0);
        bytes
    }

    /// Decodes a domain name represented in a DNS packet.
    /// Compression is taken care of internally.
    ///
    /// ### Parameters
    /// - `bytes`: Raw bytes of your DNS packet.
    /// - `cursor`: Where do we begin reading the packet from.
    ///
    /// ### Returns
    /// - The decoded name as `String`.
    /// - Cursor value for next use: last read index + 1.
    pub fn decode_name(bytes: &[u8], mut cursor: usize) -> Result<(String, usize), Error> {
        let mut parts = Vec::new();

        while cursor < bytes.len() {
            let length = bytes[cursor] as usize;

            // Skip the byte containing the length.
            cursor += 1;

            // The maximum length of a component of a DNS name is 63 characters.
            // So in a normal DNS name part the top 2 bits will never be set.
            // If it is, we know it is compressed.
            if length & 0b1100_0000 != 0 {
                let (compressed_name, updated_cursor) =
                    Question::decode_name_compressed(bytes, length, cursor)?;
                parts.push(compressed_name);
                cursor = updated_cursor;
                break;
            }

            if length == 0 {
                break;
            }

            if cursor + length > bytes.len() {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "name has invalid length, could have led to out of bounds access",
                ));
            }

            parts.push(String::from_utf8_lossy(&bytes[cursor..cursor + length]).to_string());
            cursor += length;
        }

        Ok((parts.join("."), cursor))
    }

    fn decode_name_compressed(
        bytes: &[u8],
        length: usize,
        mut cursor: usize,
    ) -> Result<(String, usize), Error> {
        if cursor >= bytes.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "name has invalid length, could have led to out of bounds access",
            ));
        }

        // Extracts a 16-bit pointer from the DNS message:
        //
        // In DNS compression, a pointer is indicated by a length byte where the high 2 bits are set to 11.
        // The remaining lower 6 bits of the length byte and the next byte together form the 16-bit pointer.
        //
        // 1. Mask the length byte to get the lower 6 bits of the pointer (removing the high 2 bits).
        // 2. Shift these 6 bits left by 8 positions to align them as the upper 8 bits of the 16-bit pointer.
        // 3. Combine these upper 8 bits with the next byte, which contains the lower 8 bits of the pointer,
        //    to get the complete 16-bit pointer value.
        // 4. Ensure that the next byte is safely accessed; if out of bounds, an error is returned.
        let pointer = ((length & 0b0011_1111) as usize) << 8
            | (*bytes
                .get(cursor)
                .ok_or_else(|| Error::new(ErrorKind::InvalidData, "pointer byte missing"))?
                as usize);

        // Skip past the second pointer byte.
        cursor += 1;

        // Decode the name at the pointer
        let (referenced_name, _) = Question::decode_name(bytes, pointer)?;

        Ok((referenced_name, cursor))
    }

    /// Constructs a `Question` represented in a DNS packet.
    /// ### Parameters
    /// - `bytes`: Raw bytes of your DNS packet.
    /// - `cursor`: Where do we begin reading the packet from.
    ///
    /// ### Returns
    /// - The question as `Question`.
    /// - Cursor value for next use: last read index + 1.
    pub fn from_bytes(bytes: &[u8], cursor: usize) -> Result<(Question, usize), Error> {
        let (qname, mut cursor) = Question::decode_name(bytes, cursor)?;
        // Parse qtype and qclass
        if cursor + MIN_QUESTION_SIZE_BYTES > bytes.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "not enough bytes for qtype and qclass",
            ));
        }

        let qtype = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;

        let qclass = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;

        Ok((
            Question {
                qname: qname.as_bytes().to_vec(),
                qtype,
                qclass,
            },
            cursor,
        ))
    }
}
