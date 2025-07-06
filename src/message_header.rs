use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
pub trait MessageHeader: MessageHeaderClone {
    fn get_timestamp(&self) -> Option<u32>;
    fn get_timestamp_delta(&self) -> Option<u32>;
    fn get_message_length(&self) -> Option<u32>;
    fn get_message_type_id(&self) -> Option<u8>;
    fn get_message_stream_id(&self) -> Option<u32>;
}
trait MessageHeaderClone {
    fn clone_box(&self) -> Box<dyn MessageHeader>;
}
impl<T> MessageHeaderClone for T
where
    T: 'static + MessageHeader + Clone,
{
    fn clone_box(&self) -> Box<dyn MessageHeader> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn MessageHeader> {
    fn clone(&self) -> Box<dyn MessageHeader> {
        self.clone_box()
    }
}
#[derive(Debug, Clone)]
pub struct MessageHeaderType0 {
    pub data: [u8; 11],
}
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                 timestamp                     |message length |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |      message length (cont)    |message type id| msg stream id |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |            message stream id (cont)           |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
impl MessageHeaderType0 {
    pub fn new(data: [u8; 11]) -> Self {
        Self { data }
    }
    pub fn new_from_values(
        timestamp: u32,
        message_length: u32,
        message_type_id: u8,
        message_stream_id: u32,
    ) -> Self {
        let mut data = Cursor::new([0u8; 11]);
        data.write_u24::<BigEndian>(timestamp).unwrap(); // timestamp
        data.write_u24::<BigEndian>(message_length).unwrap(); // message_length
        data.write_u8(message_type_id).unwrap(); // message_type_id
        data.write_u32::<BigEndian>(message_stream_id).unwrap(); // message_stream_id
        Self {
            data: data.into_inner(),
        }
    }
}
impl MessageHeader for MessageHeaderType0 {
    fn get_timestamp(&self) -> Option<u32> {
        let timestamp = Cursor::new(&self.data[..3]).read_u24::<BigEndian>().ok()?;
        Some(timestamp)
    }
    fn get_message_length(&self) -> Option<u32> {
        let message_length = Cursor::new(&self.data[3..6]).read_u24::<BigEndian>().ok()?;
        Some(message_length)
    }
    fn get_message_type_id(&self) -> Option<u8> {
        Some(self.data[6])
    }
    fn get_message_stream_id(&self) -> Option<u32> {
        let message_stream_id = Cursor::new(&self.data[7..]).read_u32::<BigEndian>().ok()?;
        Some(message_stream_id)
    }
    fn get_timestamp_delta(&self) -> Option<u32> {
        None
    }
}
#[derive(Debug, Clone)]
pub struct MessageHeaderType1 {
    pub data: [u8; 7],
}
impl MessageHeaderType1 {
    pub fn new(data: [u8; 7]) -> Self {
        Self { data }
    }
}
impl MessageHeader for MessageHeaderType1 {
    fn get_timestamp(&self) -> Option<u32> {
        None
    }
    fn get_message_length(&self) -> Option<u32> {
        let message_length = Cursor::new(&self.data[3..6]).read_u24::<BigEndian>().ok()?;
        Some(message_length)
    }
    fn get_message_type_id(&self) -> Option<u8> {
        Some(self.data[6])
    }
    fn get_message_stream_id(&self) -> Option<u32> {
        None
    }

    fn get_timestamp_delta(&self) -> Option<u32> {
        let timestamp_delta = Cursor::new(&self.data[..3]).read_u24::<BigEndian>().ok()?;
        Some(timestamp_delta)
    }
}
#[derive(Debug, Clone)]
pub struct MessageHeaderType2 {
    pub data: [u8; 3],
}
impl MessageHeaderType2 {
    pub fn new(data: [u8; 3]) -> Self {
        Self { data }
    }
}
impl MessageHeader for MessageHeaderType2 {
    fn get_timestamp(&self) -> Option<u32> {
        None
    }

    fn get_timestamp_delta(&self) -> Option<u32> {
        let timestamp_delta = Cursor::new(&self.data[..3]).read_u24::<BigEndian>().ok()?;
        Some(timestamp_delta)
    }

    fn get_message_length(&self) -> Option<u32> {
        None
    }

    fn get_message_type_id(&self) -> Option<u8> {
        None
    }

    fn get_message_stream_id(&self) -> Option<u32> {
        None
    }
}
#[derive(Debug, Clone)]
pub struct MessageHeaderType3 {
    pub data: [u8; 0],
}
impl MessageHeaderType3 {
    pub fn new(data: [u8; 0]) -> Self {
        Self { data }
    }
}
impl MessageHeader for MessageHeaderType3 {
    fn get_timestamp(&self) -> Option<u32> {
        None
    }

    fn get_timestamp_delta(&self) -> Option<u32> {
        None
    }

    fn get_message_length(&self) -> Option<u32> {
        None
    }

    fn get_message_type_id(&self) -> Option<u8> {
        None
    }

    fn get_message_stream_id(&self) -> Option<u32> {
        None
    }
}
