use crate::{Discrete, StepPhase, StepPhase::*};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct GenericClock {
    discrete_items: Vec<Rc<RefCell<dyn Discrete>>>,
    phases: Vec<StepPhase>,
}

impl GenericClock {
    pub fn new(discrete_items: Vec<Rc<RefCell<dyn Discrete>>>) -> Self {
        GenericClock {
            discrete_items,
            phases: vec![First, Second, Third],
        }
    }

    pub fn step(&mut self) {
        // First run all phases
        for phase in &self.phases {
            for ref mut di in self.discrete_items.iter() {
                di.borrow_mut().step(phase.clone());
            }
        }

        // Now commit all changes
        for ref mut di in self.discrete_items.iter() {
            di.borrow_mut().commit();
        }
    }
}
