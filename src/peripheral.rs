use crate::bus::{ Operation };
use std::fmt::Debug;
use crate::peripheral::address_map::AddressMap;

pub mod memory;
pub mod address_map;
pub mod interface;

#[derive(Debug)]
pub enum Error {
    PeripheralBusy,
}

pub trait Peripheral: Debug {
    fn address_map(&self) -> AddressMap;
    fn do_operation(&mut self, operation: Operation) -> Result<(), Error>;
    fn result(&self) -> Result<Option<u32>, Error>;
}
