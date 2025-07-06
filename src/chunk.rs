use std::{
    io::{self, Read},
    net::TcpStream,
};

use crate::{
    basic_header::{BasicHeader, BasicHeader1, BasicHeader2, BasicHeader3},
    message_header::{
        MessageHeader, MessageHeaderType0, MessageHeaderType1, MessageHeaderType2,
        MessageHeaderType3,
    },
};

#[derive(Clone)]
pub(crate) struct Chunk {
    pub basic_header: Box<dyn BasicHeader>,
    pub message_header: Box<dyn MessageHeader>,
    // pub extended_timestamp: Option<[u8; 4]>,
    pub chunk_data: Vec<u8>,
}
impl Chunk {
    pub fn new(
        basic_header: Box<dyn BasicHeader>,
        message_header: Box<dyn MessageHeader>,
        chunk_data: Vec<u8>,
    ) -> Self {
        Self {
            basic_header,
            message_header,
            chunk_data,
        }
    }
    pub fn read_from_stream(stream: &mut TcpStream) -> io::Result<Self> {
        let mut first_byte = [0u8; 1];
        stream.read_exact(&mut first_byte)?;

        let basic_header: Box<dyn BasicHeader> = match first_byte[0] & 0b00111111 {
            0 => {
                let mut next_byte = [0u8; 1];
                stream.read_exact(&mut next_byte)?;
                Box::new(BasicHeader2::new([first_byte[0], next_byte[0]]))
            }
            1 => {
                let mut next_2_bytes = [0u8; 2];
                stream.read_exact(&mut next_2_bytes)?;
                Box::new(BasicHeader3::new([
                    first_byte[0],
                    next_2_bytes[0],
                    next_2_bytes[1],
                ]))
            }
            _ => Box::new(BasicHeader1::new(first_byte)),
        };
        println!("fmt: {:?}, cs_id: {}", basic_header.get_fmt(), basic_header.get_cs_id());
        let message_header: Box<dyn MessageHeader> = match basic_header.get_fmt() {
            0 => {
                let mut next_11_bytes = [0u8; 11];
                stream.read_exact(&mut next_11_bytes)?;
                Box::new(MessageHeaderType0::new(next_11_bytes))
            }
            1 => {
                let mut next_7_bytes = [0u8; 7];
                stream.read_exact(&mut next_7_bytes)?;
                Box::new(MessageHeaderType1::new(next_7_bytes))
            }
            2 => {
                let mut next_3_bytes = [0u8; 3];
                stream.read_exact(&mut next_3_bytes)?;
                Box::new(MessageHeaderType2::new(next_3_bytes))
            }
            _ => Box::new(MessageHeaderType3::new([])),
        };

        let chunk_data = if let Some(message_length) = message_header.get_message_length() {
            let mut data = vec![0u8; message_length as usize];
            stream.read_exact(&mut data)?;
            data
        } else {
            vec![]
        };

        Ok(Self {
            basic_header,
            message_header,
            chunk_data,
        })
    }
}
