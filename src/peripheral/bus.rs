use crate::cpu::Clocked;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Eq, PartialEq)]
enum BusState {
    BusyWrite,
    BusyRead,
    Idle
}

pub struct BusPeripheral {
    start_address: u32,
    end_address: u32,
    write_latency: usize,
    read_latency: usize,
    bussed_peripheral: Box<dyn BussedPeripheral>
}

impl BusPeripheral {
    fn new(
        start_address: u32,
        end_address: u32,
        write_latency: usize,
        read_latency: usize,
        bussed_peripheral: Box<dyn BussedPeripheral>
    ) -> Self {
        BusPeripheral {
            start_address,
            end_address,
            write_latency,
            read_latency,
            bussed_peripheral
        }
    }
}

pub struct Bus {
    state: BusState,
    bus_peripherals: Vec<BusPeripheral>,
    active_peripheral_index: usize,
    active_address: u32,
    active_write_value: u32,
    result: u32,
    latency_counter: usize
}

impl Clocked for Bus {
    fn clock_high(&mut self) {
        if self.state == BusState::Idle {
            return;
        }

        self.latency_counter -= 1;
        println!("lat: {}", self.latency_counter);
    }

    fn clock_low(&mut self) {
        if self.state == BusState::Idle || self.latency_counter > 0 {
            return;
        }

        let mut peripheral = &mut self.bus_peripherals[self.active_peripheral_index];
        match self.state {
            BusState::BusyWrite => peripheral.bussed_peripheral.write(
                self.active_address,
                self.active_write_value
            ),
            BusState::BusyRead => {
                self.result = peripheral.bussed_peripheral.read(
                    self.active_address
                );
            },
            _ => panic!("Not valid state on on idle")
        };

        self.latency_counter = 0;
        self.state = BusState::Idle;
    }
}

impl Bus {
    pub fn new(peripherals: Vec<BusPeripheral>) -> Self {
        Bus {
            state: BusState::Idle,
            bus_peripherals: peripherals,
            active_peripheral_index: 0,
            active_address: 0,
            active_write_value: 0,
            result: 0,
            latency_counter: 0
        }
    }

    fn activate_peripheral(&mut self, address: u32) {
        debug_assert_ne!(
            self.state,
            BusState::Idle,
            "Bus state must already be set to either BusyRead or BusyWrite before entering this function"
        );

        let (index, peripheral) = self.bus_peripherals
            .iter()
            .enumerate()
            .find(|&(size, p)|
                p.start_address <= address && address <= p.end_address
            )
            .unwrap();


        self.latency_counter = if self.state == BusState::BusyRead {
            peripheral.read_latency
        } else {
            peripheral.write_latency
        };

        self.active_address = address;
        self.active_peripheral_index = index;
    }

    pub fn write(&mut self, address: u32, value: u32) {
        debug_assert_eq!(
            self.state,
            BusState::Idle,
            "Bus must be in idle state before calling write. Current state is: {:?}",
            self.state
        );

        self.active_write_value = value;
        self.state = BusState::BusyWrite;

        self.activate_peripheral(address);
    }

    pub fn read(&mut self, address: u32) {
        debug_assert_eq!(
            self.state,
            BusState::Idle,
            "Bus must be in idle state before calling read. Current state is: {:?}",
            self.state
        );

        self.state = BusState::BusyRead;
        self.activate_peripheral(address);
    }

    pub fn result(&self) -> u32 {
        debug_assert_eq!(
            self.state,
            BusState::Idle,
            "Bus must be in idle state before calling result. Current state is: {:?}",
            self.state
        );

        self.result
    }

    pub fn idle(&self) -> bool {
        self.state == BusState::Idle
    }
}

pub trait BussedPeripheral {
    fn write(&mut self, address: u32, value: u32);
    fn read(&mut self, address: u32) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestBusPeripheral {
        value: Option<u32>
    }

    impl BussedPeripheral for TestBusPeripheral {
        fn write(&mut self, _address: u32, value: u32) {
            self.value = Some(value);
        }
        fn read(&mut self, _address: u32) -> u32 {
            self.value.unwrap()
        }
    }

    fn create_simple_bus_peripheral(initial_value: Option<u32>, write_latency: usize, read_latency: usize) -> BusPeripheral {
        let bussed_peripheral = TestBusPeripheral {
            value: initial_value
        };

        BusPeripheral::new(
           0,
            4,
            write_latency,
            read_latency,
            Box::new(bussed_peripheral)
        )
    }

    fn create_simple_bus(initial_value: Option<u32>, write_latency: usize, read_latency: usize) -> Bus {
        let bus_peripheral = create_simple_bus_peripheral(initial_value, write_latency, read_latency);
        Bus::new(vec![bus_peripheral])
    }

    fn clock_bus(bus: &mut Bus) {
        bus.clock_high();
        bus.clock_low();
    }

    #[test]
    fn can_write_to_peripheral() {
        let mut bus = create_simple_bus(None, 1, 1);
        bus.write(0, 123);
        clock_bus(&mut bus);

        let value = bus.bus_peripherals[0].bussed_peripheral.read(0);
        assert_eq!(value, 123);
    }

    #[test]
    fn can_read_from_peripheral() {
        let mut bus = create_simple_bus(Some(123), 1, 1);
        bus.read(0);
        clock_bus(&mut bus);

        let value = bus.result;
        assert_eq!(value, 123);
    }

    #[test]
    fn value_written_can_be_read_back() {
        let mut bus = create_simple_bus(None, 1, 1);
        bus.write(0, 3456);
        clock_bus(&mut bus);

        bus.read(0);
        clock_bus(&mut bus);

        let value = bus.result;
        assert_eq!(value, 3456);
    }

    #[test]
    fn read_doesnt_appear_on_result_till_after_single_clock_cycles() {
        let mut bus = create_simple_bus(None, 1, 1);
        bus.write(0, 3456);
        clock_bus(&mut bus);

        let result_read_one = bus.result();
        // initialized to zero
        assert_eq!(result_read_one, 0);

        bus.read(0);
        clock_bus(&mut bus);

        let result_read_two = bus.result();

        assert_ne!(result_read_one, result_read_two);
        assert_eq!(result_read_two, 3456);
    }

    #[test]
    fn read_doesnt_appear_on_result_till_after_three_clock_cycles() {
        let mut bus = create_simple_bus(None, 1, 3);
        bus.write(0, 3456);
        clock_bus(&mut bus);

        let result_read_one = bus.result();
        // initialized to zero
        assert_eq!(result_read_one, 0);

        bus.read(0);
        clock_bus(&mut bus);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        let result_read_two = bus.result();

        assert_ne!(result_read_one, result_read_two);
        assert_eq!(result_read_two, 3456);
    }

    #[test]
    #[should_panic]
    fn calling_result_while_not_idle_panics() {
        let mut bus = create_simple_bus(Some(0), 1, 1);

        bus.read(0);
        assert_eq!(bus.idle(), false);
        bus.result();
    }

    #[test]
    #[should_panic]
    fn calling_result_while_not_idle_panics_before_three_clock_cycles() {
        let mut bus = create_simple_bus(Some(0), 1, 3);

        assert_eq!(bus.idle(), true);
        bus.read(0);
        assert_eq!(bus.idle(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.idle(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.idle(), false);
        bus.result();
    }

    #[test]
    fn bus_not_idle_on_write_until_single_clock_passes() {
        let mut bus = create_simple_bus(Some(0), 1, 1);

        assert_eq!(bus.idle(), true);
        bus.write(0, 123);
        assert_eq!(bus.idle(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.idle(), true);
    }

    #[test]
    fn bus_not_idle_on_read_until_single_clock_passes() {
        let mut bus = create_simple_bus(Some(0), 1, 1);

        assert_eq!(bus.idle(), true);
        bus.read(0);
        assert_eq!(bus.idle(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.idle(), true);
    }

    #[test]
    fn bus_not_idle_on_write_until_3_clock_passes() {
        let mut bus = create_simple_bus(Some(0), 3, 1);

        assert_eq!(bus.idle(), true);
        bus.write(0, 123);
        assert_eq!(bus.idle(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.idle(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.idle(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.idle(), true);
    }

    #[test]
    fn bus_not_idle_on_read_until_3_clock_passes() {
        let mut bus = create_simple_bus(Some(0), 1, 3);

        assert_eq!(bus.idle(), true);
        bus.read(0);
        assert_eq!(bus.idle(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.idle(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.idle(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.idle(), true);
    }
}