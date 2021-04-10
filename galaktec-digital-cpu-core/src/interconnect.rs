use std::fmt::Debug;

pub trait Peripheral<Input, Output>: Debug
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    fn transmit(&mut self, command: Input);
    fn receive(&mut self) -> Option<Output>;
}

pub trait Controller<Input, Output>: Debug
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    fn receive(&mut self) -> Option<Input>;
    fn transmit(&mut self, output: Output);
}

#[derive(Debug)]
pub struct Interconnect<Input, Output>
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    input: Option<Input>,
    output: Option<Output>,
}

impl<Input, Output> Interconnect<Input, Output>
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    pub fn new() -> Self {
        Interconnect {
            input: None,
            output: None,
        }
    }
}

impl<Input, Output> Peripheral<Input, Output> for Interconnect<Input, Output>
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    fn transmit(&mut self, input: Input) {
        self.input = Some(input);
    }

    fn receive(&mut self) -> Option<Output> {
        std::mem::replace(&mut self.output, None)
    }
}

impl<Input, Output> Controller<Input, Output> for Interconnect<Input, Output>
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    fn receive(&mut self) -> Option<Input> {
        std::mem::replace(&mut self.input, None)
    }

    fn transmit(&mut self, output: Output) {
        self.output = Some(output);
    }
}
