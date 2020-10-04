use super::{Bus, BusState};
use crate::cpu::Clocked;

impl Clocked for Bus {
    fn clock_high(&mut self) {
        debug_assert_ne!(
            matches!(self.state, BusState::Update),
            true,
            "BusState set to Update is invalid on clock high, this is a logic error"
        );

        for periph in self.bus_peripherals.iter_mut() {
            periph.peripheral.clock_high();
        }

        if let BusState::Waiting = self.state {
            debug_assert_ne!(
                matches!(self.active_peripheral_index, None),
                true,
                "There should be an active peripheral, this is a logic error"
            );

            if let Some(index) = self.active_peripheral_index {
                self.state = match self.bus_peripherals[index].peripheral.busy() {
                    true => BusState::Waiting,
                    false => BusState::Update,
                };
            };
        }
    }

    fn clock_low(&mut self) {
        for periph in self.bus_peripherals.iter_mut() {
            periph.peripheral.clock_low();
        }

        if let BusState::Update = self.state {
            debug_assert_ne!(
                matches!(self.active_peripheral_index, None),
                true,
                "There should be an active peripheral, this is a logic error"
            );

            if let Some(index) = self.active_peripheral_index {
                self.result = self.bus_peripherals[index].peripheral.result();
            }
            self.state = BusState::Idle;
        }
    }
}
