mod interface;
mod peripheral;

use self::{BusState::*, DataWidth::*, Error::*, Operation::*};

#[derive(Debug, PartialEq)]
enum BusState {
    Acquiring,
    Acquired,
    Releasing,
    Released,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    BusOperationFailed,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DataWidth {
    Byte,
    HalfWord,
    Word,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operation {
    Write {
        data_width: DataWidth,
        address: u32,
        data: u32,
    },
    Read {
        data_width: DataWidth,
        address: u32,
    },
}

#[derive(Debug)]
pub struct Bus {
    state: BusState,
    operation: Option<Operation>,
    operation_result: Option<u32>,
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
        if self.state != Released {
            return Err(BusOperationFailed);
        }

        self.operation = Some(operation);
        self.state = Acquiring;
        Ok(())
    }

    pub fn release(&mut self, result: Option<u32>) -> Result<(), Error> {
        if self.state != Acquired {
            return Err(BusOperationFailed);
        }

        self.operation_result = result;
        self.operation = None;
        self.state = Released;
        Ok(())
    }

    pub fn operation_for_address_range(
        &mut self,
        start_address: u32,
        block_size: u32,
    ) -> Result<Operation, Error> {
        if self.state != Acquiring {
            return Err(BusOperationFailed);
        }

        let op = self
            .operation
            .expect("The bus operation should be set when in the acquired state");

        let end_address = start_address + block_size;
        match op {
            Write { address, .. } | Read { address, .. }
                if start_address <= address && address < end_address =>
            {
                self.state = Acquired;
                Ok(op)
            }
            _ => Err(BusOperationFailed),
        }
    }

    pub fn operation_result(&self) -> Result<Option<u32>, Error> {
        if self.state != Released {
            return Err(BusOperationFailed);
        }

        Ok(self.operation_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::Clock;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn initialized_bus_should_not_have_result() {
        let bus = Bus::new();

        let result = bus.operation_result();
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn can_acquire_bus_for_write_if_not_acquired() {
        let mut bus = Bus::new();

        let result = bus.acquire(Write {
            data_width: Word,
            address: 123,
            data: 456,
        });

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn cant_acquire_bus_for_write_if_acquired() {
        let mut bus = Bus::new();

        let op = Write {
            data_width: Word,
            address: 123,
            data: 456,
        };
        let result_one = bus.acquire(op);

        assert_eq!(result_one, Ok(()));

        let result_two = bus.acquire(op);

        assert_eq!(result_two, Err(BusOperationFailed));
    }

    #[test]
    fn can_acquire_bus_for_read_if_not_acquired() {
        let mut bus = Bus::new();

        let result = bus.acquire(Read {
            data_width: Word,
            address: 123,
        });
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn cant_acquire_bus_for_read_if_acquired() {
        let mut bus = Bus::new();

        let op = Read {
            data_width: Word,
            address: 123,
        };
        let result_one = bus.acquire(op);
        assert_eq!(result_one, Ok(()));

        let result_two = bus.acquire(op);
        assert_eq!(result_two, Err(BusOperationFailed));
    }

    #[test]
    fn operation_returns_if_write_operation_within_range() {
        let mut bus = Bus::new();

        let op = Write {
            data_width: Word,
            address: 1,
            data: 1,
        };
        let result = bus.acquire(op);

        assert_eq!(result, Ok(()));

        let operation = bus.operation_for_address_range(0, 2);
        assert_eq!(operation, Ok(op));
    }

    #[test]
    fn get_operation_returns_if_operation_operation_within_range() {
        let mut bus = Bus::new();

        let op = Read {
            data_width: Word,
            address: 1,
        };
        let result = bus.acquire(op);
        assert_eq!(result, Ok(()));

        let operation = bus.operation_for_address_range(0, 2);
        assert_eq!(operation, Ok(op));
    }

    #[test]
    fn get_operation_returns_if_address_same_as_start_address() {
        let mut bus = Bus::new();

        let op = Read {
            data_width: Word,
            address: 0,
        };
        let result = bus.acquire(op);
        assert_eq!(result, Ok(()));

        let operation = bus.operation_for_address_range(0, 1);
        assert_eq!(operation, Ok(op));
    }

    #[test]
    fn get_operation_fails_if_address_same_as_end_address() {
        let mut bus = Bus::new();

        let op = Read {
            data_width: Word,
            address: 2,
        };
        let result = bus.acquire(op);
        assert_eq!(result, Ok(()));

        let operation = bus.operation_for_address_range(0, 2);
        assert_eq!(operation, Err(BusOperationFailed));
    }

    #[test]
    fn get_operation_returns_if_address_within_range() {
        let mut bus = Bus::new();

        let op = Read {
            data_width: Word,
            address: 1,
        };
        let result = bus.acquire(op);
        assert_eq!(result, Ok(()));

        let operation = bus.operation_for_address_range(0, 2);
        assert_eq!(operation, Ok(op));
    }

    #[test]
    fn get_operation_fails_if_address_not_in_range() {
        let mut bus = Bus::new();

        let result = bus.acquire(Read {
            data_width: Word,
            address: 1,
        });
        assert_eq!(result, Ok(()));

        let operation = bus.operation_for_address_range(2, 4);

        assert_eq!(operation, Err(BusOperationFailed));
    }

    #[test]
    fn get_operation_fails_if_called_without_bus_being_acquired() {
        let mut bus = Bus::new();

        let operation = bus.operation_for_address_range(0, 1);
        assert_eq!(operation, Err(BusOperationFailed));
    }

    #[test]
    fn get_operation_result_returns_result_set() {
        let mut bus = Bus::new();

        bus.acquire(Read {
            data_width: Word,
            address: 1,
        });
        let _ = bus.operation_for_address_range(0, 2);
        bus.release(Some(123));

        let result = bus.operation_result();
        assert_eq!(result, Ok(Some(123)));
    }

    #[test]
    fn get_operation_result_returns_result_not_ready() {
        let mut bus = Bus::new();

        bus.acquire(Read {
            data_width: Word,
            address: 1,
        });
        let result = bus.operation_result();
        assert_eq!(result, Err(BusOperationFailed));
    }

    #[test]
    fn release_sets_operation_result() {
        let mut bus = Bus::new();

        let result_one = bus.operation_result();
        assert_eq!(matches!(result_one, Ok(None)), true);

        bus.acquire(Read {
            data_width: Word,
            address: 1,
        });
        let _ = bus.operation_for_address_range(0, 2);
        bus.release(Some(1));

        let result_two = bus.operation_result();
        assert_eq!(result_two, Ok(Some(1)));
    }

    #[test]
    fn release_will_fail_if_called_after_acquire_and_release_cycle() {
        let mut bus = Bus::new();

        let result_one = bus.operation_result();
        assert_eq!(result_one, Ok(None));

        bus.acquire(Read {
            data_width: Word,
            address: 1,
        });
        let _ = bus.operation_for_address_range(0, 2);
        bus.release(Some(1));

        let result_two = bus.operation_result();
        assert_eq!(result_two, Ok(Some(1)));

        let result_three = bus.release(None);
        assert_eq!(result_three, Err(BusOperationFailed));
    }

    #[test]
    fn release_will_fail_if_called_after_initialization() {
        let mut bus = Bus::new();

        let result = bus.release(None);
        assert_eq!(result, Err(BusOperationFailed));
    }
}
