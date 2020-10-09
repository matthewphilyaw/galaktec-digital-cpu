
#[derive(Debug, Copy, Clone)]
pub struct AddressMap {
    base_address: u32,
    block_size: u32
}

impl AddressMap {
    pub fn new(base_address: u32, block_size: u32) -> Self {
        debug_assert_ne!(block_size, 0, "Block size can not be zero");

        AddressMap {
            base_address,
            block_size
        }
    }

    fn end_address(&self) -> u32 {
        self.base_address + self.block_size
    }

    pub fn address_in_range(&self, address: u32) -> bool {
        self.base_address <= address && address < self.end_address()
    }

    pub fn relative_address(&self, address: u32) -> u32 {
        debug_assert!(self.address_in_range(address), "Address not in range");
        address - self.base_address
    }
}