pub mod clock;
pub mod device;

pub use crate::clock::GenericClock;
pub use crate::device::Device;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum StepPhase {
    First,
    Second,
    Third,
}

pub trait Discrete: Debug {
    fn step(&mut self, phase: StepPhase);
    fn commit(&mut self);
}

pub trait Unit<ExternalEvent, State>: Debug
where
    State: Clone + Default + Debug,
    ExternalEvent: Debug,
{
    fn step(&mut self, phase: StepPhase, external_event_queue: &Vec<ExternalEvent>);
    fn commit(&mut self) -> State;
}

pub trait EventHandler<ExternalEvent>: Debug
where
    ExternalEvent: Debug,
{
    fn add_event(&mut self, event: ExternalEvent);
}

pub trait Observable<State>: Debug
where
    State: Clone + Default + Debug,
{
    fn state(&self) -> State;
}
