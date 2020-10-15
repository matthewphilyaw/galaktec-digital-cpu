use crate::{ Unit, Discrete, Communal };

pub struct GenericDevice<InternalEvent, ExternalEvent, Result> {
    internal_event_queue: Vec<InternalEvent>,
    external_event_queue: Vec<ExternalEvent>,
    unit: Box<dyn Unit<InternalEvent, ExternalEvent, Result>>,
    last_result: Result,
}

impl<InternalEvent, ExternalEvent, Result> GenericDevice<InternalEvent, ExternalEvent, Result>
    where
        Result: Default,
{
    pub fn new(unit: Box<dyn Unit<InternalEvent, ExternalEvent, Result>>) -> Self {
        GenericDevice {
            internal_event_queue: Vec::with_capacity(10),
            external_event_queue: Vec::with_capacity(10),
            unit,
            last_result: Default::default(),
        }
    }
}

impl<InternalEvent, ExternalEvent, Result> Communal<ExternalEvent, Result>
for GenericDevice<InternalEvent, ExternalEvent, Result>
    where
        Result: Default,
{
    fn add_event(&mut self, event: ExternalEvent) {
        self.external_event_queue.push(event);
    }

    fn last_result(&self) -> &Result {
        &self.last_result
    }
}

impl<InternalEvent, ExternalEvent, Result> Discrete
for GenericDevice<InternalEvent, ExternalEvent, Result>
    where
        Result: Default,
{
    fn step(&mut self) {
        // Need to wrap this in state machine only allowing step to be called once
        self.internal_event_queue.clear();
        self.unit.step(&mut self.internal_event_queue);
    }

    fn commit(&mut self) {
        let result = self
            .unit
            .commit(&self.internal_event_queue, &self.external_event_queue);
        self.last_result = result;
        self.internal_event_queue.clear();
        self.external_event_queue.clear();
    }
}