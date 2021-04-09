use crate::interconnect::Peripheral;
use std::fmt::Debug;

pub trait DiscreteUnit: Debug {
    fn send(&mut self);
    fn update(&mut self);
}
