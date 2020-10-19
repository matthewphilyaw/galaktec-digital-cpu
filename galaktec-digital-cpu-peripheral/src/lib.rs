mod address_map;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DataWidth {
    Byte,
    HalfWord,
    Word,
}

pub enum Event {
    Write {
        data_width: DataWidth,
        address: u32,
        data: u32,
    },
    Read {
        data_width: DataWidth,
        address: u32,
    },
}
