use galaktec_digital_cpu_discrete::{DiscreteDevice, Observable, GenericClock, ReactiveDevice, React};

use std::cell::{RefCell};
use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
enum CounterOperation {
    Set(usize),
    Reset,
}

#[derive(Debug)]
struct Counter {
    events: Vec<CounterOperation>,
    count: usize,
    last_count: usize
}

impl Counter {
    fn new() -> Self {
        Counter {
            events: vec![],
            count: 0,
            last_count: 0
        }
    }

    fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

impl ReactiveDevice for Counter { }

impl DiscreteDevice for Counter {
    fn activate(&mut self) {
        self.count += 1;
    }

    fn settle(&mut self) {
        for ev in self.events.iter() {
            match ev {
                CounterOperation::Set(value) => self.count = *value,
                CounterOperation::Reset => self.count = 0
            }
        }
    }

    fn deactivate(&mut self) {
        self.last_count = self.count;
    }
}

impl Observable for Counter {
    type State = usize;

    fn state(&self) -> usize {
        self.last_count
    }
}

impl React for Counter {
    type Event = CounterOperation;

    fn react(&mut self, event: Self::Event) {
        self.events.push(event);
    }
}

type CounterDevice = Rc<RefCell<dyn ReactiveDevice<Event = CounterOperation, State = usize>>>;
#[derive(Debug)]
struct CounterReset {
    trigger_at: usize,
    set_to: usize,
    counter_device: CounterDevice,
}

impl CounterReset {
    fn new(
        trigger_at: usize,
        set_to: usize,
        counter_device: CounterDevice
    ) -> Self {
        CounterReset {
            trigger_at,
            set_to,
            counter_device
        }
    }

    fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

impl DiscreteDevice for CounterReset {
    fn activate(&mut self) {
        let current_count = self.counter_device.borrow().state();

        if current_count == self.trigger_at {
            self.counter_device.borrow_mut().react(CounterOperation::Set(
                self.set_to
            ));
        }
    }

    fn settle(&mut self) {

    }

    fn deactivate(&mut self) {

    }
}

#[test]
fn counter_test() {
    let counter = Counter::new().into_rc();
    let counter_reset = CounterReset::new(
        10,
        20,
        counter.clone(),
    ).into_rc();


    let mut clock = GenericClock::new(
        vec![
            counter.clone(),
            counter_reset
        ]
    );

    for n in 0..11 {
        println!(
            "counter before step {}: {}",
            n,
            counter.borrow().state()
        );
        clock.step();
        println!(
            "counter after step {}: {}",
            n,
            counter.borrow().state()
        );
    }

    assert_eq!(counter.borrow().count, 20);
}
