use crate::{Discrete, Observable, StepPhase, Unit, EventHandler};
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Device<ExternalEvent, State>
where
    State: Clone + Default + Debug,
    ExternalEvent: Debug
{
    external_event_queue_a: Vec<ExternalEvent>,
    external_event_queue_b: Vec<ExternalEvent>,
    unit: Box<dyn Unit<ExternalEvent, State>>,
    state: State,
}

impl<ExternalEvent, State> Device<ExternalEvent, State>
where
    State: Clone + Default + Debug,
    ExternalEvent: Debug
{
    pub fn new(unit: Box<dyn Unit<ExternalEvent, State>>) -> Self {
        Device {
            external_event_queue_a: Vec::with_capacity(5),
            external_event_queue_b: Vec::with_capacity(5),
            unit,
            state: Default::default(),
        }
    }

    pub fn new_rc_ref(unit: Box<dyn Unit<ExternalEvent, State>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Device::new(unit)))
    }
}

impl<ExternalEvent, State> EventHandler<ExternalEvent> for Device<ExternalEvent, State>
where
    State: Clone + Default + Debug,
    ExternalEvent: Debug
{
    fn add_event(&mut self, event: ExternalEvent) {
        self.external_event_queue_a.push(event);
    }
}

impl<ExternalEvent, State> Observable<State> for Device<ExternalEvent, State>
where
    State: Clone + Default + Debug,
    ExternalEvent: Debug
{
    fn state(&self) -> State {
        self.state.clone()
    }
}

impl<ExternalEvent, State> Discrete for Device<ExternalEvent, State>
where
    State: Clone + Default + Debug,
    ExternalEvent: Debug
{
    fn step(&mut self, phase: StepPhase) {
        self.unit.step(phase.clone(), &self.external_event_queue_b);
        self.external_event_queue_b.clear();

        std::mem::swap(&mut self.external_event_queue_a, &mut self.external_event_queue_b);
    }

    fn commit(&mut self) {
        self.state = self.unit.commit();
    }
}
