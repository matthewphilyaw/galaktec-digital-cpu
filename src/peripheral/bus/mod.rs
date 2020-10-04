use super::PeripheralInterface;
use crate::cpu::Clocked;

mod clock_interface;
mod peripheral_interface;

#[derive(Debug)]
enum BusState {
    Waiting,
    Update,
    Idle,
}

pub struct BusPeripheral {
    start_address: u32,
    end_address: u32,
    peripheral: Box<dyn PeripheralInterface>,
}

impl BusPeripheral {
    fn new(start_address: u32, end_address: u32, peripheral: Box<dyn PeripheralInterface>) -> Self {
        BusPeripheral {
            start_address,
            end_address,
            peripheral,
        }
    }
}

pub struct Bus {
    state: BusState,
    bus_peripherals: Vec<BusPeripheral>,
    active_peripheral_index: Option<usize>,
    result: u32,
}

impl Bus {
    pub fn new(peripherals: Vec<BusPeripheral>) -> Self {
        Bus {
            state: BusState::Idle,
            bus_peripherals: peripherals,
            active_peripheral_index: None,
            result: 0,
        }
    }

    fn get_peripheral_index(&mut self, address: u32) -> usize {
        self.bus_peripherals
            .iter()
            .position(|p| p.start_address <= address && address <= p.end_address)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct WordBusPeripheral {
        storage: Vec<u32>,
        write_latency: usize,
        read_latency: usize,

        busy_latency: usize,
        busy: bool,
        result: u32,
    }

    impl WordBusPeripheral {
        fn new(initial_values: Vec<u32>, write_latency: usize, read_latency: usize) -> Self {
            WordBusPeripheral {
                storage: initial_values,
                write_latency,
                read_latency,
                busy_latency: 0,
                busy: false,
                result: 0,
            }
        }
    }

    impl Clocked for WordBusPeripheral {
        fn clock_high(&mut self) {
            if self.busy {
                if self.busy_latency > 0 {
                    self.busy_latency -= 1
                }
            }
        }

        fn clock_low(&mut self) {
            if self.busy && self.busy_latency == 0 {
                self.busy = false;
            }
        }
    }

    impl PeripheralInterface for WordBusPeripheral {
        fn write_req(&mut self, address: u32, value: u32) {
            assert_eq!(self.busy(), false, "Can't write if peripheral is busy");

            self.storage[address as usize] = value;
            self.busy_latency = self.write_latency;
            self.busy = true;
        }

        fn read_req(&mut self, address: u32) {
            assert_eq!(self.busy(), false, "Can't read if peripheral is busy");

            self.result = self.storage[address as usize];
            self.busy_latency = self.read_latency;
            self.busy = true;
        }

        fn busy(&self) -> bool {
            self.busy
        }

        fn result(&self) -> u32 {
            assert_eq!(
                self.busy(),
                false,
                "Can't read result while peripheral is busy"
            );
            self.result
        }
    }

    fn create_word_bus_peripheral_with_address(
        initial_value: Vec<u32>,
        address: u32,
    ) -> BusPeripheral {
        let length = initial_value.len() - 1;
        let bussed_peripheral = WordBusPeripheral::new(initial_value, 1, 1);

        BusPeripheral::new(
            address,
            address + (length as u32),
            Box::new(bussed_peripheral),
        )
    }

    // End address is always four words past start
    fn create_bus_peripheral_with_start_address(
        address: u32,
        initial_value: Vec<u32>,
    ) -> BusPeripheral {
        let bussed_peripheral = WordBusPeripheral::new(initial_value, 1, 1);

        BusPeripheral::new(address, address + 3, Box::new(bussed_peripheral))
    }

    fn create_simple_bus_peripheral(
        initial_value: Vec<u32>,
        write_latency: usize,
        read_latency: usize,
    ) -> BusPeripheral {
        let bussed_peripheral = WordBusPeripheral::new(initial_value, write_latency, read_latency);

        BusPeripheral::new(0, 3, Box::new(bussed_peripheral))
    }

    fn create_simple_bus(
        initial_value: Vec<u32>,
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
        let mut bus = create_simple_bus(vec![0], 1, 1);
        bus.write_req(0, 123);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        bus.bus_peripherals[0].peripheral.read_req(0);
        bus.bus_peripherals[0].peripheral.clock_high();
        bus.bus_peripherals[0].peripheral.clock_low();
        let value = bus.bus_peripherals[0].peripheral.result();
        assert_eq!(value, 123);
    }

    #[test]
    fn can_read_from_peripheral() {
        let mut bus = create_simple_bus(vec![123], 1, 1);
        bus.read_req(0);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        let value = bus.result();
        assert_eq!(value, 123);
    }

    #[test]
    fn value_written_can_be_read_back() {
        let mut bus = create_simple_bus(vec![0], 1, 1);
        bus.write_req(0, 3456);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        bus.read_req(0);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        let value = bus.result();
        assert_eq!(value, 3456);
    }

    #[test]
    fn read_doesnt_appear_on_result_till_after_single_clock_cycles() {
        let mut bus = create_simple_bus(vec![0], 1, 1);
        bus.write_req(0, 3456);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        let result_read_one = bus.result;
        // initialized to zero
        assert_eq!(result_read_one, 0);

        bus.read_req(0);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        let result_read_two = bus.result;

        assert_ne!(result_read_one, result_read_two);
        assert_eq!(result_read_two, 3456);
    }

    #[test]
    fn read_doesnt_appear_on_result_till_after_three_clock_cycles() {
        let mut bus = create_simple_bus(vec![0], 1, 3);
        bus.write_req(0, 3456);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        let result_read_one = bus.result;
        // initialized to zero
        assert_eq!(result_read_one, 0);

        bus.read_req(0);
        clock_bus(&mut bus);
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
        let mut bus = create_simple_bus(vec![0], 1, 1);

        bus.read_req(0);
        assert_eq!(bus.busy(), false);
        bus.result();
    }

    #[test]
    #[should_panic]
    fn calling_result_while_not_idle_panics_before_three_clock_cycles() {
        let mut bus = create_simple_bus(vec![0], 1, 3);

        assert_eq!(bus.busy(), true);
        bus.read_req(0);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        bus.result();
    }

    #[test]
    fn bus_not_idle_on_write_until_single_clock_passes() {
        let mut bus = create_simple_bus(vec![0], 1, 1);

        assert_eq!(bus.busy(), true);
        bus.write_req(0, 123);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.busy(), true);
    }

    #[test]
    fn bus_not_idle_on_read_until_single_clock_passes() {
        let mut bus = create_simple_bus(vec![0], 1, 1);

        assert_eq!(bus.busy(), true);
        bus.read_req(0);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.busy(), true);
    }

    #[test]
    fn bus_not_idle_on_write_until_3_clock_passes() {
        let mut bus = create_simple_bus(vec![0], 2, 1);

        assert_eq!(bus.busy(), true);
        bus.write_req(0, 123);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.busy(), true);
    }

    #[test]
    fn bus_not_idle_on_read_until_3_clock_passes() {
        let mut bus = create_simple_bus(vec![0], 1, 2);

        assert_eq!(bus.busy(), true);
        bus.read_req(0);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.busy(), false);
        clock_bus(&mut bus);
        assert_eq!(bus.busy(), true);
    }

    #[test]
    fn can_write_to_two_different_peripherals() {
        let bus_periph_1 = create_bus_peripheral_with_start_address(0, vec![0]);
        let bus_periph_2 = create_bus_peripheral_with_start_address(4, vec![0]);

        let mut bus = Bus::new(vec![bus_periph_1, bus_periph_2]);

        bus.write_req(0, 1234);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        bus.write_req(4, 4567);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        bus.bus_peripherals[0].peripheral.read_req(0);
        bus.bus_peripherals[0].peripheral.clock_high();
        bus.bus_peripherals[0].peripheral.clock_low();

        bus.bus_peripherals[1].peripheral.read_req(0);
        bus.bus_peripherals[1].peripheral.clock_high();
        bus.bus_peripherals[1].peripheral.clock_low();

        let periph1_val = bus.bus_peripherals[0].peripheral.result();
        let periph2_val = bus.bus_peripherals[1].peripheral.result();

        assert_eq!(periph1_val, 1234);
        assert_eq!(periph2_val, 4567);
    }

    #[test]
    fn can_read_from_two_different_peripherals() {
        let bus_periph_1 = create_bus_peripheral_with_start_address(0, vec![1234]);
        let bus_periph_2 = create_bus_peripheral_with_start_address(4, vec![4567]);

        let mut bus = Bus::new(vec![bus_periph_1, bus_periph_2]);

        bus.read_req(0);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        let periph1_val = bus.result();

        bus.read_req(4);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        let periph2_val = bus.result();

        assert_eq!(periph1_val, 1234);
        assert_eq!(periph2_val, 4567);
    }

    #[test]
    fn can_write_to_multiple_addresses_within_peripheral() {
        let periph = create_word_bus_peripheral_with_address(vec![0; 2], 0);

        let mut bus = Bus::new(vec![periph]);

        bus.write_req(0, 12345);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        bus.bus_peripherals[0].peripheral.read_req(0);
        bus.bus_peripherals[0].peripheral.clock_high();
        bus.bus_peripherals[0].peripheral.clock_low();
        let write_1 = bus.bus_peripherals[0].peripheral.result();

        bus.write_req(1, 54321);
        clock_bus(&mut bus);
        clock_bus(&mut bus);

        bus.bus_peripherals[0].peripheral.read_req(1);
        bus.bus_peripherals[0].peripheral.clock_high();
        bus.bus_peripherals[0].peripheral.clock_low();
        let write_2 = bus.bus_peripherals[0].peripheral.result();

        assert_eq!(write_1, 12345);
        assert_eq!(write_2, 54321);
    }
}
