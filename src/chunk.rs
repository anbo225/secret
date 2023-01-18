use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt::Display;

use crate::chunk_type::ChunkType;

#[derive(Debug)]
pub enum ChunkError {
    InvalidCrc(u32, u32),
    InvalidChunkDatLength(u32, u32),
    InvalidChunkType,
    InvalidUTF8DataString,
}

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn create_crc(chunk_type: &ChunkType, chunk_data: &Vec<u8>) -> u32 {
        let crc_struct = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        crc_struct.checksum(&[chunk_type.bytes().as_slice(), chunk_data.as_slice()].concat())
    }

    fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Chunk {
        let crc = Chunk::create_crc(&chunk_type, &chunk_data);
        Chunk {
            length: chunk_data.len() as u32,
            chunk_type,
            chunk_data,
            crc,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }
    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        self.chunk_data.as_slice()
    }

    fn crc(&self) -> u32 {
        self.crc
    }
    fn data_as_string(&self) -> Result<String, ChunkError> {
        String::from_utf8(self.chunk_data.clone()).map_err(|_| ChunkError::InvalidUTF8DataString)
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&Vec<u8>> for Chunk {
    type Error = ChunkError;
    fn try_from(bytes: &Vec<u8>) -> Result<Chunk, ChunkError> {
        let chunk_data_length = u32::from_be_bytes(bytes[..4].try_into().unwrap());
        let type_bytes: [u8; 4] = bytes[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(type_bytes).unwrap();
        let idx = bytes.len() - 4;
        let chunk_data = &bytes[8..idx].try_into().unwrap();
        let crc = u32::from_be_bytes(bytes[idx..].try_into().unwrap());
        let calc_crc = Chunk::create_crc(&chunk_type, &chunk_data);
        let actual_chunk_data_length = chunk_data.len().try_into().unwrap();
        if chunk_data_length != actual_chunk_data_length {
            return Err(ChunkError::InvalidChunkDatLength(
                chunk_data_length,
                actual_chunk_data_length,
            ));
        }
        if calc_crc != crc {
            return Err(ChunkError::InvalidCrc(calc_crc, crc));
        }
        Ok(Chunk::new(chunk_type, chunk_data.to_vec()))
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data_as_string().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
