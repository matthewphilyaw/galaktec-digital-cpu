#[derive(Debug, Copy, Clone)]
pub struct AddressMap {
    base_address: usize,
    block_size: usize,
}

impl AddressMap {
    pub fn new(base_address: usize, block_size: usize) -> Self {
        debug_assert_ne!(block_size, 0, "Block size can not be zero");

        AddressMap {
            base_address,
            block_size,
        }
    }

    fn end_address(&self) -> usize {
        self.base_address + self.block_size
    }

    pub fn address_in_range(&self, address: usize) -> bool {
        self.base_address <= address && address < self.end_address()
    }

    pub fn relative_address(&self, address: usize) -> usize {
        debug_assert!(self.address_in_range(address), "Address not in range");
        address - self.base_address
    }
}
