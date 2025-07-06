use rand::Rng;
use std::io::{self, Read, Write};
use std::net::TcpStream;

use crate::utils::timestamp;

fn debug_print_bytes(label: &str, bytes: &[u8]) {
    print!("{}: ", label);
    for byte in bytes {
        print!("{:02X} ", byte);
    }
    println!();
}

#[derive(Debug)]
struct HandshakeChunk0 {
    version: [u8; 1],
}
impl HandshakeChunk0 {
    fn new() -> Self {
        Self { version: [0u8; 1] }
    }

    fn read(&mut self, stream: &mut TcpStream) -> io::Result<&mut Self> {
        stream.read_exact(&mut self.version)?;
        Ok(self)
    }

    fn set_version(&mut self, version: u8) {
        self.version = [version];
    }
    fn write(&self, stream: &mut TcpStream) -> io::Result<()> {
        stream.write_all(&self.version)?;
        stream.flush()?;
        Ok(())
    }
}
#[derive(Debug)]
struct HandshakeChunk1 {
    time: [u8; 4],
    zero: [u8; 4],
    random_data: [u8; 1528],
}
impl HandshakeChunk1 {
    fn new() -> Self {
        Self {
            time: [0u8; 4],
            zero: [0u8; 4],
            random_data: [0u8; 1528],
        }
    }
    fn read(&mut self, stream: &mut TcpStream) -> io::Result<&mut Self> {
        stream.read_exact(&mut self.time)?;
        stream.read_exact(&mut self.zero)?;
        stream.read_exact(&mut self.random_data)?;
        Ok(self)
    }
    fn write(&self, stream: &mut TcpStream) -> io::Result<()> {
        stream.write_all(&self.time)?;
        stream.write_all(&self.zero)?;
        stream.write_all(&self.random_data)?;
        stream.flush()?;
        Ok(())
    }
    fn set_time(&mut self) {
        let timestamp = timestamp();
        self.time = u32::to_be_bytes(timestamp);
    }
    fn set_zero(&mut self) {
        self.zero = [0u8; 4];
    }
    fn set_random_data(&mut self) {
        for element in self.random_data.iter_mut() {
            let mut rng = rand::rng();
            *element = rng.random::<u8>();
        }
    }
    fn prepare(&mut self) {
        self.set_time();
        self.set_zero();
        self.set_random_data();
    }
}
#[derive(Debug)]
struct HandshakeChunk2 {
    time: [u8; 4],
    time_2: [u8; 4],
    random_echo: [u8; 1528],
}
impl HandshakeChunk2 {
    fn new() -> Self {
        Self {
            time: [0u8; 4],
            time_2: [0u8; 4],
            random_echo: [0u8; 1528],
        }
    }
    fn read(&mut self, stream: &mut TcpStream) -> io::Result<&mut Self> {
        stream.read_exact(&mut self.time)?;
        stream.read_exact(&mut self.time_2)?;
        stream.read_exact(&mut self.random_echo)?;
        Ok(self)
    }
    fn write(&self, stream: &mut TcpStream) -> io::Result<()> {
        stream.write_all(&self.time)?;
        stream.write_all(&self.time_2)?;
        stream.write_all(&self.random_echo)?;
        stream.flush()?;
        Ok(())
    }
    fn set_time(&mut self, time: [u8; 4]) {
        self.time = time;
    }
    fn set_time_2(&mut self, time: u32) {
        self.time_2 = u32::to_be_bytes(time);
    }
    fn set_random_echo(&mut self, random_echo: [u8; 1528]) {
        self.random_echo = random_echo;
    }
}

#[derive(Debug)]
pub(crate) struct Handshake {
    c0: HandshakeChunk0,
    c1: HandshakeChunk1,
    c2: HandshakeChunk2,
    s0: HandshakeChunk0,
    s1: HandshakeChunk1,
    s2: HandshakeChunk2,
}
impl Handshake {
    pub fn new() -> Self {
        Self {
            c0: HandshakeChunk0::new(),
            c1: HandshakeChunk1::new(),
            c2: HandshakeChunk2::new(),
            s0: HandshakeChunk0::new(),
            s1: HandshakeChunk1::new(),
            s2: HandshakeChunk2::new(),
        }
    }
    pub fn response(&mut self, stream: &mut TcpStream) -> io::Result<()> {
        self.c0.read(stream)?;
        // TODO validate version!!!!!!
        self.s0.set_version(3);
        self.s0.write(stream)?;

        self.s1.prepare();
        self.s1.write(stream)?;

        self.c1.read(stream)?;

        self.s2.set_time_2(0);
        self.s2.set_time(self.c1.time);
        self.s2.set_random_echo(self.c1.random_data);
        self.s2.write(stream)?;
        println!("{:?}", self.c1.random_data == self.s2.random_echo);
        self.c2.read(stream)?;
        Ok(())
    }
}
