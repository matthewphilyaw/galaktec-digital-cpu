use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::{Rc};

use galaktec_digital_cpu_core::{Broadcast, Clock, Controller, Interconnect, Peripheral, Update};

/* -------------- Counter Peripheral ---------------------- */

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum CounterOperation {
    Set(usize),
    Reset,
}

#[derive(Debug)]
struct CounterPeripheral {
    count: usize,
    controller: Rc<RefCell<dyn Controller<CounterOperation, usize>>>,
}

impl CounterPeripheral {
    fn new(controller: Rc<RefCell<dyn Controller<CounterOperation, usize>>>) -> Self {
        CounterPeripheral {
            count: 0,
            controller,
        }
    }
}

impl Update for CounterPeripheral {
    fn update(&mut self) {
        let mut controller = self.controller.as_ref().borrow_mut();
        let input = controller.receive();
        self.count = if let Some(op) = input {
            match op {
                CounterOperation::Set(new_value) => new_value,
                CounterOperation::Reset => 0,
            }
        } else {
            self.count + 1
        };

        controller.transmit(self.count)
    }
}

/* -------------- Counter Reset Peripheral ---------------------- */

#[derive(Debug)]
struct CounterResetPeripheral {
    trigger_at: usize,
    set_to: usize,
    counter_peripheral: Rc<RefCell<dyn Peripheral<CounterOperation, usize>>>,
}

impl CounterResetPeripheral {
    fn new(
        trigger_at: usize,
        set_to: usize,
        counter_peripheral: Rc<RefCell<dyn Peripheral<CounterOperation, usize>>>,
    ) -> Self {
        CounterResetPeripheral {
            trigger_at,
            set_to,
            counter_peripheral,
        }
    }
}

impl Broadcast for CounterResetPeripheral {
    fn broadcast(&mut self) {
        let mut peripheral = self.counter_peripheral.as_ref().borrow_mut();
        if let Some(current_count) = peripheral.receive() {
            if current_count == self.trigger_at {
                peripheral.transmit(CounterOperation::Set(self.set_to));
            }
        }
    }
}

#[test]
fn counter_test() {
    let counter_interconnect: Rc<RefCell<Interconnect<CounterOperation, usize>>> =
        Rc::new(RefCell::new(Interconnect::new()));
    let counter = Rc::new(RefCell::new(CounterPeripheral::new(
        counter_interconnect.clone(),
    )));
    let counter_reset = Rc::new(RefCell::new(CounterResetPeripheral::new(
        10,
        20,
        counter_interconnect.clone(),
    )));

    let mut clock = Clock::new(vec![counter_reset], vec![counter]);

    for n in 0..11 {
        clock.step();

        let count =
            Peripheral::receive(counter_interconnect.as_ref().borrow_mut().deref_mut()).unwrap();

        if n == 11 {
            assert_eq!(count, 20);
        } else {
            assert_eq!(count, n + 1);
        }
    }
}
