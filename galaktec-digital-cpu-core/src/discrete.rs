use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Broadcast: Debug {
    fn broadcast(&mut self) {}
}

pub trait Update: Debug {
    fn update(&mut self) {}
}

type Broadcastable = Rc<RefCell<dyn Broadcast>>;
type Updatable = Rc<RefCell<dyn Update>>;

#[derive(Debug)]
pub struct Clock {
    broadcast_items: Vec<Broadcastable>,
    update_items: Vec<Updatable>,
}

impl Clock {
    pub fn new(broadcast_items: Vec<Broadcastable>, update_items: Vec<Updatable>) -> Self {
        Clock {
            broadcast_items,
            update_items,
        }
    }

    pub fn step(&mut self) {
        for di in self.broadcast_items.iter_mut() {
            di.as_ref().borrow_mut().broadcast();
        }

        for di in self.update_items.iter_mut() {
            di.as_ref().borrow_mut().update();
        }
    }
}
