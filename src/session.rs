use crate::{
    basic_header::BasicHeader,
    chunk::Chunk,
    handshake::Handshake,
    message::{CommandMessageAmf0, Message, SetChunkSize},
};
use core::fmt;
use std::{collections::HashMap, io, net::TcpStream};

pub struct PrecedingChunkAttributes {
    pub message_type_id: u8,
}
pub(crate) struct Session<'a> {
    preceding: HashMap<u32, PrecedingChunkAttributes>,
    // bytes_received: u32,
    stream: &'a mut TcpStream,
}
impl<'a> Session<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self {
            preceding: HashMap::new(),
            // bytes_received: 0,
            stream,
        }
    }
    // fn get_bytes_received(&self) -> u32 {
    //     self.bytes_received
    // }
    pub fn handle(&mut self) -> io::Result<()> {
        Handshake::new().response(self.stream)?;

        loop {
            let chunk = Chunk::read_from_stream(self.stream)?;
            let fmt = chunk.basic_header.get_fmt();
            let cs_id = chunk.basic_header.get_cs_id();
            let message_type_id = match fmt {
                0 | 1 => chunk.message_header.get_message_type_id().unwrap(),
                _ => self.preceding.get(&cs_id).unwrap().message_type_id,
            };

            if let Some(message_stream_id) = chunk.message_header.get_message_stream_id() {
                println!("message_stream_id: {}", message_stream_id,);
            };

            println!("fmt: {}", fmt);
            println!("cs_id: {}", cs_id);

            println!("message_type_id: {}", message_type_id,);
            match message_type_id {
                1 => {
                    println!("Handle Set Chunk Size (1)");
                    let message = SetChunkSize::new(chunk.chunk_data.try_into().unwrap());
                    message.handle(self.stream);
                }
                2 => {
                    println!("Abort Message (2)")
                }
                3 => {
                    println!("Acknowledgement (3)")
                }
                4 => {
                    println!("User Control Messages (4)")
                }
                5 => {
                    println!("Window Acknowledgement Size (5)")
                }
                6 => {
                    println!("Set Peer Bandwidth (6)")
                }
                // 7 =>
                8 => {
                    println!("Audio Message (8)")
                }
                9 => {
                    println!("Video Message (9)")
                }
                // 10 =>
                // 11 =>
                // 12 =>
                // 13 =>
                // 14 =>
                15 => {
                    println!("Data Message Amf3 (15)")
                }
                16 => {
                    println!("Shared Object Message (16)")
                }
                17 => {
                    println!("Command Message Amf3 (17)")
                }
                18 => {
                    println!("Data Message Amf0 (18)")
                }
                19 => {
                    println!("Shared Object Message (19)")
                }
                20 => {
                    println!("Command Message Amf0 (20)");
                    let message = CommandMessageAmf0::new(chunk.chunk_data.try_into().unwrap());
                    message.handle(self.stream);
                }
                22 => {
                    println!("Aggregate Message (22)")
                }
                _ => {
                    println!("Unhandled message!")
                }
            }

            if let Some(timestamp) = chunk.message_header.get_timestamp() {
                println!("timestamp: {}", timestamp);
            }
            if let Some(get_timestamp_delta) = chunk.message_header.get_timestamp_delta() {
                println!("timestamp_delta: {}", get_timestamp_delta);
            }

            self.preceding
                .entry(cs_id)
                .and_modify(|attributes| attributes.message_type_id = message_type_id)
                .or_insert(PrecedingChunkAttributes { message_type_id: message_type_id });
        }
        Ok(())
    }
}
