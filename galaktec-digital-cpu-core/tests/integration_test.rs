use std::cell::RefCell;
use std::rc::Rc;

use galaktec_digital_cpu_core::{Clock, FullDuplexPort, create_interconnect, Discrete};

/* -------------- Counter Peripheral ---------------------- */

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum CounterOperation {
    Set(usize),
}

#[derive(Debug)]
struct CounterPeripheral {
    count: usize,
    port: FullDuplexPort<CounterOperation, usize>,
}

impl CounterPeripheral {
    fn new(port: FullDuplexPort<CounterOperation, usize>) -> Self {
        CounterPeripheral {
            count: 0,
            port,
        }
    }
}

impl Discrete for CounterPeripheral {
    fn transmit(&mut self) {
        self.port.transmit(self.count)
    }
    fn update(&mut self) {
        let input = self.port.receive();
        self.count = if let Some(op) = input {
            match op {
                CounterOperation::Set(new_value) => new_value,
            }
        } else {
            self.count + 1
        };
    }
}

/* -------------- Counter Reset Peripheral ---------------------- */

#[derive(Debug)]
struct CounterResetPeripheral {
    trigger_at: usize,
    set_to: usize,
    counter_port: FullDuplexPort<usize, CounterOperation>,
    observed_count: Option<usize>,
}

impl CounterResetPeripheral {
    fn new(
        trigger_at: usize,
        set_to: usize,
        counter_port: FullDuplexPort<usize, CounterOperation>,
    ) -> Self {
        CounterResetPeripheral {
            trigger_at,
            set_to,
            counter_port,
            observed_count: None,
        }
    }
}

impl Discrete for CounterResetPeripheral {
    fn transmit(&mut self) {
        if let Some(current_count) = self.observed_count {
            if current_count == self.trigger_at {
                self.counter_port
                    .transmit(CounterOperation::Set(self.set_to));
            }
        }
    }

    fn update(&mut self) {
        self.observed_count = self.counter_port.receive();
    }
}

#[test]
fn counter_before_reset_order_test() {
    let (c, p) = create_interconnect();
    let counter = Rc::new(RefCell::new(CounterPeripheral::new(c)));
    let counter_reset = Rc::new(RefCell::new(CounterResetPeripheral::new(10, 20, p)));
    let mut clock = Clock::new(vec![counter.clone(), counter_reset.clone()]);

    for n in 0..12 {
        clock.step();

        let count = counter.borrow().count;
        if n == 11 {
            assert_eq!(count, 20);
        } else {
            assert_eq!(count, n + 1);
        }
    }
}

#[test]
fn reset_before_counter_order_test() {
    let (c, p) = create_interconnect();
    let counter = Rc::new(RefCell::new(CounterPeripheral::new(c)));
    let counter_reset = Rc::new(RefCell::new(CounterResetPeripheral::new(10, 20, p)));
    let mut clock = Clock::new(vec![counter_reset.clone(), counter.clone()]);

    for n in 0..12 {
        clock.step();

        let count = counter.borrow().count;
        if n == 11 {
            assert_eq!(count, 20);
        } else {
            assert_eq!(count, n + 1);
        }
    }
}
