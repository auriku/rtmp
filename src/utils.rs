use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Cursor;

pub fn timestamp() -> u32 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");
        duration.as_millis() as u32
}

pub trait CursorExt {
    fn cursor(&self) -> Cursor<&[u8]>;
}

// Implement the CursorExt trait for byte slices
impl CursorExt for [u8] {
    fn cursor(&self) -> Cursor<&[u8]> {
        Cursor::new(self)
    }
}
