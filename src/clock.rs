/// Represents a clockable component in the system
///
/// Each component will get two signals a clock high and a clock low. On clock high a component should
/// only update any internal state that will NOT change the public API. This allows all components to
/// react to each others changes from the last clock low signal. Otherwise the behavior would change
/// depending on the order the components were clocked.
///
/// Clock low would be the signal to commit the changes so that the public facing API reflects the
/// internal changes created by clock high. Effectively propagates the changes in lock step.
pub trait ClockedHigh {
    fn clock_high(&mut self);
}

pub trait ClockedLow {
    fn clock_low(&mut self);
}
