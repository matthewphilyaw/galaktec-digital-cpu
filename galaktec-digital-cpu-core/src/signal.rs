use std::cell::Cell;
use std::fmt::Debug;

pub trait SignalData: Debug + Copy + Clone {}
impl<T: Debug + Copy + Clone> SignalData for T {}

#[derive(Debug)]
pub(crate) enum SignalError {
    Busy,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum SignalState {
    Free,
    Set,
    Propagated,
}

#[derive(Debug)]
pub(crate) struct Signal<T: SignalData> {
    next_data: Cell<Option<T>>,
    data: Cell<Option<T>>,
    state: Cell<SignalState>,
}

impl<T: SignalData> Signal<T> {
    pub(crate) fn new() -> Self {
        Signal {
            next_data: Cell::new(None),
            data: Cell::new(None),
            state: Cell::new(SignalState::Free),
        }
    }

    pub(crate) fn set_data(&self, data: T) -> Result<(), SignalError> {
        match self.state.get() {
            SignalState::Free => {
                self.state.set(SignalState::Set);
                self.next_data.set(Some(data));
                Ok(())
            }
            _ => Err(SignalError::Busy),
        }
    }

    pub(crate) fn data(&self) -> Option<T> {
        self.data.take()
    }

    pub(crate) fn set(&self) -> bool {
        if let SignalState::Set = self.state.get() {
            true
        } else {
            false
        }
    }

    pub(crate) fn propagate(&self) {
        if let SignalState::Set = self.state.get() {
            self.state.set(SignalState::Propagated);
            self.data.set(self.next_data.take());
        } else {
            panic!("Invalid state");
        }
    }

    pub(crate) fn reset(&self) {
        self.state.set(SignalState::Free);
        self.next_data.set(None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_data_does_not_change_data_before_propagate_is_called() {
        let signal = Signal::<i32>::new();

        signal.set_data(1).unwrap();
        assert_eq!(signal.data(), None);
    }

    #[test]
    fn set_data_is_propagated_to_data_after_propagate_is_called() {
        let signal = Signal::<i32>::new();

        signal.set_data(1).unwrap();
        assert_eq!(signal.data(), None);

        signal.propagate();
        assert_eq!(signal.data(), Some(1));
    }

    #[test]
    fn set_data_is_propagated_correctly_through_two_cycles() {
        let signal = Signal::<i32>::new();

        signal.set_data(1).unwrap();
        assert_eq!(signal.data(), None);

        signal.propagate();
        signal.reset();
        assert_eq!(signal.data(), Some(1));

        signal.set_data(2).unwrap();
        assert_eq!(signal.data(), None);

        signal.propagate();
        signal.reset();
        assert_eq!(signal.data(), Some(2));
    }

    #[test]
    fn data_set_is_true_when_set_data_was_called_and_propagate_has_not() {
        let signal = Signal::<i32>::new();

        signal.set_data(1).unwrap();
        assert_eq!(signal.data(), None);
        assert_eq!(signal.set(), true);
    }

    #[test]
    fn data_set_is_false_when_propagate_has_been_called() {
        let signal = Signal::<i32>::new();

        signal.set_data(1).unwrap();
        assert_eq!(signal.data(), None);
        assert_eq!(signal.set(), true);

        signal.propagate();
        assert_eq!(signal.data(), Some(1));
        assert_eq!(signal.set(), false);
    }
}
