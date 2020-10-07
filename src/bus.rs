use crate::clock::{ClockedHigh, ClockedLow};

use self::{BusState::*, Error::*, Operation::*};

enum BusState {
    Acquiring,
    Acquired,
    Releasing,
    Released,
}

#[derive(Debug)]
pub enum Error {
    BusOperationFailed,
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Write { address: u32, data: u32 },
    Read { address: u32 },
}

pub struct Bus {
    state: BusState,
    operation: Option<Operation>,
    operation_result: Option<u32>,
}

impl ClockedLow for Bus {
    fn clock_low(&mut self) {
        match self.state {
            Acquiring => self.state = Acquired,
            Releasing => self.state = Released,
            _ => (),
        }
    }
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            state: BusState::Released,
            operation: None,
            operation_result: None,
        }
    }

    pub fn acquire(&mut self, operation: Operation) -> Result<(), Error> {
        if let Released = self.state {
            self.operation = Some(operation);
            self.state = Acquiring;
            Ok(())
        } else {
            Err(BusOperationFailed)
        }
    }

    pub fn release(&mut self, result: Option<u32>) -> Result<(), Error> {
        if let Acquired = self.state {
            self.operation_result = result;
            self.operation = None;
            self.state = Releasing;
            Ok(())
        } else {
            Err(BusOperationFailed)
        }
    }

    pub fn get_operation(&self, start_addr: u32, end_addr: u32) -> Result<Operation, Error> {
        if let Acquired = self.state {
            let op = self
                .operation
                .expect("The bus operation should be set when in the acquired state");
            match op {
                Write { address, .. } | Read { address }
                    if start_addr <= address && address <= end_addr =>
                {
                    Ok(op)
                }
                _ => Err(BusOperationFailed),
            }
        } else {
            Err(BusOperationFailed)
        }
    }

    pub fn get_operation_result(&self) -> Result<Option<u32>, Error> {
        if let Released = self.state {
            Ok(self.operation_result)
        } else {
            Err(BusOperationFailed)
        }
    }

    pub fn acquired(&self) -> bool {
        if let Released = self.state {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialized_bus_should_not_be_acquired() {
        let bus = Bus::new();
        assert_eq!(bus.acquired(), false);
    }

    #[test]
    fn initialized_bus_should_not_have_result() {
        let bus = Bus::new();

        let result = bus.get_operation_result();
        assert_eq!(matches!(result, Ok(None)), true);
    }

    #[test]
    fn can_acquire_bus_for_write_if_not_acquired() {
        let mut bus = Bus::new();

        let result = bus.acquire(Write {
            address: 123,
            data: 456,
        });

        assert_eq!(matches!(result, Ok(())), true);
    }

    #[test]
    fn cant_acquire_bus_for_write_if_acquired() {
        let mut bus = Bus::new();

        let result_one = bus.acquire(Write {
            address: 123,
            data: 456,
        });

        assert_eq!(matches!(result_one, Ok(())), true);

        let result_two = bus.acquire(Write {
            address: 123,
            data: 456,
        });

        assert_eq!(matches!(result_two, Err(BusOperationFailed)), true);
    }

    #[test]
    fn can_acquire_bus_for_read_if_not_acquired() {
        let mut bus = Bus::new();

        let result = bus.acquire(Read { address: 123 });
        assert_eq!(matches!(result, Ok(())), true);
    }

    #[test]
    fn cant_acquire_bus_for_read_if_acquired() {
        let mut bus = Bus::new();

        let result_one = bus.acquire(Read { address: 123 });
        assert_eq!(matches!(result_one, Ok(())), true);

        let result_two = bus.acquire(Read { address: 123 });
        assert_eq!(matches!(result_two, Err(BusOperationFailed)), true);
    }

    #[test]
    fn bus_reports_being_acquired_for_write_op() {
        let mut bus = Bus::new();

        let result_one = bus.acquire(Write {
            address: 123,
            data: 123,
        });
        assert_eq!(matches!(result_one, Ok(())), true);
        assert_eq!(bus.acquired(), true);
    }

    #[test]
    fn bus_reports_being_acquired_for_read_op() {
        let mut bus = Bus::new();

        let result_one = bus.acquire(Read { address: 123 });
        assert_eq!(matches!(result_one, Ok(())), true);
        assert_eq!(bus.acquired(), true);
    }

    #[test]
    fn get_operation_returns_if_write_operation_within_range() {
        let mut bus = Bus::new();

        let op = Write {
            address: 1,
            data: 1,
        };
        let result = bus.acquire(op);
        bus.clock_low();

        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(0, 2);
        bus.clock_low();
        assert_eq!(matches!(operation, Ok(op)), true);
    }

    #[test]
    fn get_operation_returns_if_operation_operation_within_range() {
        let mut bus = Bus::new();

        let op = Read { address: 1 };
        let result = bus.acquire(op);
        bus.clock_low();
        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(0, 2);
        bus.clock_low();
        assert_eq!(matches!(operation, Ok(op)), true);
    }

    #[test]
    fn get_operation_returns_if_address_same_as_start_address() {
        let mut bus = Bus::new();

        let op = Read { address: 0 };
        let result = bus.acquire(op);
        bus.clock_low();
        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(0, 1);
        bus.clock_low();
        assert_eq!(matches!(operation, Ok(op)), true);
    }

    #[test]
    fn get_operation_returns_if_address_same_as_end_address_inclusive() {
        let mut bus = Bus::new();

        let op = Read { address: 2 };
        let result = bus.acquire(op);
        bus.clock_low();
        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(0, 2);
        bus.clock_low();
        assert_eq!(matches!(operation, Ok(op)), true);
    }

    #[test]
    fn get_operation_returns_if_address_within_range() {
        let mut bus = Bus::new();

        let op = Read { address: 1 };
        let result = bus.acquire(op);
        bus.clock_low();
        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(0, 2);
        bus.clock_low();
        assert_eq!(matches!(operation, Ok(op)), true);
    }

    #[test]
    fn get_operation_fails_if_in_wrong_state() {
        let mut bus = Bus::new();

        let result = bus.acquire(Read { address: 1 });
        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(2, 4);

        assert_eq!(matches!(operation, Err(BusOperationFailed)), true)
    }

    #[test]
    fn get_operation_fails_if_called_without_bus_being_acquired() {
        let mut bus = Bus::new();

        let operation = bus.get_operation(0, 1);
        assert_eq!(matches!(operation, Err(BusOperationFailed)), true)
    }

    #[test]
    fn get_operation_result_returns_result_set() {
        let mut bus = Bus::new();

        bus.acquire(Read { address: 1 });
        bus.clock_low();
        bus.release(Some(123));
        bus.clock_low();

        let result = bus.get_operation_result();
        assert_eq!(matches!(result, Ok(Some(123))), true);
    }

    #[test]
    fn get_operation_result_returns_result_not_ready() {
        let mut bus = Bus::new();

        bus.acquire(Read { address: 1 });
        let result = bus.get_operation_result();
        assert_eq!(matches!(result, Err(BusOperationFailed)), true);
    }

    #[test]
    fn acquired_is_false_after_release() {
        let mut bus = Bus::new();

        bus.acquire(Read { address: 1 });
        bus.clock_low();

        assert_eq!(bus.acquired(), true);

        bus.release(None);
        bus.clock_low();
        assert_eq!(bus.acquired(), false);
    }

    #[test]
    fn release_sets_operation_result() {
        let mut bus = Bus::new();

        let result_one = bus.get_operation_result();
        assert_eq!(matches!(result_one, Ok(None)), true);

        bus.acquire(Read { address: 1 });
        bus.clock_low();

        bus.release(Some(1));
        bus.clock_low();

        let result_two = bus.get_operation_result();
        assert_eq!(matches!(result_two, Ok(Some(1))), true);
    }

    #[test]
    fn release_will_fail_if_called_after_acquire_and_release_cycle() {
        let mut bus = Bus::new();

        let result_one = bus.get_operation_result();
        assert_eq!(matches!(result_one, Ok(None)), true);

        bus.acquire(Read { address: 1 });
        bus.clock_low();

        bus.release(Some(1));
        bus.clock_low();

        let result_two = bus.get_operation_result();
        assert_eq!(matches!(result_two, Ok(Some(1))), true);

        let result_three = bus.release(None);
        assert_eq!(matches!(result_three, Err(BusOperationFailed)), true);
    }

    #[test]
    fn release_will_fail_if_called_after_initialization() {
        let mut bus = Bus::new();

        let result = bus.release(None);
        assert_eq!(matches!(result, Err(BusOperationFailed)), true);
    }
}

/*
   clock (+ high, - low)
   - 1
   h acquire
   l state change to acquired
   - 2
   h memory get_operation
   l memory - update state possibly burning latency
   - 3
   h memory release bus set to releasing
   l bus transition to released
   - 4
   h cpu access bus value
   l x
   -
*/
