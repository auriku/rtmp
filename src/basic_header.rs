pub(crate) trait BasicHeader: BasicHeaderClone {
    fn get_fmt(&self) -> u8;
    fn get_cs_id(&self) -> u32;
}
trait BasicHeaderClone {
    fn clone_box(&self) -> Box<dyn BasicHeader>;
}
impl<T> BasicHeaderClone for T
where
    T: 'static + BasicHeader + Clone,
{
    fn clone_box(&self) -> Box<dyn BasicHeader> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn BasicHeader> {
    fn clone(&self) -> Box<dyn BasicHeader> {
        self.clone_box()
    }
}
#[derive(Debug, Clone)]
pub struct BasicHeader1 {
    pub data: [u8; 1],
}
impl BasicHeader1 {
    pub fn new(data: [u8; 1]) -> Self {
        Self { data }
    }
    pub fn new_from_values(fmt: u8, cs_id: u8) -> Self {
        Self { data: [fmt << 6 | (cs_id & 0b00111111)] }
    }
}
impl BasicHeader for BasicHeader1 {
    fn get_fmt(&self) -> u8 {
        self.data[0] >> 6 & 0b11
    }

    fn get_cs_id(&self) -> u32 {
        (self.data[0] & 0b00111111) as u32
    }
}
#[derive(Debug, Clone)]
pub struct BasicHeader2 {
    data: [u8; 2],
}
impl BasicHeader2 {
    pub fn new(data: [u8; 2]) -> Self {
        Self { data }
    }
}
impl BasicHeader for BasicHeader2 {
    fn get_fmt(&self) -> u8 {
        self.data[0] >> 6 & 0b11
    }

    fn get_cs_id(&self) -> u32 {
        self.data[1] as u32 + 64
    }
}
#[derive(Debug, Clone)]
pub struct BasicHeader3 {
    data: [u8; 3],
}
impl BasicHeader3 {
    pub fn new(data: [u8; 3]) -> Self {
        Self { data }
    }
}
impl BasicHeader for BasicHeader3 {
    fn get_fmt(&self) -> u8 {
        self.data[0] >> 6 & 0b11
    }

    fn get_cs_id(&self) -> u32 {
        self.data[2] as u32 * 256  + self.data[1] as u32 + 64
    }
}