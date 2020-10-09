use crate::bus::{ Operation };
use std::fmt::Debug;

pub mod address_map;
pub mod interface;

pub use self::address_map::*;
pub use self::interface::*;

#[derive(Debug)]
pub enum Error {
    PeripheralBusy,
}

pub trait Peripheral: Debug {
    fn address_map(&self) -> AddressMap;
    fn do_operation(&mut self, operation: Operation) -> Result<(), Error>;
    fn result(&self) -> Result<Option<u32>, Error>;
}
