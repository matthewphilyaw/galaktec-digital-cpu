use std::fmt::Debug;
use std::rc::Rc;

use crate::latency::CycleDelay;
use crate::signal::{Signal, SignalData, SignalError};
use crate::discrete::Update;

pub type PeripheralPort<PeripheralInput, PeripheralOutput> = Port<PeripheralOutput, PeripheralInput>;
pub type ControllerPort<PeripheralInput, PeripheralOutput> = Port<PeripheralInput, PeripheralOutput>;

#[derive(Debug)]
pub enum PortTransmitError {
    Busy,
}

#[derive(Debug)]
pub enum PortReceiveError {
    NoData,
}

type SignalPair<PeripheralInput, PeripheralOutput> = (
    Rc<Signal<PeripheralInput>>,
    Rc<Signal<PeripheralOutput>>,
);

#[derive(Debug)]
enum HalfDuplexState {
    Write,
    Read,
}

#[derive(Debug)]
pub struct Port<Transmit: SignalData, Receive: SignalData> {
    transmit_signal: Rc<Signal<Transmit>>,
    receive_signal: Rc<Signal<Receive>>,
}

impl<Transmit: SignalData, Receive: SignalData> Port<Transmit, Receive> {
    pub(crate) fn new(
        transmit_signal: Rc<Signal<Transmit>>,
        receive_signal: Rc<Signal<Receive>>,
    ) -> Self {
        Port {
            transmit_signal,
            receive_signal,
        }
    }

    pub fn transmit(&mut self, output: Transmit) -> Result<(), PortTransmitError> {
        if let Err(err) = self.transmit_signal.set_data(output) {
            return match err {
                SignalError::Busy => Err(PortTransmitError::Busy),
            };
        }

        Ok(())
    }

    pub fn receive(&mut self) -> Result<Receive, PortReceiveError> {
        match self.receive_signal.data() {
            Some(data) => Ok(data),
            None => Err(PortReceiveError::NoData),
        }
    }
}

#[derive(Debug)]
pub struct HalfDuplexInterconnect<PeripheralInput: SignalData, PeripheralOutput: SignalData> {
    state: HalfDuplexState,
    cycle_delay: CycleDelay,
    input_signal: Rc<Signal<PeripheralInput>>,
    output_signal: Rc<Signal<PeripheralOutput>>,
}

impl<PeripheralInput: SignalData, PeripheralOutput: SignalData>
    HalfDuplexInterconnect<PeripheralInput, PeripheralOutput>
{
    pub fn new() -> (
        PeripheralPort<PeripheralInput, PeripheralOutput>,
        ControllerPort<PeripheralInput, PeripheralOutput>,
        Self,
    ) {
        Self::new_with_latency(1)
    }

    pub fn new_with_latency(
        delay: usize,
    ) -> (
        PeripheralPort<PeripheralInput, PeripheralOutput>,
        ControllerPort<PeripheralInput, PeripheralOutput>,
        Self,
    ) {
        let (input_signal, output_signal) = create_signal_pair();
        let (peripheral_port, controller_port) = create_ports(&input_signal, &output_signal);

        let interconnect = HalfDuplexInterconnect {
            state: HalfDuplexState::Write,
            cycle_delay: CycleDelay::new(delay),
            input_signal,
            output_signal,
        };

        (peripheral_port, controller_port, interconnect)
    }
}

impl<PeripheralInput: SignalData, PeripheralOutput: SignalData> Update
    for HalfDuplexInterconnect<PeripheralInput, PeripheralOutput>
{
    fn update(&mut self) {
        // always update input signal
        match self.state {
            HalfDuplexState::Write => {
                if !self.input_signal.set() {
                    return;
                }

                self.input_signal.propagate();
                self.state = HalfDuplexState::Read;
            }
            HalfDuplexState::Read => {
                if !self.output_signal.set() {
                    return;
                }

                self.cycle_delay.step();
                if !self.cycle_delay.elapsed() {
                    return;
                }

                self.output_signal.propagate();
                self.output_signal.reset();
                self.input_signal.reset();
                self.cycle_delay.reset();
                self.state = HalfDuplexState::Write;
            }
        }
    }
}

#[derive(Debug)]
pub struct FullDuplexInterconnect<PeripheralInput: SignalData, PeripheralOutput: SignalData> {
    cycle_delay: CycleDelay,
    input_signal: Rc<Signal<PeripheralInput>>,
    output_signal: Rc<Signal<PeripheralOutput>>,
}

impl<PeripheralInput: SignalData, PeripheralOutput: SignalData>
    FullDuplexInterconnect<PeripheralInput, PeripheralOutput>
{
    pub fn new() -> (
        PeripheralPort<PeripheralInput, PeripheralOutput>,
        ControllerPort<PeripheralInput, PeripheralOutput>,
        Self,
    ) {
        Self::new_with_latency(1)
    }

    pub fn new_with_latency(
        delay: usize,
    ) -> (
        PeripheralPort<PeripheralInput, PeripheralOutput>,
        ControllerPort<PeripheralInput, PeripheralOutput>,
        Self,
    ) {
        let (input_signal, output_signal) = create_signal_pair();
        let (peripheral_port, controller_port) = create_ports(&input_signal, &output_signal);

        let interconnect = FullDuplexInterconnect {
            cycle_delay: CycleDelay::new(delay),
            input_signal,
            output_signal,
        };

        (peripheral_port, controller_port, interconnect)
    }
}

impl<PeripheralInput: SignalData, PeripheralOutput: SignalData> Update
    for FullDuplexInterconnect<PeripheralInput, PeripheralOutput>
{
    fn update(&mut self) {
        // always update input signal
        if self.input_signal.set() {
            self.input_signal.propagate();
            self.input_signal.reset();
        }

        if !self.output_signal.set() {
            return;
        }

        self.cycle_delay.step();
        if !self.cycle_delay.elapsed() {
            return;
        }

        self.output_signal.propagate();
        self.output_signal.reset();
        self.cycle_delay.reset();
    }
}

fn create_signal_pair<PeripheralInput: SignalData, PeripheralOutput: SignalData>(
) -> SignalPair<PeripheralInput, PeripheralOutput> {
    (
        Rc::new(Signal::new()),
        Rc::new(Signal::new()),
    )
}

fn create_ports<PeripheralInput: SignalData, PeripheralOutput: SignalData>(
    input_signal: &Rc<Signal<PeripheralInput>>,
    output_signal: &Rc<Signal<PeripheralOutput>>,
) -> (
    PeripheralPort<PeripheralInput, PeripheralOutput>,
    ControllerPort<PeripheralInput, PeripheralOutput>,
) {
    (
        PeripheralPort::new(output_signal.clone(), input_signal.clone()),
        ControllerPort::new(input_signal.clone(), output_signal.clone()),
    )
}
