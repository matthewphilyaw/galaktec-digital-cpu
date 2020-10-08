use crate::{bus::Bus, clock::ClockedLow};
use std::{cell::RefCell, rc::Rc};

pub struct Memory {
    words: Vec<u32>,
    bus: Rc<RefCell<Bus>>,
    base_latency: usize,
    latency_counter: usize,
}

impl ClockedLow for Memory {
    fn clock_low(&mut self) {}
}
