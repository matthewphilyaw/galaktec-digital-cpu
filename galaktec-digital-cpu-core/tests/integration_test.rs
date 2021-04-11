use std::cell::RefCell;
use std::rc::Rc;

use galaktec_digital_cpu_core::{interconnect, Clock, Controller, Peripheral, Update};

/* -------------- Counter Peripheral ---------------------- */

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum CounterOperation {
    Set(usize),
}

#[derive(Debug)]
struct CounterPeripheral {
    count: usize,
    controller: Controller<CounterOperation, usize>,
}

impl CounterPeripheral {
    fn new(controller: Controller<CounterOperation, usize>) -> Self {
        CounterPeripheral {
            count: 0,
            controller,
        }
    }
}

impl Update for CounterPeripheral {
    fn update(&mut self) {
        let input = self.controller.receive();
        self.count = if let Some(op) = input {
            match op {
                CounterOperation::Set(new_value) => new_value,
            }
        } else {
            self.count + 1
        };

        self.controller.transmit(self.count)
    }
}

/* -------------- Counter Reset Peripheral ---------------------- */

#[derive(Debug)]
struct CounterResetPeripheral {
    trigger_at: usize,
    set_to: usize,
    counter_peripheral: Peripheral<CounterOperation, usize>,
    observed_count: Option<usize>,
}

impl CounterResetPeripheral {
    fn new(
        trigger_at: usize,
        set_to: usize,
        counter_peripheral: Peripheral<CounterOperation, usize>,
    ) -> Self {
        CounterResetPeripheral {
            trigger_at,
            set_to,
            counter_peripheral,
            observed_count: None,
        }
    }
}

impl Update for CounterResetPeripheral {
    fn update(&mut self) {
        self.observed_count = self.counter_peripheral.receive();
        if let Some(current_count) = self.observed_count {
            if current_count == self.trigger_at {
                self.counter_peripheral
                    .transmit(CounterOperation::Set(self.set_to));
            }
        }
    }
}

#[test]
fn counter_before_reset_order_test() {
    let (c, p, i) = interconnect();
    let counter = Rc::new(RefCell::new(CounterPeripheral::new(c)));
    let counter_reset = Rc::new(RefCell::new(CounterResetPeripheral::new(10, 20, p)));
    let mut clock = Clock::new(vec![], vec![counter, counter_reset.clone()]);

    for n in 0..13 {
        clock.step();
        i.borrow_mut().tick();

        match counter_reset.borrow().observed_count {
            Some(count) => {
                if n == 12 {
                    assert_eq!(count, 20);
                } else {
                    assert_eq!(count, n);
                }
            }
            _ => continue,
        }
    }
}

#[test]
fn reset_before_counter_order_test() {
    let (c, p, i) = interconnect();
    let counter = Rc::new(RefCell::new(CounterPeripheral::new(c)));
    let counter_reset = Rc::new(RefCell::new(CounterResetPeripheral::new(10, 20, p)));
    let mut clock = Clock::new(vec![], vec![counter_reset.clone(), counter]);

    for n in 0..13 {
        clock.step();
        i.borrow_mut().tick();

        match counter_reset.borrow().observed_count {
            Some(count) => {
                if n == 12 {
                    assert_eq!(count, 20);
                } else {
                    assert_eq!(count, n);
                }
            }
            _ => continue,
        }
    }
}
