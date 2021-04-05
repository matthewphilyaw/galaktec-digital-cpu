use std::fmt::Debug;

pub trait Discrete: Debug {
    fn send(&mut self);
    fn update(&mut self);
}
