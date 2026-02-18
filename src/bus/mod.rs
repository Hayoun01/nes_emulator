pub mod simple_bus;

pub type Byte = u8;
pub type Word = u16;

pub trait Bus {
    fn read(&mut self, addr: Word, read_only: bool) -> Byte;
    fn write(&mut self, addr: Word, value: Byte);
}
