use galaktec_digital_cpu_discrete::{
    DiscreteDevice, GenericClock, Observable, React, ReactiveDevice,
};

use galaktec_digital_cpu_core::{Discrete, GenericIODevice};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
enum CounterOperation {
    Set(usize),
    Reset,
}

#[derive(Debug)]
struct Counter {
    io_device: GenericIODevice<CounterOperation, usize>,
    temp_count: usize,
}

impl Counter {
    fn new(io_device: GenericIODevice<CounterOperation, usize>) -> Self {
        Counter {
            io_device,
            temp_count: 0,
        }
    }
}

impl Discrete for Counter {
    fn activate(&mut self) {
        self.count += 1;
    }

    fn process_input(&mut self) {
        let io = &self.io_device;
        for ev in io.events() {
            match ev {
                CounterOperation::Set(value) => self.count = *value,
                CounterOperation::Reset => self.count = 0,
            }
        }
    }

    fn deactivate(&mut self) {
        self.io_device.borrow_mut().set_data(self.temp_count);
        self.io_device.borrow_mut().clear_events();
    }
}

type CounterDevice = Rc<RefCell<dyn ReactiveDevice<Event = CounterOperation, State = usize>>>;
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

    fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

impl DiscreteDevice for CounterReset {
    fn activate(&mut self) {
        let current_count = self.counter_device.borrow().state();

        if current_count == self.trigger_at {
            self.counter_device
                .borrow_mut()
                .react(CounterOperation::Set(self.set_to));
        }
    }

    fn settle(&mut self) {}

    fn deactivate(&mut self) {}
}

#[test]
fn counter_test() {
    let counter = Counter::new().into_rc();
    let counter_reset = CounterReset::new(10, 20, counter.clone()).into_rc();

    let mut clock = GenericClock::new(vec![counter.clone(), counter_reset]);

    for n in 0..11 {
        println!("counter before step {}: {}", n, counter.borrow().state());
        clock.step();
        println!("counter after step {}: {}", n, counter.borrow().state());
    }

    assert_eq!(counter.borrow().count, 20);
}
