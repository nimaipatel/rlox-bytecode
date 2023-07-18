#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}