use super::{Bus, BusPeripheral, BusState};

use super::super::PeripheralInterface;

impl PeripheralInterface for Bus {
    fn write_req(&mut self, address: u32, value: u32) {
        debug_assert_eq!(
            matches!(self.state, BusState::Idle),
            true,
            "Bus must be in idle state before calling write. Current state is: {:?}",
            self.state
        );

        let index = self.get_peripheral_index(address);
        let adjusted_addr = address - self.bus_peripherals[index].start_address;
        self.bus_peripherals[index]
            .peripheral
            .write_req(adjusted_addr, value);
        self.active_peripheral_index = Some(index);
        self.state = BusState::Waiting;
    }

    fn read_req(&mut self, address: u32) {
        debug_assert_eq!(
            matches!(self.state, BusState::Idle),
            true,
            "Bus must be in idle state before calling read. Current state is: {:?}",
            self.state
        );

        let index = self.get_peripheral_index(address);
        let adjusted_addr = address - self.bus_peripherals[index].start_address;
        self.bus_peripherals[index]
            .peripheral
            .read_req(adjusted_addr);
        self.active_peripheral_index = Some(index);
        self.state = BusState::Waiting;
    }

    fn busy(&self) -> bool {
        if let BusState::Idle = self.state {
            true
        } else {
            false
        }
    }

    fn result(&self) -> u32 {
        debug_assert_eq!(
            matches!(self.state, BusState::Idle),
            true,
            "Bus must be in idle state before calling result. Current state is: {:?}",
            self.state
        );

        self.result
    }
}
