use crate::cpu::Clocked;

#[derive(Debug)]
enum BusState {
    Waiting,
    Update,
    Idle,
}

pub struct BusPeripheral {
    start_address: u32,
    end_address: u32,
    write_latency: usize,
    read_latency: usize,
    bussed_peripheral: Box<dyn BussedPeripheral>,
}

impl BusPeripheral {
    fn new(
        start_address: u32,
        end_address: u32,
        write_latency: usize,
        read_latency: usize,
        bussed_peripheral: Box<dyn BussedPeripheral>,
    ) -> Self {
        BusPeripheral {
            start_address,
            end_address,
            write_latency,
            read_latency,
            bussed_peripheral,
        }
    }
}

pub struct Bus {
    state: BusState,
    bus_peripherals: Vec<BusPeripheral>,
    operation: Option<Box<dyn FnMut(&mut Vec<BusPeripheral>) -> u32>>,
    result: u32,
    latency_counter: usize,
}

impl Clocked for Bus {
    fn clock_high(&mut self) {
        debug_assert_ne!(
            matches!(self.state, BusState::Update),
            true,
            "BusState set to Update is invalid on clock high, this is a logic error"
        );

        if let BusState::Waiting = self.state {
            debug_assert_ne!(
                self.latency_counter,
                0,
                "Latency counter can not be zero at this point in the waiting state, this is a logic error"
            );

            self.latency_counter -= 1;
            if self.latency_counter == 0 {
                debug_assert_ne!(
                    matches!(self.operation, None),
                    true,
                    "Operation MUST be defined on bus update"
                );

                if let Some(f) = &mut self.operation {
                    self.result = (f)(&mut self.bus_peripherals);
                    self.state = BusState::Update;
                }
            }
        }
    }

    fn clock_low(&mut self) {
        if let BusState::Update = self.state {
            self.operation = None;
            self.state = BusState::Idle;
        }
    }
}

impl Bus {
    pub fn new(peripherals: Vec<BusPeripheral>) -> Self {
        Bus {
            state: BusState::Idle,
            operation: None,
            bus_peripherals: peripherals,
            result: 0,
            latency_counter: 0,
        }
    }

    fn get_peripheral_options(&mut self, address: u32) -> (usize, usize, usize) {
        let (index, peripheral) = self
            .bus_peripherals
            .iter()
            .enumerate()
            .find(|&(_size, p)| p.start_address <= address && address <= p.end_address)
            .unwrap();

        (index, peripheral.read_latency, peripheral.write_latency)
    }

    fn set_wait_state(
        &mut self,
        latency: usize,
        operation: Box<dyn FnMut(&mut Vec<BusPeripheral>) -> u32>,
    ) {
        self.latency_counter = latency;
        self.operation = Some(operation);
        self.state = BusState::Waiting;
    }

    pub fn write(&mut self, address: u32, value: u32) {
        debug_assert_eq!(
            matches!(self.state, BusState::Idle),
            true,
            "Bus must be in idle state before calling write. Current state is: {:?}",
            self.state
        );

        let (index, _read_latency, write_latency) = self.get_peripheral_options(address);
        let operation = Box::new(move |p: &mut Vec<BusPeripheral>| {
            p[index].bussed_peripheral.write(address, value);
            0 // return zero, in the future this might return an error code on write
        });

        self.set_wait_state(write_latency, operation);
    }

    pub fn read(&mut self, address: u32) {
        debug_assert_eq!(
            matches!(self.state, BusState::Idle),
            true,
            "Bus must be in idle state before calling read. Current state is: {:?}",
            self.state
        );

        let (index, read_latency, _write_latency) = self.get_peripheral_options(address);
        let operation =
            Box::new(move |p: &mut Vec<BusPeripheral>| p[index].bussed_peripheral.read(address));

        self.set_wait_state(read_latency, operation);
    }

    pub fn result(&self) -> u32 {
        debug_assert_eq!(
            matches!(self.state, BusState::Idle),
            true,
            "Bus must be in idle state before calling result. Current state is: {:?}",
            self.state
        );

        self.result
    }

    pub fn idle(&self) -> bool {
        if let BusState::Idle = self.state {
            true
        } else {
            false
        }
    }
}

pub trait BussedPeripheral {
    fn write(&mut self, address: u32, value: u32);
    fn read(&mut self, address: u32) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct SimpleTestBusPeripheral {
        value: Option<u32>,
    }

    impl BussedPeripheral for SimpleTestBusPeripheral {
        fn write(&mut self, _address: u32, value: u32) {
            self.value = Some(value);
        }
        fn read(&mut self, _address: u32) -> u32 {
            self.value.unwrap()
        }
    }

    pub struct WordBusPeripheral {
        storage: Vec<u32>,
    }

    impl WordBusPeripheral {
        fn new(initial_values: Vec<u32>) -> Self {
            WordBusPeripheral {
                storage: initial_values
            }
        }
    }

    impl BussedPeripheral for WordBusPeripheral {
        fn write(&mut self, address: u32, value: u32) {
            self.storage[address as usize] = value;
        }

        fn read(&mut self, address: u32) -> u32 {
            self.storage[address as usize]
        }
    }

    fn create_word_bus_peripheral_with_address(initial_value: Vec<u32>, address: u32) -> BusPeripheral {
        let length = initial_value.len() - 1;
        let bussed_peripheral = WordBusPeripheral {
            storage: initial_value,
        };

        BusPeripheral::new(
            address,
            address + (length as u32),
            1,
            1,
            Box::new(bussed_peripheral),
        )
    }

    // End address is always four words past start
    fn create_bus_peripheral_with_start_address(
        address: u32,
        initial_value: Option<u32>
    ) -> BusPeripheral {
        let bussed_peripheral = SimpleTestBusPeripheral {
            value: initial_value,
        };

        BusPeripheral::new(
            address,
            address + 3,
            1,
            1,
            Box::new(bussed_peripheral),
        )
    }

    fn create_simple_bus_peripheral(
        initial_value: Option<u32>,
        write_latency: usize,
        read_latency: usize,
    ) -> BusPeripheral {
        let bussed_peripheral = SimpleTestBusPeripheral {
            value: initial_value,
        };

        BusPeripheral::new(
            0,
            3,
            write_latency,
            read_latency,
            Box::new(bussed_peripheral),
        )
    }

    fn create_simple_bus(
        initial_value: Option<u32>,
        write_latency: usize,
        read_latency: usize,
    ) -> Bus {
        let bus_peripheral =
            create_simple_bus_peripheral(initial_value, write_latency, read_latency);
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

    #[test]
    fn can_write_to_two_different_peripherals() {
        let bus_periph_1 = create_bus_peripheral_with_start_address(0, None);
        let bus_periph_2 = create_bus_peripheral_with_start_address(4, None);

        let mut bus = Bus::new(vec![
            bus_periph_1,
            bus_periph_2
        ]);

        bus.write(0, 1234);
        clock_bus(&mut bus);

        bus.write(4, 4567);
        clock_bus(&mut bus);

        let periph1_val = bus.bus_peripherals[0].bussed_peripheral.read(0);
        let periph2_val = bus.bus_peripherals[1].bussed_peripheral.read(0);

        assert_eq!(periph1_val, 1234);
        assert_eq!(periph2_val, 4567);
    }

    #[test]
    fn can_read_from_two_different_peripherals() {
        let bus_periph_1 = create_bus_peripheral_with_start_address(0, Some(1234));
        let bus_periph_2 = create_bus_peripheral_with_start_address(4, Some(4567));

        let mut bus = Bus::new(vec![
            bus_periph_1,
            bus_periph_2
        ]);

        bus.read(0);
        clock_bus(&mut bus);

        let periph1_val = bus.result;

        bus.read(4);
        clock_bus(&mut bus);

        let periph2_val = bus.result;

        assert_eq!(periph1_val, 1234);
        assert_eq!(periph2_val, 4567);
    }

    #[test]
    fn can_write_to_multiple_addresses_within_peripheral() {
        let periph = create_word_bus_peripheral_with_address(
            vec![0; 2],
            0,
        );

        let mut bus = Bus::new(vec![periph]);

        bus.write(0, 12345);
        clock_bus(&mut bus);

        bus.write(1, 54321);
        clock_bus(&mut bus);

        let write_1 = bus.bus_peripherals[0].bussed_peripheral.read(0);
        let write_2 = bus.bus_peripherals[0].bussed_peripheral.read(1);

        assert_eq!(write_1, 12345);
        assert_eq!(write_2, 54321);
    }

}
