use galaktec_digital_cpu_core::{
    Discrete, GenericClock, GenericIODevice, IODevice, Input, Output, WithIO,
};

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, Copy, Clone, PartialEq)]
enum CounterOperation {
    Set(usize),
    Reset,
}

#[derive(Debug)]
struct Counter {
    io_device: Rc<RefCell<GenericIODevice<CounterOperation, usize>>>,
    temp_count: usize,
}

impl Counter {
    fn new() -> Self {
        let io = GenericIODevice::new();

        Counter {
            io_device: Rc::new(RefCell::new(io)),
            temp_count: 0,
        }
    }
}

impl Discrete for Counter {
    fn activate(&mut self) {
        self.temp_count += 1;
    }

    fn process_input(&mut self) {
        for ev in self.io_device.borrow().events() {
            match ev {
                CounterOperation::Set(value) => self.temp_count = *value,
                CounterOperation::Reset => self.temp_count = 0,
            }
        }
    }

    fn deactivate(&mut self) {
        self.io_device.borrow_mut().set_data(self.temp_count);
        self.io_device.borrow_mut().clear_events();
    }
}

impl WithIO<CounterOperation, usize> for Counter {
    fn io(&self) -> Weak<RefCell<GenericIODevice<CounterOperation, usize>>> {
        Rc::downgrade(&self.io_device)
    }
}

type CounterDevice = Weak<RefCell<dyn IODevice<CounterOperation, usize>>>;
#[derive(Debug)]
struct CounterReset {
    trigger_at: usize,
    set_to: usize,
    counter_device: CounterDevice,
}

impl CounterReset {
    fn new(trigger_at: usize, set_to: usize, counter_device: CounterDevice) -> Self {
        CounterReset {
            trigger_at,
            set_to,
            counter_device,
        }
    }
}

impl Discrete for CounterReset {
    fn activate(&mut self) {
        if let Some(cd) = self.counter_device.upgrade() {
            let current_count = cd.borrow().output();

            if current_count == self.trigger_at {
                cd.borrow_mut().push(CounterOperation::Set(self.set_to));
            }
        }
    }

    fn process_input(&mut self) {}

    fn deactivate(&mut self) {}
}

#[test]
fn counter_test() {
    let counter = Box::new(Counter::new());
    let counter_reset = Box::new(CounterReset::new(10, 20, counter.io()));

    let observer = counter.io();
    let mut clock = GenericClock::new(vec![counter, counter_reset]);

    for n in 0..11 {
        println!(
            "counter before step {}: {}",
            n,
            observer.upgrade().unwrap().borrow().output()
        );
        clock.step();
        println!(
            "counter after step {}: {}",
            n,
            observer.upgrade().unwrap().borrow().output()
        );
    }

    assert_eq!(observer.upgrade().unwrap().borrow().output(), 20);
}
