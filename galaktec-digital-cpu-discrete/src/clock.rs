use std::rc::Rc;

use crate::Discrete;
use std::cell::RefCell;

struct GenericClock {
    discrete_items: Vec<Rc<RefCell<dyn Discrete>>>
}

impl GenericClock {
    fn new(discrete_items: Vec<Rc<RefCell<dyn Discrete>>>) -> Self {
        GenericClock {
            discrete_items
        }
    }

    fn step(&mut self) {
        // First run all steps
        for ref mut di in self.discrete_items.iter() {
            di.borrow_mut().step();
        }

        // Now commit all changes
        for ref mut di in self.discrete_items.iter() {
            di.borrow_mut().commit();
        }
    }
}