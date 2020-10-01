
/// Represents a clockable component in the system
///
/// Each component will get two signals a clock high and a clock low. On clock high a component should
/// only update any internal state that will NOT change the public API. This allows all components to
/// react to each others changes from the last clock low signal. Otherwise the behavior would change
/// depending on the order the components were clocked.
///
/// Clock low would be the signal to commit the changes so that the public facing API reflects the
/// internal changes created by clock high. Effectively propagates the changes in lock step.
pub trait Clocked {
    /// A clocked component should ONLY modify internal state on clock high
    /// not presenting in external changes.
    fn clock_high(&mut self);
    /// On clock low the component should then modify public facing state so that on the next
    /// clock_high components will "see" the changes.
    fn clock_low(&mut self);
}

pub trait Cpu: Clocked {
    fn load_binary(&self, buffer: &Vec<u32>) -> ();
    fn reset(&mut self) -> ();
}