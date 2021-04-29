use std::fmt::Debug;

pub trait Transmit: Debug {
    fn transmit(&mut self) {}
}

pub trait Update: Debug {
    fn update(&mut self) {}
}

pub fn transmit<T: Transmit>(imp: &mut T) {
    imp.transmit();
}

pub fn update<T: Update>(imp: &mut T) {
    imp.update();
}
