mod address_map;
mod bus;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Data {
    Byte(u32),
    HalfWord(u32),
    Word(u32),
}

pub trait Peripheral {
    fn write(&mut self, address: u32, data: Data);
    fn read(&self, address: u32);
    fn read_result(&self) -> Option<u32>;
    fn busy(&self) -> bool;
}
