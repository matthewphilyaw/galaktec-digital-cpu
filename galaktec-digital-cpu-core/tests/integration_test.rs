use galaktec_digital_cpu_core::{create_zero_latency_interconnect, transmit, update, ControllerConnector, PeripheralConnector, Transmit, Update, create_latent_interconnect};
use std::ops::DerefMut;

/* -------------- Counter Peripheral ---------------------- */

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum CounterOperation {
    Set(usize),
}

#[derive(Debug)]
struct CounterPeripheral {
    count: usize,
    connector: PeripheralConnector<CounterOperation, usize>,
}

impl CounterPeripheral {
    fn new(connector: PeripheralConnector<CounterOperation, usize>) -> Self {
        CounterPeripheral {
            count: 0,
            connector,
        }
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
    counter_controller: ControllerConnector<CounterOperation, usize>,
    observed_count: Option<usize>,
}

impl CounterResetPeripheral {
    fn new(
        trigger_at: usize,
        set_to: usize,
        counter_controller: ControllerConnector<CounterOperation, usize>,
    ) -> Self {
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
                self.counter_controller
                    .transmit(CounterOperation::Set(self.set_to));
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
fn input_latency_test_one_cycle_delay() {
    let (c, p, signals) = create_latent_interconnect(1, 0);
    let mut counters = vec![CounterPeripheral::new(c)];
    let mut counter_resets = vec![CounterResetPeripheral::new(10, 20, p)];

    for n in 0..13 {
        counters.iter_mut().for_each(transmit);
        counter_resets.iter_mut().for_each(transmit);

        update(signals.0.as_ref().borrow_mut().deref_mut());
        update(signals.1.as_ref().borrow_mut().deref_mut());

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
fn output_latency_test_one_cycle_delay() {
    let (c, p, signals) = create_latent_interconnect(0, 1);
    let mut counters = vec![CounterPeripheral::new(c)];
    let mut counter_resets = vec![CounterResetPeripheral::new(10, 20, p)];

    for n in 0..13 {
        counters.iter_mut().for_each(transmit);
        counter_resets.iter_mut().for_each(transmit);

        update(signals.0.as_ref().borrow_mut().deref_mut());
        update(signals.1.as_ref().borrow_mut().deref_mut());

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
fn counter_before_reset_order_test() {
    let (c, p, signals) = create_zero_latency_interconnect();
    let mut counters = vec![CounterPeripheral::new(c)];
    let mut counter_resets = vec![CounterResetPeripheral::new(10, 20, p)];

    for n in 0..12 {
        counters.iter_mut().for_each(transmit);
        counter_resets.iter_mut().for_each(transmit);

        update(signals.0.as_ref().borrow_mut().deref_mut());
        update(signals.1.as_ref().borrow_mut().deref_mut());

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
    let (c, p, signals) = create_zero_latency_interconnect();
    let mut counters = vec![CounterPeripheral::new(c)];
    let mut counter_resets = vec![CounterResetPeripheral::new(10, 20, p)];

    for n in 0..12 {
        counter_resets.iter_mut().for_each(transmit);
        counters.iter_mut().for_each(transmit);

        update(signals.0.as_ref().borrow_mut().deref_mut());
        update(signals.1.as_ref().borrow_mut().deref_mut());

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
    let mut signal_inputs = vec![];
    let mut signal_outputs = vec![];

    for _ in 0..5 {
        let (c, p, signals) = create_zero_latency_interconnect();
        counters.push(CounterPeripheral::new(c));
        counter_resets.push(CounterResetPeripheral::new(10, 20, p));

        signal_inputs.push(signals.0);
        signal_outputs.push(signals.1);
    }

    for n in 0..12 {
        counters.iter_mut().for_each(transmit);
        counter_resets.iter_mut().for_each(transmit);

        signal_inputs.iter_mut().for_each(|s| update(s.as_ref().borrow_mut().deref_mut()));
        signal_outputs.iter_mut().for_each(|s| update(s.as_ref().borrow_mut().deref_mut()));

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