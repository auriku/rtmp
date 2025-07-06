use byteorder::{BigEndian, WriteBytesExt};
use std::{
    io::{Cursor, Read, Write},
    net::TcpStream,
    time::Duration,
};

use amf::{Amf0Value, Value, Version};

use crate::{
    basic_header::BasicHeader1,
    message_header::{MessageHeaderType0, MessageHeaderType1},
};

pub trait Message {
    fn handle(&self, stream: &mut TcpStream);
}
// =====================================================================================
pub struct SetChunkSize {
    payload: [u8; 4], // chunk_size
}
impl SetChunkSize {
    pub fn new(payload: [u8; 4]) -> Self {
        Self { payload }
    }
}
impl Message for SetChunkSize {
    fn handle(&self, stream: &mut TcpStream) {
        println!("handle SetChunkSize");
        println!("chunk sizeset to: {}", u32::from_be_bytes(self.payload))
    }
}

// =====================================================================================
pub struct AcknowledgementWindowSize {
    payload: [u8; 4],
}
impl AcknowledgementWindowSize {
    pub fn new(payload: [u8; 4]) -> Self {
        Self { payload }
    }
}
impl Message for AcknowledgementWindowSize {
    fn handle(&self, stream: &mut TcpStream) {
        todo!()
    }
}
// =====================================================================================
pub struct SetPeerBandwidth {
    payload: [u8; 5],
}
impl SetPeerBandwidth {
    pub fn new(payload: [u8; 5]) -> Self {
        Self { payload }
    }
    pub fn new_from_values(ack_win_size: u32, limit_type: u8) -> Self {
        let mut data = Cursor::new([0u8; 5]);
        data.write_u32::<BigEndian>(ack_win_size).unwrap();
        data.write_u8(limit_type).unwrap();
        Self { payload: data.into_inner() }
    }
}
// =====================================================================================
struct UserControlMessage {
    payload: Vec<u8>,
}
impl UserControlMessage {
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
}
// =====================================================================================
pub struct CommandMessageAmf0 {
    payload: Vec<u8>,
}
impl CommandMessageAmf0 {
    pub fn new(data: Vec<u8>) -> Self {
        Self { payload: data }
    }
}
impl Message for CommandMessageAmf0 {
    // +--------------+                              +-------------+
    // |    Client    |             |                |    Server   |
    // +------+-------+             |                +------+------+
    //        |              Handshaking done               |
    //        |                     |                       |
    //        |                     |                       |
    //        |                     |                       |
    //        |                     |                       |
    //        |----------- Command Message(connect) ------->|
    //        |                                             |
    //        |<------- Window Acknowledgement Size --------|
    //        |                                             |
    //        |<----------- Set Peer Bandwidth -------------|
    //        |                                             |
    //        |-------- Window Acknowledgement Size ------->|
    //        |                                             |
    //        |<------ User Control Message(StreamBegin) ---|
    //        |                                             |
    //        |<------------ Command Message ---------------|
    //        |       (_result- connect response)           |
    //        |                                             |

    fn handle(&self, stream: &mut TcpStream) {
        println!("CommandMessageAmf0");

        let mut decoder = amf::amf0::Decoder::new(&self.payload[..]);
        let command = decoder.decode();
        let transaction_id = decoder.decode();
        let command_object = decoder.decode();
        println!("{:?}", command);
        println!("{:?}", transaction_id);
        println!("{:?}", command_object);

        //acknowledgement_window_size message chunk
        let basic_header = BasicHeader1::new_from_values(0, 2);
        let message_header = MessageHeaderType0::new_from_values(0, 4, 5, 0);
        let acknowledgement_window_size = AcknowledgementWindowSize::new(u32::to_be_bytes(5000000));

        match stream.write_all(&basic_header.data) {
            Ok(_) => println!("Basic header sent successfully."),
            Err(e) => println!("Failed to send basic header: {:?}", e),
        };
        match stream.write_all(&message_header.data) {
            Ok(_) => println!("Message header sent successfully."),
            Err(e) => println!("Failed to send message header: {:?}", e),
        };
        match stream.write_all(&acknowledgement_window_size.payload) {
            Ok(_) => println!("acknowledgement_window_size sent successfully."),
            Err(e) => println!("Failed to send acknowledgement_window_size: {:?}", e),
        };
        match stream.flush() {
            Ok(_) => println!("flushed successfully."),
            Err(e) => println!("Failed to flush: {:?}", e),
        }

        // set_peer_bandwidth message chunk
        let basic_header = BasicHeader1::new_from_values(0, 2);
        let message_header = MessageHeaderType0::new_from_values(0, 5, 6, 0);
        let set_peer_bandwidth = SetPeerBandwidth::new_from_values(5000000, 2);

        match stream.write_all(&basic_header.data) {
            Ok(_) => println!("Basic header sent successfully."),
            Err(e) => println!("Failed to send basic header: {:?}", e),
        };
        match stream.write_all(&message_header.data) {
            Ok(_) => println!("Message header sent successfully."),
            Err(e) => println!("Failed to send message header: {:?}", e),
        };
        match stream.write_all(&set_peer_bandwidth.payload) {
            Ok(_) => println!("set_peer_bandwidth sent successfully."),
            Err(e) => println!("Failed to send set_peer_bandwidth: {:?}", e),
        };
        match stream.flush() {
            Ok(_) => println!("flushed successfully."),
            Err(e) => println!("Failed to flush: {:?}", e),
        }

        // // stream_begin
        // let basic_header = BasicHeader1::new_from_values(0, 2);
        // let message_header = MessageHeaderType0::new_from_values(0, 6, 4, 0);
        // let mut stream_begin_payload: Cursor<Vec<u8>> = Cursor::new(vec![]);
        // stream_begin_payload.write_u16::<BigEndian>(0).unwrap();
        // stream_begin_payload.write_u32::<BigEndian>(1).unwrap();
        // let stream_begin = UserControlMessage::new(stream_begin_payload.into_inner());

        // match stream.write_all(&basic_header.data) {
        //     Ok(_) => println!("Basic header sent successfully."),
        //     Err(e) => println!("Failed to send basic header: {:?}", e),
        // };
        // match stream.write_all(&message_header.data) {
        //     Ok(_) => println!("Message header sent successfully."),
        //     Err(e) => println!("Failed to send message header: {:?}", e),
        // };
        // match stream.write_all(&stream_begin.payload) {
        //     Ok(_) => println!("set_peer_bandwidth sent successfully."),
        //     Err(e) => println!("Failed to send set_peer_bandwidth: {:?}", e),
        // };
        // match stream.flush() {
        //     Ok(_) => println!("flushed successfully."),
        //     Err(e) => println!("Failed to flush: {:?}", e),
        // }
        // stream
        //     .set_read_timeout(Some(Duration::new(5, 0)))
        //     .expect("Failed to set read timeout");


        let mut response_message = Vec::new();
        Amf0Value::String("_result".to_string())
            .write_to(&mut response_message).unwrap();
        Amf0Value::Number(1.0)
            .write_to(&mut response_message).unwrap();
        // Amf0Value::Object { class_name, entries }
        Amf0Value::Object {
            class_name: None,
            entries: vec![
                amf::Pair {
                    key: "fmsVer".to_string(),
                    value: Amf0Value::String("FMS/3,5,7,7007".to_string()),
                },
                amf::Pair {
                    key: "capabilities".to_string(),
                    value: Amf0Value::Number(31.0),
                },
            ],
        }
        .write_to(&mut response_message).unwrap();

        let basic_header = BasicHeader1::new_from_values(0, 3);
        let message_header = MessageHeaderType0::new_from_values(0, response_message.len() as u32, 20, 0);

        match stream.write_all(&basic_header.data) {
            Ok(_) => println!("Basic header sent successfully."),
            Err(e) => println!("Failed to send basic header: {:?}", e),
        };
        match stream.write_all(&message_header.data) {
            Ok(_) => println!("Message header sent successfully."),
            Err(e) => println!("Failed to send message header: {:?}", e),
        };
        match stream.write_all(&response_message) {
            Ok(_) => println!("response sent successfully."),
            Err(e) => println!("Failed to send set_peer_bandwidth: {:?}", e),
        };
        match stream.flush() {
            Ok(_) => println!("flushed successfully."),
            Err(e) => println!("Failed to flush: {:?}", e),
        }

    }
}

// enum ProtocolControlMessage {
//     SetChunkSize, // https://rtmp.veriskope.com/docs/spec/#541set-chunk-size-1
//     AbortMessage, // https://rtmp.veriskope.com/docs/spec/#542abort-message-2
//     Acknowledgement, // https://rtmp.veriskope.com/docs/spec/#543acknowledgement-3
//     WindowAcknowledgementSize, // https://rtmp.veriskope.com/docs/spec/#544window-acknowledgement-size-5
//     SetPeerBandwidth, // https://rtmp.veriskope.com/docs/spec/#545set-peer-bandwidth-6
// }
