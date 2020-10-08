use crate::bus::Operation;
use crate::bus::interface::Interface;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Error {
    PeripheralNotReady,
    PeripheralBusy,
}

pub trait Peripheral: Debug {
    fn do_operation(&mut self, operation: Operation, interface: &Interface) -> Result<(), Error>;
    fn result(&mut self) -> Result<Option<u32>, Error>;
}
