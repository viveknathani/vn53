use super::{constants::MIN_RECORD_SIZE_BYTES, Question};
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Record {
    pub name: Vec<u8>,
    pub record_type: u16,
    pub class_type: u16,
    pub ttl: u32,
    pub data: Vec<u8>,
}

impl Record {
    /// Constructs a `Record` represented in a DNS packet.
    /// ### Parameters
    /// - `bytes`: Raw bytes of your DNS packet.
    /// - `cursor`: Where do we begin reading the packet from.
    ///
    /// ### Returns
    /// - The record as `Record`.
    /// - Cursor value for next use: last read index + 1.
    pub fn from_bytes(bytes: &[u8], cursor: usize) -> Result<(Record, usize), Error> {
        let (qname, mut cursor) = Question::decode_name(bytes, cursor)?;

        if cursor + MIN_RECORD_SIZE_BYTES > bytes.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "not enough bytes for a record",
            ));
        }

        let record_type = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;

        let class_type = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;

        let ttl = u32::from_be_bytes([
            bytes[cursor],
            bytes[cursor + 1],
            bytes[cursor + 2],
            bytes[cursor + 3],
        ]);
        cursor += 4;

        let data_len = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;

        let data = bytes[cursor..cursor + data_len as usize].to_vec();
        cursor += data_len as usize;

        Ok((
            Record {
                name: qname.as_bytes().to_vec(),
                record_type,
                class_type,
                ttl,
                data,
            },
            cursor,
        ))
    }

    /// Constructs a vector of records from the given DNS packet.
    /// It cannot know where to stop, hence it requires `n`.
    /// ### Parameters
    /// - `n`: How many bytes do we read.
    /// - `bytes`: Raw bytes of your DNS packet.
    /// - `cursor`: Where do we begin reading the packet from.
    ///
    /// ### Returns
    /// - The record as `Record`.
    /// - Cursor value for next use: last read index + 1.
    pub fn next_n_from_bytes(
        n: u16,
        bytes: &[u8],
        mut cursor: usize,
    ) -> Result<(Vec<Record>, usize), Error> {
        let mut records = Vec::new();
        for _i in 0..n {
            let (record, updated_cursor) = Record::from_bytes(bytes, cursor)?;
            records.push(record);
            cursor = updated_cursor;
        }
        Ok((records, cursor))
    }
}
