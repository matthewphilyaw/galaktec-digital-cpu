use std::{cell::RefCell, rc::Rc};
use crate::bus::Bus;
use crate::peripheral::{ Peripheral, address_map::AddressMap };
use crate::clock::ClockedLow;

use self::State::*;
use std::borrow::Borrow;

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    FetchOperation,
    OperationInProgress,
}

#[derive(Debug)]
pub struct Interface {
    state: State,
    address_map: AddressMap,
    bus: Rc<RefCell<Bus>>,
    peripheral: Box<dyn Peripheral>,
}

impl Interface {
   pub fn new(
       address_map: AddressMap,
       bus: Rc<RefCell<Bus>>,
       peripheral: Box<dyn Peripheral>
   ) -> Self {
       Interface {
           state: FetchOperation,
           address_map,
           bus,
           peripheral
       }
   }
}

impl ClockedLow for Interface {
    fn clock_low(&mut self) {
        if self.state == FetchOperation {
            let operation = self
                .bus
                .borrow_mut()
                .operation_for_address_map(self.peripheral.address_map());
            debug_assert!(operation.is_ok(), "Bus operation failed on fetching operation");

            let result = self.peripheral.do_operation(operation.unwrap());
            debug_assert!(result.is_ok(), "Peripheral should accept operation in this state");

            self.state = OperationInProgress;
        }

        if self.state == OperationInProgress {
            let peripheral_result = self.peripheral.result();
            if let Ok(result) = peripheral_result {
                let bus_result = self.bus.borrow_mut().release(result);
                debug_assert!(bus_result.is_ok(), "Bus operation failed on release");

                self.state = FetchOperation;
            }
        }
    }
}
