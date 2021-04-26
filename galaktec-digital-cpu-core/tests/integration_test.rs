use galaktec_digital_cpu_core::{transmit, update, ControllerPort, Interconnect, PeripheralPort, Transmit, Update};

/* -------------- Counter Peripheral ---------------------- */

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum CounterOperation {
    Set(usize),
}

#[derive(Debug)]
struct CounterPeripheral {
    count: usize,
    connector: PeripheralPort<CounterOperation, usize>,
}

impl CounterPeripheral {
    fn new(connector: PeripheralPort<CounterOperation, usize>) -> Self {
        CounterPeripheral { count: 0, connector }
    }
}

impl Transmit for CounterPeripheral {
    fn transmit(&mut self) {
        self.connector.transmit(self.count);
    }
}

impl Update for CounterPeripheral {
    fn update(&mut self) {
        let input = self.connector.receive();
        self.count = if let Some(op) = input {
            match op {
                CounterOperation::Set(new_value) => new_value,
            }
        } else {
            self.count + 1
        };
    }
}

/* -------------- Counter Reset Peripheral ---------------------- */

#[derive(Debug)]
struct CounterResetPeripheral {
    trigger_at: usize,
    set_to: usize,
    counter_controller: ControllerPort<CounterOperation, usize>,
    observed_count: Option<usize>,
}

impl CounterResetPeripheral {
    fn new(trigger_at: usize, set_to: usize, counter_controller: ControllerPort<CounterOperation, usize>) -> Self {
        CounterResetPeripheral {
            trigger_at,
            set_to,
            counter_controller,
            observed_count: None,
        }
    }
}

impl Transmit for CounterResetPeripheral {
    fn transmit(&mut self) {
        if let Some(current_count) = self.observed_count {
            if current_count == self.trigger_at {
                self.counter_controller.transmit(CounterOperation::Set(self.set_to));
            }
        }
    }
}

impl Update for CounterResetPeripheral {
    fn update(&mut self) {
        self.observed_count = self.counter_controller.receive();
    }
}

#[test]
fn latency_test_two_cycle_delay() {
    let mut interconnect = Interconnect::new_with_latency(2);
    let mut counters = vec![CounterPeripheral::new(interconnect.peripheral_connector)];
    let mut counter_resets = vec![CounterResetPeripheral::new(10, 20, interconnect.controller_connector)];

    for n in 0..13 {
        counters.iter_mut().for_each(transmit);
        counter_resets.iter_mut().for_each(transmit);

        interconnect.interconnect_state.update();

        counters.iter_mut().for_each(update);
        counter_resets.iter_mut().for_each(update);

        let count = counters.first().unwrap().count;
        if n == 12 {
            assert_eq!(count, 20);
        } else {
            assert_eq!(count, n + 1);
        }
    }
}

#[test]
fn latency_test_three_cycle_delay() {
    let mut interconnect = Interconnect::new_with_latency(3);
    let mut counters = vec![CounterPeripheral::new(interconnect.peripheral_connector)];
    let mut counter_resets = vec![CounterResetPeripheral::new(9, 20, interconnect.controller_connector)];

    let end = 13;
    for n in 0..end {
        counters.iter_mut().for_each(transmit);
        counter_resets.iter_mut().for_each(transmit);

        interconnect.interconnect_state.update();

        counters.iter_mut().for_each(update);
        counter_resets.iter_mut().for_each(update);

        let count = counters.first().unwrap().count;
        if n == end - 1 {
            assert_eq!(count, 20);
        } else {
            assert_eq!(count, n + 1);
        }
    }
}

#[test]
fn latency_test_four_cycle_delay() {
    let mut interconnect = Interconnect::new_with_latency(4);
    let mut counters = vec![CounterPeripheral::new(interconnect.peripheral_connector)];
    let mut counter_resets = vec![CounterResetPeripheral::new(8, 20, interconnect.controller_connector)];

    let end = 13;
    for n in 0..end {
        counters.iter_mut().for_each(transmit);
        counter_resets.iter_mut().for_each(transmit);

        interconnect.interconnect_state.update();

        counters.iter_mut().for_each(update);
        counter_resets.iter_mut().for_each(update);

        let count = counters.first().unwrap().count;
        if n == end - 1 {
            assert_eq!(count, 20);
        } else {
            assert_eq!(count, n + 1);
        }
    }
}

#[test]
fn counter_before_reset_order_test() {
    let mut interconnect = Interconnect::new();
    let mut counters = vec![CounterPeripheral::new(interconnect.peripheral_connector)];
    let mut counter_resets = vec![CounterResetPeripheral::new(10, 20, interconnect.controller_connector)];

    for n in 0..12 {
        counters.iter_mut().for_each(transmit);
        counter_resets.iter_mut().for_each(transmit);

        interconnect.interconnect_state.update();

        counters.iter_mut().for_each(update);
        counter_resets.iter_mut().for_each(update);

        let count = counters.first().unwrap().count;
        if n == 11 {
            assert_eq!(count, 20);
        } else {
            assert_eq!(count, n + 1);
        }
    }
}

#[test]
fn reset_before_counter_order_test() {
    let mut interconnect = Interconnect::new();
    let mut counters = vec![CounterPeripheral::new(interconnect.peripheral_connector)];
    let mut counter_resets = vec![CounterResetPeripheral::new(10, 20, interconnect.controller_connector)];

    for n in 0..12 {
        counter_resets.iter_mut().for_each(transmit);
        counters.iter_mut().for_each(transmit);

        interconnect.interconnect_state.update();

        counter_resets.iter_mut().for_each(update);
        counters.iter_mut().for_each(update);

        let count = counters.first().unwrap().count;
        if n == 11 {
            assert_eq!(count, 20);
        } else {
            assert_eq!(count, n + 1);
        }
    }
}

#[test]
fn works_over_vec_counters() {
    let mut counters = vec![];
    let mut counter_resets = vec![];
    let mut interconnect_states = vec![];

    for _ in 0..5 {
        let interconnect = Interconnect::new();

        counters.push(CounterPeripheral::new(interconnect.controller_connector));
        counter_resets.push(CounterResetPeripheral::new(10, 20, interconnect.peripheral_connector));
        interconnect_states.push(interconnect.interconnect_state);
    }

    for n in 0..12 {
        counters.iter_mut().for_each(transmit);
        counter_resets.iter_mut().for_each(transmit);

        interconnect_states.iter_mut().for_each(update);

        counters.iter_mut().for_each(update);
        counter_resets.iter_mut().for_each(update);

        for c in counters.iter_mut() {
            let count = c.count;
            if n == 11 {
                assert_eq!(count, 20);
            } else {
                assert_eq!(count, n + 1);
            }
        }
    }
}
