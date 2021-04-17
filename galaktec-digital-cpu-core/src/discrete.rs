use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Discrete: Debug {
    fn transmit(&mut self) {}
    fn update(&mut self) {}
}

type DiscreteItem = Rc<RefCell<dyn Discrete>>;

#[derive(Debug)]
pub struct Clock {
    discrete_items: Vec<DiscreteItem>,
}

impl Clock {
    pub fn new(discrete_items: Vec<DiscreteItem>) -> Self {
        Clock {
            discrete_items,
        }
    }

    pub fn step(&mut self) {
        for di in self.discrete_items.iter_mut() {
            di.borrow_mut().transmit();
        }

        for di in self.discrete_items.iter_mut() {
            di.borrow_mut().update();
        }
    }
}
