enum Error {
    CurrentlyAsserted,
    CurrentlyDeasserted,
    NoAssertionForAddress
}

pub enum AssertionKind {
    Write,
    Read
}

pub struct Assertion {
    pub kind: AssertionKind,
    pub address: u32,
    pub data: Option<u32>,
}

pub struct DataBus {
    assertion: Option<Assertion>,
    data: Option<u32>
}

impl DataBus {
    pub fn new() -> Self {
        DataBus {
            assertion: None,
            data: None
        }
    }

    pub fn assert(&mut self, assertion: Assertion) -> Result<(), Error> {
        if let None = self.assertion {
            self.assertion = Some(assertion);
            Ok(())
        } else {
            Err(Error::CurrentlyAsserted)
        }
    }

    pub fn deassert(&mut self, data: Option<u32>) -> Result<(), Error> {
        if let None = self.assertion {
            Err(Error::CurrentlyDeasserted)
        } else {
            self.data = data;
            self.assertion = None;
            Ok(())
        }
    }

    fn address_within_range(address: u32, target_start: u32, target_end: u32) -> bool {
        target_start <= address && address < target_end
    }

    pub fn get_assertion(&mut self, start_address: u32, end_address: u32) -> Result<&Assertion, Error> {
        match &self.assertion {
            Some(a) if start_address <= a.address && a.address <= end_address => Ok(a),
            None => Err(Error::CurrentlyDeasserted),
            _ => Err(Error::NoAssertionForAddress)
        }
    }

    pub fn asserted(&self) -> bool {
        if let None = self.assertion {
            false
        } else {
            true
        }
    }
}
