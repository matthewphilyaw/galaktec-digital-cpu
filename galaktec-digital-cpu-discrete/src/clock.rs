use crate::DiscreteDevice;
use std::cell::RefCell;
use std::rc::Rc;

type Discrete = RefCell<dyn DiscreteDevice>;

#[derive(Debug)]
pub struct GenericClock {
    discrete_items: Vec<Rc<Discrete>>,
}

impl GenericClock {
    pub fn new(discrete_items: Vec<Rc<Discrete>>) -> Self {
        GenericClock { discrete_items }
    }

    pub fn step(&mut self) {
        for ref mut di in self.discrete_items.iter() {
            di.borrow_mut().activate();
        }

        for ref mut di in self.discrete_items.iter() {
            di.borrow_mut().settle();
        }

        for ref mut di in self.discrete_items.iter() {
            di.borrow_mut().deactivate();
        }
    }
}
