use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use crate::Update;

pub trait SignalData: Debug + Copy {}
impl<T: Debug + Copy> SignalData for T {}

pub enum SignalError {
    Busy
}

#[derive(Debug)]
pub struct Signal<T: SignalData> {
    latency: usize,
    delay_count: usize,
    data: Option<T>,
    next_data: Option<T>
}

impl<T: SignalData> Signal<T> {
    fn new(latency: usize) -> Self {
        Signal { data: None, latency, delay_count: 0, next_data: None }
    }

    fn set_data(&mut self, data: T) -> Result<(), SignalError> {
        if self.next_data.is_some() {
            return Err(SignalError::Busy);
        }

        self.next_data = Some(data);
        self.delay_count = 0;

        Ok(())
    }

    fn data(&self) -> Option<T> {
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

#[derive(Debug)]
pub struct FullDuplexPort<Transmit: SignalData, Receive: SignalData> {
    receive_signal: Rc<RefCell<Signal<Receive>>>,
    transmit_signal: Rc<RefCell<Signal<Transmit>>>,
}

impl<Transmit: SignalData, Receive: SignalData> FullDuplexPort<Transmit, Receive> {
    pub fn transmit(&mut self, output: Transmit) -> Result<(), SignalError> {
        self.transmit_signal.borrow_mut().set_data(output)
    }

    pub fn receive(&mut self) -> Option<Receive> {
        self.receive_signal.borrow().data().clone()
    }
}

pub type PeripheralConnector<PeripheralInput, PeripheralOutput> =
    FullDuplexPort<PeripheralOutput, PeripheralInput>;
pub type ControllerConnector<PeripheralInput, PeripheralOutput> =
    FullDuplexPort<PeripheralInput, PeripheralOutput>;

pub fn create_latent_interconnect<PeripheralInput: SignalData, PeripheralOutput: SignalData>(
    input_latency: usize,
    output_latency: usize
) -> (
    PeripheralConnector<PeripheralInput, PeripheralOutput>,
    ControllerConnector<PeripheralInput, PeripheralOutput>,
    (Rc<RefCell<Signal<PeripheralInput>>>, Rc<RefCell<Signal<PeripheralOutput>>>)
) {
    let signal_a = Rc::new(RefCell::new(Signal::new(input_latency)));
    let signal_b = Rc::new(RefCell::new(Signal::new(output_latency)));

    (
        PeripheralConnector {
            receive_signal: signal_a.clone(),
            transmit_signal: signal_b.clone(),
        },
        ControllerConnector {
            receive_signal: signal_b.clone(),
            transmit_signal: signal_a.clone(),
        },
        (signal_a, signal_b)
    )
}

pub fn create_zero_latency_interconnect<PeripheralInput: SignalData, PeripheralOutput: SignalData>() -> (
    PeripheralConnector<PeripheralInput, PeripheralOutput>,
    ControllerConnector<PeripheralInput, PeripheralOutput>,
    (Rc<RefCell<Signal<PeripheralInput>>>, Rc<RefCell<Signal<PeripheralOutput>>>)
) {
    create_latent_interconnect(0,0)
}
