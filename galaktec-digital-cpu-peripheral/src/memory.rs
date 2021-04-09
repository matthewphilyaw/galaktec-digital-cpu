use galaktec_digital_cpu_core::{DiscreteUnit, GenericIODevice, IODevice, Input, Output, WithIO};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::{Peripheral, WithPeripheral};
use core::mem;

#[derive(Debug, Clone, PartialEq)]
enum MemoryOperation {
    Write { address: usize, data: usize },
    Read { address: usize },
}

#[derive(Debug)]
struct Memory {
    latency: usize,
    counter: usize,
    operation: Option<MemoryOperation>,
    io_device: Rc<RefCell<GenericIODevice<MemoryOperation, usize>>>,
    raw_buffer: Vec<usize>,
}

impl Memory {
    fn new(capacity: usize, latency: usize) -> Self {
        Memory {
            latency,
            counter: 0,
            operation: None,
            io_device: Rc::new(RefCell::new(GenericIODevice::new())),
            raw_buffer: vec![0; capacity],
        }
    }
}

impl DiscreteUnit for Memory {
    fn send(&mut self) {}
    fn update(&mut self) {
        if self.operation.is_none() {
            if let Some(event) = self.io_device.borrow().events().first() {
                self.operation = Some(event.clone());
                self.counter = 1;
            }
        } else if self.operation.is_some() && self.counter < self.latency {
            self.counter += 1;
        }

        if self.operation.is_some() && self.counter == self.latency {
            let op = mem::replace(&mut self.operation, None).unwrap();
            match op {
                MemoryOperation::Write { address, data } => self.raw_buffer[address] = data,
                MemoryOperation::Read { address } => {
                    let output = self.raw_buffer[address];
                    self.io_device.borrow_mut().set_data(output);
                }
            }

            self.counter = 0;
            self.operation = None;
            self.io_device.borrow_mut().clear_events();
        }
    }
}

impl WithIO<MemoryOperation, usize> for Memory {
    fn io(&self) -> Weak<RefCell<GenericIODevice<MemoryOperation, usize>>> {
        Rc::downgrade(&self.io_device)
    }
}

impl WithPeripheral for Memory {
    fn peripheral(&self) -> Box<dyn Peripheral> {
        Box::new(MemoryPeripheral(Rc::downgrade(&self.io_device)))
    }
}

#[derive(Debug)]
struct MemoryPeripheral(Weak<RefCell<GenericIODevice<MemoryOperation, usize>>>);

impl Peripheral for MemoryPeripheral {
    fn write(&mut self, address: usize, data: usize) -> bool {
        let ptr = &mut self.0.upgrade();
        debug_assert_ne!(ptr.is_some(), false);

        if let Some(io) = ptr {
            io.borrow_mut()
                .push(MemoryOperation::Write { address, data })
        } else {
            false
        }
    }

    fn read(&mut self, address: usize) -> bool {
        let ptr = &mut self.0.upgrade();
        debug_assert_ne!(ptr.is_some(), false);

        if let Some(io) = ptr {
            io.borrow_mut().push(MemoryOperation::Read { address })
        } else {
            false
        }
    }

    fn output(&self) -> usize {
        let ptr = &mut self.0.upgrade();
        debug_assert_ne!(ptr.is_some(), false);

        if let Some(io) = ptr {
            io.borrow().output()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::Memory;
    use crate::WithPeripheral;
    use galaktec_digital_cpu_core::{DiscreteUnit, Output};

    #[test]
    fn write_within_n_cycles() {
        for cycles in 1..3 {
            let mut memory = Memory::new(1, cycles);
            let mut per = memory.peripheral();

            let input = 123;
            per.write(0, input);

            assert_eq!(0, memory.raw_buffer[0]);

            for _ in 0..cycles {
                memory.update();
            }

            assert_eq!(input, memory.raw_buffer[0])
        }
    }

    #[test]
    fn read_within_n_cycles() {
        for cycles in 1..3 {
            let mut memory = Memory::new(1, cycles);
            let mut per = memory.peripheral();

            let input = 123;
            memory.raw_buffer[0] = input;

            per.read(0);

            assert_eq!(0, memory.io_device.borrow().output());

            for _ in 0..cycles {
                memory.update();
            }

            assert_eq!(input, memory.io_device.borrow().output());
        }
    }

    #[test]
    fn deactivate_clears_io_device_events() {
        let mut memory = Memory::new(1, 1);
        let mut per = memory.peripheral();

        let input_one = 123;
        memory.raw_buffer[0] = input_one;

        per.read(0);

        assert_eq!(0, memory.io_device.borrow().output());

        memory.update();

        assert_eq!(input_one, memory.io_device.borrow().output());

        let input_two = 456;
        memory.raw_buffer[0] = input_two;

        memory.update();

        assert_ne!(input_two, memory.io_device.borrow().output());
    }

    #[test]
    fn subsequent_write_within_same_cycle_returns_false() {
        let mut memory = Memory::new(1, 1);
        let mut per = memory.peripheral();

        let input = 123;
        assert_eq!(per.write(0, input), true);
        assert_eq!(per.write(0, input), false);
    }

    #[test]
    fn subsequent_read_within_same_cycle_returns_false() {
        let mut memory = Memory::new(1, 1);
        let mut per = memory.peripheral();

        assert_eq!(per.read(0), true);
        assert_eq!(per.read(0), false);
    }

    #[test]
    fn subsequent_write_allowed_after_previous_write_finishes() {
        let mut memory = Memory::new(1, 1);
        let mut per = memory.peripheral();

        let input = 123;
        assert_eq!(per.write(0, input), true);
        assert_eq!(per.write(0, input), false);

        memory.update();

        assert_eq!(input, memory.raw_buffer[0]);

        let input_two = 456;
        assert_eq!(per.write(0, input_two), true);
        assert_eq!(per.write(0, input_two), false);

        memory.update();

        assert_eq!(input_two, memory.raw_buffer[0]);
    }

    #[test]
    fn subsequent_read_allowed_after_previous_write_finishes() {
        let mut memory = Memory::new(1, 1);
        let mut per = memory.peripheral();

        let input = 123;
        memory.raw_buffer[0] = input;

        assert_eq!(per.read(0), true);
        assert_eq!(per.read(0), false);

        memory.update();

        assert_eq!(input, memory.io_device.borrow().output());

        let input_two = 456;
        memory.raw_buffer[0] = input_two;

        assert_eq!(per.read(0), true);
        assert_eq!(per.read(0), false);

        memory.update();

        assert_eq!(input_two, memory.io_device.borrow().output());
    }
}
