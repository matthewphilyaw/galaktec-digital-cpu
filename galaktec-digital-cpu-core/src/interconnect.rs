use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use crate::signal::{Signal, SignalData, SignalError};

pub type PeripheralConnector<PeripheralInput, PeripheralOutput> =
    FullDuplexPort<PeripheralOutput, PeripheralInput>;
pub type ControllerConnector<PeripheralInput, PeripheralOutput> =
    FullDuplexPort<PeripheralInput, PeripheralOutput>;

pub enum InterconnectOption {
    ZeroLatency,
    Latent { input: usize, output: usize },
}

#[derive(Debug)]
pub struct Interconnect<PeripheralInput: SignalData, PeripheralOutput: SignalData> {
    pub controller_connector: ControllerConnector<PeripheralInput, PeripheralOutput>,
    pub peripheral_connector: PeripheralConnector<PeripheralInput, PeripheralOutput>,
    pub input_signal: Rc<RefCell<Signal<PeripheralInput>>>,
    pub output_signal: Rc<RefCell<Signal<PeripheralOutput>>>,
}

impl<PeripheralInput: SignalData, PeripheralOutput: SignalData>
    Interconnect<PeripheralInput, PeripheralOutput>
{
    pub fn new(option: InterconnectOption) -> Self {
        let (input_latency, output_latency) = match option {
            InterconnectOption::ZeroLatency => (0, 0),
            InterconnectOption::Latent { input, output } => (input, output),
        };

        let input_signal: Rc<RefCell<Signal<PeripheralInput>>> =
            Rc::new(RefCell::new(Signal::new(input_latency)));
        let output_signal: Rc<RefCell<Signal<PeripheralOutput>>> =
            Rc::new(RefCell::new(Signal::new(output_latency)));

        let peripheral_connector =
            PeripheralConnector::new(output_signal.clone(), input_signal.clone());

        let controller_connector =
            ControllerConnector::new(input_signal.clone(), output_signal.clone());

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
