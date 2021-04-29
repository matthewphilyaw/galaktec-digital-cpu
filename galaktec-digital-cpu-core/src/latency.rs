#[derive(Debug)]
pub(crate) struct CycleDelay {
    delay: usize,
    counter: usize,
}

impl CycleDelay {
    pub(crate) fn new(delay: usize) -> Self {
        assert_ne!(delay, 0, "Delay must be one or greater.");
        CycleDelay {
            delay: delay - 1,
            counter: 0,
        }
    }

    pub(crate) fn step(&mut self) {
        if self.counter > self.delay {
            return;
        }

        self.counter += 1;
    }

    pub(crate) fn reset(&mut self) {
        self.counter = 0;
    }

    pub(crate) fn elapsed(&mut self) -> bool {
        self.counter > self.delay
    }
}
