use std::{cell::RefCell, rc::Rc};
use crate::bus::{ Bus, peripheral::Peripheral};
use crate::clock::ClockedLow;
use std::borrow::BorrowMut;

use self::State::*;


#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    Free,
    Waiting
}

#[derive(Debug)]
pub struct Interface {
    state: State,
    base_address: u32,
    block_size: u32,
    bus: Rc<RefCell<Bus>>,
    peripheral: Box<dyn Peripheral>,
}

impl Interface {
   pub fn new(
       base_address: u32,
       block_size: u32,
       bus: Rc<RefCell<Bus>>,
       peripheral: Box<dyn Peripheral>
   ) -> Self {
       debug_assert_ne!(block_size, 0, "Block size must be greater than zero");

       Interface {
           state: Free,
           base_address,
           block_size,
           bus,
           peripheral
       }
   }
}

impl ClockedLow for Interface {
    fn clock_low(&mut self) {


    }
}
