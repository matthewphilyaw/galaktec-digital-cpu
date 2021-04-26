use std::fmt::Debug;

pub trait SignalData: Debug + Copy {}

impl<T: Debug + Copy> SignalData for T {}

#[derive(Debug)]
pub(crate) enum SignalError {
    Busy,
}

#[derive(Debug, PartialEq)]
pub(crate) enum SignalState {
    Ready,
    Set,
    Propagated,
}

#[derive(Debug)]
pub struct Signal<T: SignalData> {
    data: Option<T>,
    next_data: Option<T>,
    state: SignalState,
}

impl<T: SignalData> Signal<T> {
    pub(crate) fn new() -> Self {
        Signal {
            data: None,
            next_data: None,
            state: SignalState::Ready,
        }
    }

    pub(crate) fn set_data(&mut self, data: Option<T>) -> Result<(), SignalError> {
        match self.state {
            SignalState::Ready => {
                self.next_data = data;
                self.state = SignalState::Set;
                Ok(())
            }
            _ => Err(SignalError::Busy),
        }
    }

    pub(crate) fn data(&self) -> Option<T> {
        self.data
    }

    pub(crate) fn set(&self) -> bool {
        self.state == SignalState::Set
    }

    pub(crate) fn propagate(&mut self) {
        assert_eq!(
            self.state,
            SignalState::Set,
            "Can't propagate a signal that hasn't been set."
        );

        self.data = std::mem::replace(&mut self.next_data, None);
        self.state = SignalState::Propagated;
    }

    pub(crate) fn reset(&mut self) {
        self.next_data = None;
        self.state = SignalState::Ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_data_does_not_change_data_before_propagate_is_called() {
        let mut signal = Signal::<i32>::new();

        signal.set_data(Some(1));
        assert_eq!(signal.data(), None);
    }

    #[test]
    fn set_data_is_propagated_to_data_after_propagate_is_called() {
        let mut signal = Signal::<i32>::new();

        signal.set_data(Some(1));
        assert_eq!(signal.data(), None);

        signal.propagate();
        assert_eq!(signal.data(), Some(1));
    }

    #[test]
    fn set_data_is_propagated_correctly_through_two_cycles() {
        let mut signal = Signal::<i32>::new();

        signal.set_data(Some(1));
        assert_eq!(signal.data(), None);

        signal.propagate();
        signal.reset();
        assert_eq!(signal.data(), Some(1));

        signal.set_data(Some(2));
        assert_eq!(signal.data(), Some(1));

        signal.propagate();
        signal.reset();
        assert_eq!(signal.data(), Some(2));
    }

    #[test]
    fn data_set_is_true_when_set_data_was_called_and_propagate_has_not() {
        let mut signal = Signal::<i32>::new();

        signal.set_data(Some(1));
        assert_eq!(signal.data(), None);
        assert_eq!(signal.set(), true);
    }

    #[test]
    fn data_set_is_false_when_propagate_has_been_called() {
        let mut signal = Signal::<i32>::new();

        signal.set_data(Some(1));
        assert_eq!(signal.data(), None);
        assert_eq!(signal.set(), true);

        signal.propagate();
        assert_eq!(signal.data(), Some(1));
        assert_eq!(signal.set(), false);
    }
}
