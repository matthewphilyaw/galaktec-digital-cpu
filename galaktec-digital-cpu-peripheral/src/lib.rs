use std::fmt::Debug;

mod address_map;
mod memory;

pub trait WithPeripheral {
    fn peripheral(&self) -> Box<dyn Peripheral>;
}

pub trait Peripheral: Debug {
    fn write(&mut self, address: usize, data: usize) -> bool;
    fn read(&mut self, address: usize) -> bool;
    fn output(&self) -> usize;
}
