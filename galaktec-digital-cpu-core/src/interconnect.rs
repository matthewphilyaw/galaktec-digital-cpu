use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Interconnect<Input, Output>
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    input: Option<Input>,
    output: Option<Output>,
}

#[derive(Debug)]
pub struct Peripheral<Input, Output>
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    interconnect: Rc<RefCell<Interconnect<Input, Output>>>
}

impl <Input, Output> Peripheral<Input, Output>
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    pub fn transmit(&mut self, input: Input) {
        self.interconnect.as_ref().borrow_mut().input = Some(input);
    }

    pub fn receive(&mut self) -> Option<Output> {
        std::mem::replace(&mut self.interconnect.borrow_mut().output, None)
    }
}


#[derive(Debug)]
pub struct Controller<Input, Output>
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    interconnect: Rc<RefCell<Interconnect<Input, Output>>>
}


impl<Input, Output> Controller<Input, Output>
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    pub fn receive(&mut self) -> Option<Input> {
        std::mem::replace(&mut self.interconnect.borrow_mut().input, None)
    }

    pub fn transmit(&mut self, output: Output) {
        self.interconnect.borrow_mut().output = Some(output);
    }
}

pub fn interconnect<Input, Output>() -> (Controller<Input, Output>, Peripheral<Input, Output>)
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    let interconnect = Rc::new(RefCell::new(Interconnect {
        input: None,
        output: None,
    }));

    (
        Controller { interconnect: interconnect.clone() },
        Peripheral { interconnect },
    )
}
