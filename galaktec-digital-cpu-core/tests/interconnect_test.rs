use galaktec_digital_cpu_core::{GenericClock, Interconnect, Peripheral, Controller, DiscreteUnit};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::DerefMut;

/* -------------- Counter Peripheral ---------------------- */

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum CounterOperation {
    Set(usize),
    Reset
}

#[derive(Debug)]
struct CounterPeripheral {
    count: usize,
    controller: Rc<RefCell<dyn Controller<CounterOperation, usize>>>
}

impl CounterPeripheral {
    fn new(controller: Rc<RefCell<dyn Controller<CounterOperation, usize>>>) -> Self {
        CounterPeripheral {
            count: 0,
            controller
        }
    }
}

impl DiscreteUnit for CounterPeripheral {
    fn send(&mut self) {}
    fn update(&mut self) {
        let mut controller = self.controller.as_ref().borrow_mut();
        let input = controller.receive();
        self.count = if let Some(op) = input {
            match op {
                CounterOperation::Set(new_value) => new_value,
                CounterOperation::Reset => 0
            }
        } else {
            self.count + 1
        };

        controller.transmit(self.count)
    }
}

/* -------------- Counter Reset Controller ---------------------- */

#[derive(Debug)]
struct CounterResetController {
    trigger_at: usize,
    set_to: usize,
    counter_peripheral: Rc<RefCell<dyn Peripheral<CounterOperation, usize>>>,
}

impl CounterResetController {
    fn new(trigger_at: usize, set_to: usize, counter_peripheral: Rc<RefCell<dyn Peripheral<CounterOperation, usize>>>) -> Self {
        CounterResetController {
            trigger_at,
            set_to,
            counter_peripheral
        }
    }
}

impl DiscreteUnit for CounterResetController {
    fn send(&mut self) {
        let mut peripheral = self.counter_peripheral.as_ref().borrow_mut();
        if let Some(current_count) = peripheral.receive() {
            if current_count == self.trigger_at {
                peripheral.transmit(CounterOperation::Set(self.set_to));
            }
        }
    }
    fn update(&mut self) {}
}

#[test]
fn counter_test() {
    let counter_interconnect: Rc<RefCell<Interconnect<CounterOperation, usize>>> = Rc::new(RefCell::new(Interconnect::new()));
    let counter = Box::new(CounterPeripheral::new(counter_interconnect.clone()));
    let counter_reset = Box::new(CounterResetController::new(10, 20, counter_interconnect.clone()));

    let mut clock = GenericClock::new(vec![counter, counter_reset]);

    for n in 0..11 {
        clock.step();

        let count = Peripheral::receive(counter_interconnect.as_ref().borrow_mut().deref_mut()).unwrap();

        if n == 11 {
            assert_eq!(count, 20);
        } else {
            assert_eq!(count, n + 1);
        }

    }
}
