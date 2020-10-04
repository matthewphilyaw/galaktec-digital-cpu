use crate::cpu::Clocked;

mod bus;

pub trait PeripheralInterface: Clocked {
    fn write_req(&mut self, address: u32, value: u32);
    fn read_req(&mut self, address: u32);

    fn busy(&self) -> bool;
    fn result(&self) -> u32;
}
