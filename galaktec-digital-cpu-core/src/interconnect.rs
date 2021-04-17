use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
struct Signal<Data>
where
    Data: Debug + Clone + PartialEq,
{
    data: Option<Data>
}

impl<Data> Signal<Data>
where
    Data: Debug + Clone + PartialEq,
{
    fn new() -> Self {
        Signal {
            data: None
        }
    }

    fn set_data(&mut self, data: Data) {
        self.data = Some(data);
    }

    fn consume_data(&mut self) -> Option<Data> {
        std::mem::replace(&mut self.data, None)
    }
}


#[derive(Debug)]
pub struct FullDuplexPort<Input, Output>
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    input_signal: Rc<RefCell<Signal<Input>>>,
    output_signal: Rc<RefCell<Signal<Output>>>,
}

impl<Input, Output> FullDuplexPort<Input, Output>
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    pub fn transmit(&mut self, output: Output) {
        &self.output_signal.borrow_mut().set_data(output);
    }

    pub fn receive(&mut self) -> Option<Input> {
        self.input_signal.borrow_mut().consume_data()
    }
}



pub fn create_interconnect<Input, Output>() -> (FullDuplexPort<Input, Output>, FullDuplexPort<Output, Input>)
where
    Input: Debug + Clone + PartialEq,
    Output: Debug + Clone + PartialEq,
{
    let signal_a: Rc<RefCell<Signal<Input>>> = Rc::new(RefCell::new(Signal::new()));
    let signal_b: Rc<RefCell<Signal<Output>>> = Rc::new(RefCell::new(Signal::new()));


    (
        FullDuplexPort {
            input_signal: signal_a.clone(),
            output_signal: signal_b.clone()
        },
        FullDuplexPort {
            input_signal: signal_b,
            output_signal: signal_a
        },
    )
}
