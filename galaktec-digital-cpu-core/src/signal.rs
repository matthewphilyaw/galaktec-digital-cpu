use crate::discrete::Update;
use std::fmt::Debug;

pub trait SignalData: Debug + Copy {}

impl<T: Debug + Copy> SignalData for T {}

pub enum SignalError {
    Busy,
}

#[derive(Debug)]
pub struct Signal<T: SignalData> {
    latency: usize,
    delay_count: usize,
    data: Option<T>,
    next_data: Option<T>,
}

impl<T: SignalData> Signal<T> {
    pub(crate) fn new(latency: usize) -> Self {
        Signal {
            data: None,
            latency,
            delay_count: 0,
            next_data: None,
        }
    }

    pub(crate) fn set_data(&mut self, data: T) -> Result<(), SignalError> {
        if self.next_data.is_some() {
            return Err(SignalError::Busy);
        }

        self.next_data = Some(data);
        self.delay_count = 0;

        Ok(())
    }

    pub(crate) fn data(&self) -> Option<T> {
        self.data
    }
}

impl<T: SignalData> Update for Signal<T> {
    fn update(&mut self) {
        if self.next_data.is_none() {
            return;
        }

        if self.delay_count < self.latency {
            self.delay_count += 1;
            return;
        }

        self.data = std::mem::replace(&mut self.next_data, None);
        self.delay_count = 0;
    }
}
