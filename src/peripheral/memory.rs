use crate::bus::{ Operation };
use crate::peripheral::{Peripheral, address_map::AddressMap, Error};

use crate::clock::ClockedHigh;

#[derive(Debug)]
pub struct Memory {
    address_map: AddressMap,
    base_latency: usize,
    latency_counter: usize,
    storage: Vec<u32>,
    operation: Option<Operation>,
}

impl Memory {
    pub fn new(address_map: AddressMap, size: usize, base_latency: usize) -> Self {
        Memory {
            address_map,
            storage: vec![0; size as usize],
            base_latency,
            latency_counter: 0,
            operation: None
        }
    }
}

impl Peripheral for Memory {
    fn address_map(&self) -> AddressMap {
        self.address_map
    }
    fn do_operation(&mut self, operation: Operation) -> Result<(), Error> {
        unimplemented!()
    }

    fn result(&self) -> Result<Option<u32>, Error> {
        unimplemented!()
    }
}

impl ClockedHigh for Memory {
    fn clock_high(&mut self) {

    }
}
