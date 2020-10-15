mod device;
mod clock;

pub trait Discrete {
    fn step(&mut self);
    fn commit(&mut self);
}

pub trait Unit<InternalEvent, ExternalEvent, Result>
where
    Result: Default,
{
    fn step(&self, internal_event_queue: &mut Vec<InternalEvent>);
    fn commit(
        &mut self,
        internal_events: &Vec<InternalEvent>,
        external_events: &Vec<ExternalEvent>,
    ) -> Result;
}

pub trait Communal<ExternalEvent, Result>
where
    Result: Default,
{
    fn add_event(&mut self, external_event: ExternalEvent);
    fn last_result(&self) -> &Result;
}


