use std::fmt::Debug;
use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq)]
pub enum State<T> {
    Ready(T),
    Pending,
    None
}

pub trait Interface<Command, Data>: Debug
where
    Command: Debug + Clone + PartialEq,
    Data: Debug + Clone + PartialEq,
{
    fn send_command(&mut self, command: Command);
    fn command_acknowledged(&self) -> bool;
    fn state(&self) -> State<Data>;
}

pub trait Implementation<Command, Data>: Debug
where
    Command: Debug + Clone + PartialEq,
    Data: Debug + Clone + PartialEq,
{
    fn read_command(&self) -> Option<Command>;
    fn acknowledge_command(&mut self);
    fn set_state(&mut self, state: State<Data>);
}


#[derive(Debug)]
pub struct Interconnect<Command, Data>
where
    Command: Debug + Clone + PartialEq,
    Data: Debug + Clone + PartialEq,
{
    command: Option<Command>,
    internal_state: State<Data>,
    command_acknowledged: bool
}

impl<Command, Data> Interface<Command, Data> for Interconnect<Command, Data>
where
    Command: Debug + Clone + PartialEq,
    Data: Debug + Clone + PartialEq,
{
    fn send_command(&mut self, command: Command) {
        self.command = Some(command);
        self.command_acknowledged = false;
    }

    fn command_acknowledged(&self) -> bool {
        self.command_acknowledged
    }

    fn state(&self) -> State<Data> {
        self.internal_state.clone()
    }
}

impl<Command, Data> Implementation<Command, Data> for Interconnect<Command, Data>
where
    Command: Debug + Clone + PartialEq,
    Data: Debug + Clone + PartialEq,
{
    fn read_command(&self) -> Option<Command> {
        self.command.clone()
    }

    fn acknowledge_command(&mut self) {
        self.command_acknowledged = true;
    }

    fn set_state(&mut self, state: State<Data>) {
        self.internal_state = state;
        self.command = None;
    }
}