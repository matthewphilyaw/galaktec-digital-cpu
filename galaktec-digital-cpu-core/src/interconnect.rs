use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use crate::signal::{Signal, SignalData, SignalError};

pub type PeripheralPort<PeripheralInput, PeripheralOutput> =
    FullDuplexPort<PeripheralOutput, PeripheralInput>;
pub type ControllerPort<PeripheralInput, PeripheralOutput> =
    FullDuplexPort<PeripheralInput, PeripheralOutput>;

#[derive(Debug)]
pub struct Interconnect<PeripheralInput: SignalData, PeripheralOutput: SignalData> {
    pub controller_connector: ControllerPort<PeripheralInput, PeripheralOutput>,
    pub peripheral_connector: PeripheralPort<PeripheralInput, PeripheralOutput>,
    pub input_signal: Rc<RefCell<Signal<PeripheralInput>>>,
    pub output_signal: Rc<RefCell<Signal<PeripheralOutput>>>,
}

impl<PeripheralInput: SignalData, PeripheralOutput: SignalData>
    Interconnect<PeripheralInput, PeripheralOutput>
{
    pub fn new() -> Self {
        Self::new_with_latency(0, 0)
    }

    pub fn new_with_latency(input: usize, output: usize) -> Self {
        let input_signal: Rc<RefCell<Signal<PeripheralInput>>> =
            Rc::new(RefCell::new(Signal::new(input)));
        let output_signal: Rc<RefCell<Signal<PeripheralOutput>>> =
            Rc::new(RefCell::new(Signal::new(output)));

        let peripheral_connector = PeripheralPort::new(output_signal.clone(), input_signal.clone());

        let controller_connector = ControllerPort::new(input_signal.clone(), output_signal.clone());

        Interconnect {
            controller_connector,
            peripheral_connector,
            input_signal,
            output_signal,
        }
    }
}

#[derive(Debug)]
pub struct FullDuplexPort<Transmit: SignalData, Receive: SignalData> {
    transmit_signal: Rc<RefCell<Signal<Transmit>>>,
    receive_signal: Rc<RefCell<Signal<Receive>>>,
}

impl<Transmit: SignalData, Receive: SignalData> FullDuplexPort<Transmit, Receive> {
    pub fn new(
        transmit_signal: Rc<RefCell<Signal<Transmit>>>,
        receive_signal: Rc<RefCell<Signal<Receive>>>,
    ) -> Self {
        FullDuplexPort {
            transmit_signal,
            receive_signal,
        }
    }

    pub fn transmit(&mut self, output: Transmit) -> Result<(), SignalError> {
        self.transmit_signal.borrow_mut().set_data(output)
    }

    pub fn receive(&mut self) -> Option<Receive> {
        self.receive_signal.borrow().data().clone()
    }
}
