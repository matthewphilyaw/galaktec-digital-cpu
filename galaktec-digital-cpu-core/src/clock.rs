use crate::discrete::Discrete;

type DiscreteDevice = Box<dyn Discrete>;

#[derive(Debug)]
pub struct GenericClock {
    discrete_items: Vec<DiscreteDevice>,
}

impl GenericClock {
    pub fn new(discrete_items: Vec<DiscreteDevice>) -> Self {
        GenericClock { discrete_items }
    }

    pub fn step(&mut self) {
        for di in self.discrete_items.iter_mut() {
            di.activate();
        }

        for di in self.discrete_items.iter_mut() {
            di.process_input();
        }

        for di in self.discrete_items.iter_mut() {
            di.deactivate();
        }
    }
}
