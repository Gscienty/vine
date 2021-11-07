pub struct TagValue {
    value: u64,
}

impl TagValue {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn u64_equal(&self, value: u64) -> bool {
        self.value == value
    }

    pub fn get_lo_u32(&self) -> u32 {
        (self.value & 0xffffffff) as u32
    }

    pub fn get_u64(&self) -> u64 {
        self.value
    }

    pub fn set_u64(&mut self, value: u64) {
        self.value = value;
    }
}
