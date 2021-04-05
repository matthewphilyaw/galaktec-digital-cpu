use galaktec_digital_cpu_core::{
    Discrete, GenericClock, GenericIODevice, IODevice, Output, WithIO,
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
}

impl Counter {
    fn new() -> Self {
        let io = GenericIODevice::new();

        Counter {
            io_device: Rc::new(RefCell::new(io)),
        }
    }
}

impl Discrete for Counter {
    fn send(&mut self) {}
    fn update(&mut self) {
        let mut next = self.io_device.borrow().output() + 1;

        for ev in self.io_device.borrow().events() {
            next = match ev {
                CounterOperation::Set(value) => *value,
                CounterOperation::Reset => 0,
            }
        }

        self.io_device.borrow_mut().set_data(next);
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
    fn send(&mut self) {
        if let Some(cd) = self.counter_device.upgrade() {
            let current_count = cd.borrow().output();

            if current_count == self.trigger_at {
                cd.borrow_mut().push(CounterOperation::Set(self.set_to));
            }
        }
    }
    fn update(&mut self) {}
}

#[test]
fn counter_test() {
    let counter = Box::new(Counter::new());
    let counter_reset = Box::new(CounterReset::new(10, 20, counter.io()));

    let observer = counter.io();
    let mut clock = GenericClock::new(vec![counter, counter_reset]);

    for n in 0..10 {
        assert_eq!(observer.upgrade().unwrap().borrow().output(), n);
        clock.step();
        assert_eq!(observer.upgrade().unwrap().borrow().output(), n + 1);
    }

    clock.step();
    assert_eq!(observer.upgrade().unwrap().borrow().output(), 20);
}
