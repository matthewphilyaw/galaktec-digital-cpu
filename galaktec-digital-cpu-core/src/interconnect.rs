use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use crate::signal::{Signal, SignalData};
use crate::{SignalError, Update};

pub type PeripheralPort<PeripheralInput, PeripheralOutput> = FullDuplexPort<PeripheralOutput, PeripheralInput>;
pub type ControllerPort<PeripheralInput, PeripheralOutput> = FullDuplexPort<PeripheralInput, PeripheralOutput>;

#[derive(Debug)]
pub struct InterconnectState<PeripheralInput: SignalData, PeripheralOutput: SignalData> {
    latency: usize,
    latency_counter: usize,
    input_signal: Rc<RefCell<Signal<PeripheralInput>>>,
    output_signal: Rc<RefCell<Signal<PeripheralOutput>>>,
}

impl<PeripheralInput: SignalData, PeripheralOutput: SignalData> InterconnectState<PeripheralInput, PeripheralOutput> {
    fn new(
        latency: usize,
        input_signal: Rc<RefCell<Signal<PeripheralInput>>>,
        output_signal: Rc<RefCell<Signal<PeripheralOutput>>>,
    ) -> Self {
        InterconnectState {
            latency,
            latency_counter: 0,
            input_signal,
            output_signal,
        }
    }
}

impl<PeripheralInput: SignalData, PeripheralOutput: SignalData> Update
    for InterconnectState<PeripheralInput, PeripheralOutput>
{
    fn update(&mut self) {
        // always update input signal
        if self.input_signal.borrow().set() {
            self.input_signal.borrow_mut().propagate();
            self.input_signal.borrow_mut().reset();
        }

        if !self.output_signal.borrow().set() {
            return;
        }

        if self.latency_counter < self.latency {
            self.latency_counter += 1;
            return;
        }

        self.output_signal.borrow_mut().propagate();
        self.output_signal.borrow_mut().reset();
        self.latency_counter = 0;
    }
}

#[derive(Debug)]
pub struct Interconnect<PeripheralInput: SignalData, PeripheralOutput: SignalData> {
    pub controller_connector: ControllerPort<PeripheralInput, PeripheralOutput>,
    pub peripheral_connector: PeripheralPort<PeripheralInput, PeripheralOutput>,
    pub interconnect_state: InterconnectState<PeripheralInput, PeripheralOutput>,
}

impl<PeripheralInput: SignalData, PeripheralOutput: SignalData> Interconnect<PeripheralInput, PeripheralOutput> {
    pub fn new() -> Self {
        Self::new_with_latency(1)
    }

    pub fn new_with_latency(latency: usize) -> Self {
        assert!(latency > 0, "Latency must be greater than zero");

        let input_signal: Rc<RefCell<Signal<PeripheralInput>>> = Rc::new(RefCell::new(Signal::new()));
        let output_signal: Rc<RefCell<Signal<PeripheralOutput>>> = Rc::new(RefCell::new(Signal::new()));

        let peripheral_connector = PeripheralPort::new(output_signal.clone(), input_signal.clone());
        let controller_connector = ControllerPort::new(input_signal.clone(), output_signal.clone());

        let interconnect_state = InterconnectState::new(latency - 1, input_signal, output_signal);

        Interconnect {
            controller_connector,
            peripheral_connector,
            interconnect_state,
        }
    }
}

impl<PeripheralInput: SignalData, PeripheralOutput: SignalData> Default
    for Interconnect<PeripheralInput, PeripheralOutput>
{
    fn default() -> Self {
        Self::new()
    }
}

pub enum PortError {
    Busy,
}

#[derive(Debug)]
pub struct FullDuplexPort<Transmit: SignalData, Receive: SignalData> {
    transmit_signal: Rc<RefCell<Signal<Transmit>>>,
    receive_signal: Rc<RefCell<Signal<Receive>>>,
}

impl<Transmit: SignalData, Receive: SignalData> FullDuplexPort<Transmit, Receive> {
    pub fn new(transmit_signal: Rc<RefCell<Signal<Transmit>>>, receive_signal: Rc<RefCell<Signal<Receive>>>) -> Self {
        FullDuplexPort {
            transmit_signal,
            receive_signal,
        }
    }

    pub fn transmit(&mut self, output: Transmit) -> Result<(), PortError> {
        if let Err(err) = self.transmit_signal.borrow_mut().set_data(Some(output)) {
            return match err {
                SignalError::Busy => Err(PortError::Busy),
            };
        }

        Ok(())
    }

    pub fn receive(&mut self) -> Option<Receive> {
        self.receive_signal.borrow().data()
    }
}
