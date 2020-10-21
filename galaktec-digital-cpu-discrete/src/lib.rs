pub mod clock;

pub use crate::clock::GenericClock;
use std::fmt::Debug;

pub trait React: Debug {
    type Event: Clone + Debug;

    fn react(&mut self, event: Self::Event);
}

pub trait Observable: Debug {
    type State: Clone + Default + Debug;

    fn state(&self) -> Self::State;
}

pub trait ReactiveDevice: Debug + React + Observable {}

pub trait DiscreteDevice: Debug {
    fn activate(&mut self);
    fn settle(&mut self);
    fn deactivate(&mut self);
}
