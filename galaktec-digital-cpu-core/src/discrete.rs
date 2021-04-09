use std::fmt::Debug;
use crate::interconnect::Peripheral;

pub trait DiscreteUnit: Debug
{
    fn send(&mut self);
    fn update(&mut self);
}
