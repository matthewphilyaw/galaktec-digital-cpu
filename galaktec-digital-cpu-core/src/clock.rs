use std::cell::RefCell;
use std::rc::Rc;

use crate::discrete::Discrete;

type DiscreteDevice = Rc<RefCell<dyn Discrete>>;

#[derive(Debug)]
pub struct GenericClock {
    discrete_items: Vec<DiscreteDevice>,
}

impl GenericClock {
    pub fn new(discrete_items: Vec<DiscreteDevice>) -> Self {
        GenericClock { discrete_items }
    }

    pub fn step(&mut self) {
        for ref mut di in self.discrete_items.iter() {
            di.borrow_mut().activate();
        }

        for ref mut di in self.discrete_items.iter() {
            di.borrow_mut().process_input();
        }

        for ref mut di in self.discrete_items.iter() {
            di.borrow_mut().deactivate();
        }
    }
}
