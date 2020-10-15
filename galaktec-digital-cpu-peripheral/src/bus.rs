use std::rc::Rc;
use std::cell::RefCell;
use crate::{Peripheral, Data};
use crate::address_map::AddressMap;

struct Bus {
    peripherals: Vec<(AddressMap, Rc<RefCell<Box<dyn Peripheral>>>)>,
    active_peripheral_index: Option<usize>
}

impl Bus {
    pub fn new(peripherals: Vec<(AddressMap, Rc<RefCell<Box<dyn Peripheral>>>)>) -> Self {
        Bus {
            peripherals,
            active_peripheral_index: None,
        }
    }
}

impl Peripheral for Bus {
    fn write(&mut self, address: u32, data: Data) {
        unimplemented!()
    }

    fn read(&self, address: u32) {
        unimplemented!()
    }

    fn read_result(&self) -> Option<u32> {
        unimplemented!()
    }

    fn busy(&self) -> bool {
        unimplemented!()
    }
}