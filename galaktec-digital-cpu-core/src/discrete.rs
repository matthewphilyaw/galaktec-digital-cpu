use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Weak;

pub trait Input<Event>: Debug
where
    Event: Debug + Clone + PartialEq,
{
    fn push(&mut self, event: Event) -> bool;
}

pub trait Output<Data>: Debug
where
    Data: Debug + Default + Clone,
{
    fn output(&self) -> Data;
}

pub trait IODevice<Event, Data>: Debug + Input<Event> + Output<Data>
where
    Event: Debug + Clone + PartialEq,
    Data: Debug + Default + Clone,
{
}

pub trait WithIO<Event, Data>: Debug
where
    Event: Debug + Clone + PartialEq,
    Data: Debug + Default + Clone,
{
    fn io(&self) -> Weak<RefCell<GenericIODevice<Event, Data>>>;
}

pub trait Discrete: Debug {
    fn activate(&mut self);
    fn process_input(&mut self);
    fn deactivate(&mut self);
}

#[derive(Debug)]
pub struct GenericIODevice<Event, Data>
where
    Event: Debug + Clone + PartialEq,
    Data: Debug + Default + Clone,
{
    event_vec: Vec<Event>,
    data: Data,
}

impl<Event, Data> GenericIODevice<Event, Data>
where
    Event: Debug + Clone + PartialEq,
    Data: Debug + Default + Clone,
{
    pub fn new() -> Self {
        GenericIODevice {
            event_vec: vec![],
            data: Data::default(),
        }
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.event_vec
    }

    pub fn clear_events(&mut self) {
        self.event_vec.clear();
    }

    pub fn set_data(&mut self, data: Data) {
        self.data = data;
    }
}

impl<Event, Data> Input<Event> for GenericIODevice<Event, Data>
where
    Event: Debug + Clone + PartialEq,
    Data: Debug + Default + Clone,
{
    fn push(&mut self, event: Event) -> bool {
        if self.event_vec.contains(&event) {
            return false;
        }

        self.event_vec.push(event);
        return true;
    }
}

impl<Event, Data> Output<Data> for GenericIODevice<Event, Data>
where
    Event: Debug + Clone + PartialEq,
    Data: Debug + Default + Clone,
{
    fn output(&self) -> Data {
        self.data.clone()
    }
}

impl<Event, Data> IODevice<Event, Data> for GenericIODevice<Event, Data>
where
    Event: Debug + Clone + PartialEq,
    Data: Debug + Default + Clone,
{
}
