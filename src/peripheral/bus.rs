#[derive(Debug)]
pub enum Error {
    AlreadyAcquired,
    AlreadyReleased,
    NoOperationForAddress,
    ResultNotReady,
    OperationNotReady,
}

#[derive(Debug)]
pub enum Operation {
    Write { address: u32, data: u32 },
    Read { address: u32 },
}

pub struct Bus {
    operation: Option<Operation>,
    operation_result: Option<u32>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            operation: None,
            operation_result: None,
        }
    }

    pub fn acquire(&mut self, operation: Operation) -> Result<(), Error> {
        if let None = self.operation {
            self.operation = Some(operation);
            Ok(())
        } else {
            Err(Error::AlreadyAcquired)
        }
    }

    pub fn release(&mut self, result: Option<u32>) -> Result<(), Error> {
        if let None = self.operation {
            Err(Error::AlreadyReleased)
        } else {
            self.operation_result = result;
            self.operation = None;
            Ok(())
        }
    }

    pub fn get_operation(&self, start_address: u32, end_address: u32) -> Result<&Operation, Error> {
        if let Some(operation) = &self.operation {
            match operation {
                &Operation::Write { address, .. } | &Operation::Read { address }
                    if start_address <= address && address <= end_address =>
                {
                    Ok(&operation)
                }
                _ => Err(Error::NoOperationForAddress),
            }
        } else {
            Err(Error::OperationNotReady)
        }
    }

    pub fn get_operation_result(&self) -> Result<Option<u32>, Error> {
        if let None = self.operation {
            Ok(self.operation_result)
        } else {
            Err(Error::ResultNotReady)
        }
    }

    pub fn acquired(&self) -> bool {
        if let None = self.operation {
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

        let result = bus.acquire(Operation::Write {
            address: 123,
            data: 456,
        });

        assert_eq!(matches!(result, Ok(())), true);
    }

    #[test]
    fn cant_acquire_bus_for_write_if_acquired() {
        let mut bus = Bus::new();

        let result_one = bus.acquire(Operation::Write {
            address: 123,
            data: 456,
        });

        assert_eq!(matches!(result_one, Ok(())), true);

        let result_two = bus.acquire(Operation::Write {
            address: 123,
            data: 456,
        });

        assert_eq!(matches!(result_two, Err(Error::AlreadyAcquired)), true);
    }

    #[test]
    fn can_acquire_bus_for_read_if_not_acquired() {
        let mut bus = Bus::new();

        let result = bus.acquire(Operation::Read { address: 123 });
        assert_eq!(matches!(result, Ok(())), true);
    }

    #[test]
    fn cant_acquire_bus_for_read_if_acquired() {
        let mut bus = Bus::new();

        let result_one = bus.acquire(Operation::Read { address: 123 });
        assert_eq!(matches!(result_one, Ok(())), true);

        let result_two = bus.acquire(Operation::Read { address: 123 });
        assert_eq!(matches!(result_two, Err(Error::AlreadyAcquired)), true);
    }

    #[test]
    fn bus_reports_being_acquired_for_write_op() {
        let mut bus = Bus::new();

        let result_one = bus.acquire(Operation::Write {
            address: 123,
            data: 123,
        });
        assert_eq!(matches!(result_one, Ok(())), true);
        assert_eq!(bus.acquired(), true);
    }

    #[test]
    fn bus_reports_being_acquired_for_read_op() {
        let mut bus = Bus::new();

        let result_one = bus.acquire(Operation::Read { address: 123 });
        assert_eq!(matches!(result_one, Ok(())), true);
        assert_eq!(bus.acquired(), true);
    }

    #[test]
    fn get_operation_returns_if_write_operation_within_range() {
        let mut bus = Bus::new();

        let result = bus.acquire(Operation::Write {
            address: 1,
            data: 1
        });

        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(0, 2);
        assert_eq!(matches!(operation, Ok(Operation::Write {
            address: 1,
            data: 1
        })), true);
    }

    #[test]
    fn get_operation_returns_if_operation_operation_within_range() {
        let mut bus = Bus::new();

        let result = bus.acquire(Operation::Read { address: 1 });
        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(0, 2);
        assert_eq!(matches!(operation, Ok(Operation::Read { address: 1 })), true);
    }

    #[test]
    fn get_operation_returns_if_address_same_as_start_address() {
        let mut bus = Bus::new();

        let result = bus.acquire(Operation::Read { address: 0 });
        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(0, 1);
        assert_eq!(matches!(operation, Ok(Operation::Read { address: 0 })), true);
    }

    #[test]
    fn get_operation_returns_if_address_same_as_end_address_inclusive() {
        let mut bus = Bus::new();

        let result = bus.acquire(Operation::Read { address: 2 });
        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(0, 2);
        assert_eq!(matches!(operation, Ok(Operation::Read { address: 2 })), true);
    }

    #[test]
    fn get_operation_returns_if_address_within_range() {
        let mut bus = Bus::new();

        let result = bus.acquire(Operation::Read { address: 1 });
        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(0, 2);
        assert_eq!(matches!(operation, Ok(Operation::Read { address: 1 })), true);
    }

    #[test]
    fn get_operation_returns_no_address() {
        let mut bus = Bus::new();

        let result = bus.acquire(Operation::Read { address: 1 });
        assert_eq!(matches!(result, Ok(())), true);

        let operation = bus.get_operation(2, 4);

        assert_eq!(matches!(operation, Err(Error::NoOperationForAddress)), true)
    }

    #[test]
    fn get_operation_returns_operation_not_ready() {
        let mut bus = Bus::new();

        let operation = bus.get_operation(0, 1);
        assert_eq!(matches!(operation, Err(Error::OperationNotReady)), true)
    }

    #[test]
    fn get_operation_result_returns_result_set() {
        let mut bus = Bus::new();

        bus.acquire(Operation::Read { address: 1 });
        bus.release(Some(123));

        let result = bus.get_operation_result();
        assert_eq!(matches!(result, Ok(Some(123))), true);
    }

    #[test]
    fn get_operation_result_returns_result_not_ready() {
        let mut bus = Bus::new();

        bus.acquire(Operation::Read { address: 1 });
        let result = bus.get_operation_result();
        assert_eq!(matches!(result, Err(Error::ResultNotReady)), true);
    }

    #[test]
    fn acquired_is_false_after_release() {
        let mut bus = Bus::new();

        bus.acquire(Operation::Read {
            address: 1
        });

        assert_eq!(bus.acquired(), true);
        bus.release(None);
        assert_eq!(bus.acquired(), false);
    }

    #[test]
    fn release_sets_operation_result() {
        let mut bus = Bus::new();

        let result_one = bus.get_operation_result();
        assert_eq!(matches!(result_one, Ok(None)), true);

        bus.acquire(Operation::Read { address: 1 });
        bus.release(Some(1));

        let result_two = bus.get_operation_result();
        assert_eq!(matches!(result_two, Ok(Some(1))), true);
    }

    #[test]
    fn release_will_report_already_released_if_called_after_acquiring_and_release_cycle() {
        let mut bus = Bus::new();

        let result_one = bus.get_operation_result();
        assert_eq!(matches!(result_one, Ok(None)), true);

        bus.acquire(Operation::Read { address: 1 });
        bus.release(Some(1));

        let result_two = bus.get_operation_result();
        assert_eq!(matches!(result_two, Ok(Some(1))), true);

        let result_three = bus.release(None);
        assert_eq!(matches!(result_three, Err(Error::AlreadyReleased)), true);
    }

    #[test]
    fn release_will_report_already_released_if_called_after_initialization() {
        let mut bus = Bus::new();

        let result = bus.release(None);
        assert_eq!(matches!(result, Err(Error::AlreadyReleased)), true);
    }
}
