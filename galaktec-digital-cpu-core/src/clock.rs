use std::cell::RefCell;
use std::rc::Rc;

/// Represents a clockable component in the system
///
/// Each component will get two signals a clock high and a clock low. On clock high a component should
/// only update any internal state that will NOT change the public API. This allows all components to
/// react to each others changes from the last clock low signal. Otherwise the behavior would change
/// depending on the order the components were clocked.
///
/// Clock low would be the signal to commit the changes so that the public facing API reflects the
/// internal changes created by clock high. Effectively propagates the changes in lock step.
pub trait ClockedHigh {
    fn clock_high(&mut self);
}

pub trait ClockedLow {
    fn clock_low(&mut self);
}

pub struct Clock {
    to_clock_high: Option<Vec<Rc<RefCell<dyn ClockedHigh>>>>,
    to_clock_low: Option<Vec<Rc<RefCell<dyn ClockedLow>>>>,
}

impl Clock {
    pub fn new(
        to_clock_high: Option<Vec<Rc<RefCell<dyn ClockedHigh>>>>,
        to_clock_low: Option<Vec<Rc<RefCell<dyn ClockedLow>>>>,
    ) -> Self {
        Clock {
            to_clock_high,
            to_clock_low,
        }
    }

    pub fn clock(&self) {
        if let Some(ch) = &self.to_clock_high {
            for ch in ch.iter() {
                ch.borrow_mut().clock_high();
            }
        }

        if let Some(cl) = &self.to_clock_low {
            for cl in cl.iter() {
                cl.borrow_mut().clock_low();
            }
        }
    }
}
