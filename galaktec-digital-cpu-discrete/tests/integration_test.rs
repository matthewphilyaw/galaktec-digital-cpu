use galaktec_digital_cpu_discrete::{
    Device, Discrete, EventHandler, GenericClock, Observable, StepPhase, Unit,
};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
enum ExternalCounterEvent {
    Set(usize),
    Reset,
}

#[derive(Debug)]
struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Box<dyn Unit<ExternalCounterEvent, usize>> {
        Box::new(Counter { count: 0 })
    }

    fn process_events(&mut self, events: &Vec<ExternalCounterEvent>) {
        for e in events.iter() {
            println!("processing events:  {:?}", e);
            match e {
                ExternalCounterEvent::Set(new_count) => {
                    println!("processing count:  {}", new_count);
                    self.count = *new_count
                }
                ExternalCounterEvent::Reset => self.count = 0,
            }
        }
    }
}

impl Unit<ExternalCounterEvent, usize> for Counter {
    fn step(&mut self, phase: StepPhase, external_event_queue: &Vec<ExternalCounterEvent>) {
        println!("The queue is set: {:?}", &external_event_queue);
        match phase {
            StepPhase::First => self.count += 1,
            StepPhase::Second | StepPhase::Third => {
                println!("processing phase: {:?}", phase);
                self.process_events(&external_event_queue)
            }
        };
    }

    fn commit(&mut self) -> usize {
        self.count
    }
}

trait CounterInterface: Debug {
    fn set_counter(&mut self, value: usize);
    fn reset_counter(&mut self);
    fn counter_value(&self) -> usize;
}

impl CounterInterface for Device<ExternalCounterEvent, usize> {
    fn set_counter(&mut self, value: usize) {
        self.add_event(ExternalCounterEvent::Set(value));
    }

    fn reset_counter(&mut self) {
        self.add_event(ExternalCounterEvent::Reset);
    }

    fn counter_value(&self) -> usize {
        self.state()
    }
}

#[derive(Debug)]
struct CounterReset {
    trigger_at: usize,
    set_to: usize,
    counter: Rc<RefCell<dyn CounterInterface>>,
}

impl Unit<usize, usize> for CounterReset {
    fn step(&mut self, phase: StepPhase, _external_event_queue: &Vec<usize>) {
        match phase {
            StepPhase::First => {
                let current_count = self.counter.borrow().counter_value();
                println!("counter value: {}", current_count);
                if current_count == self.trigger_at {
                    println!("setting counter to: {}", self.set_to);
                    self.counter.borrow_mut().set_counter(self.set_to);
                }
            }
            _ => (),
        }
    }

    fn commit(&mut self) -> usize {
        12
    }
}

impl CounterReset {
    fn new(
        trigger_at: usize,
        set_to: usize,
        counter: Rc<RefCell<dyn CounterInterface>>,
    ) -> Box<dyn Unit<usize, usize>> {
        Box::new(CounterReset {
            trigger_at,
            set_to,
            counter,
        })
    }
}

#[test]
fn reset_clock() {
    let counter_device = Device::new_rc_ref(Counter::new());
    let reset_device = Device::new_rc_ref(CounterReset::new(10, 20, counter_device.clone()));

    let mut clock = GenericClock::new(vec![reset_device.clone(), counter_device.clone()]);

    for n in 0..11 {
        println!(
            "counter before step {}: {}",
            n,
            counter_device.borrow().state()
        );
        clock.step();
        println!(
            "counter after step {}: {}",
            n,
            counter_device.borrow().state()
        );
    }

    assert_eq!(counter_device.borrow().state(), 20);
}
