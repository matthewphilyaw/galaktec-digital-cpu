use std::fmt::Debug;

pub trait Transmit: Debug {
    fn transmit(&mut self) {}
}

pub trait Update: Debug {
    fn update(&mut self) {}
}

pub fn transmit(imp: &mut impl Transmit) {
    imp.transmit();
}

pub fn update(imp: &mut impl Update) {
    imp.update();
}
