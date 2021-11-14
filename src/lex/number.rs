pub enum LexNumberValue {
    Invalid,
    UInt(u64),
    Float(f64),
}

impl LexNumberValue {
    pub fn new_binary(v: &Vec<char>) -> Self {
        let mut value: u64 = 0;
        for c in v.iter().skip(2) {
            value = (value << 1)
                + match c {
                    &'0' => 0,
                    &'1' => 1,
                    _ => return LexNumberValue::Invalid,
                }
        }

        LexNumberValue::UInt(value)
    }

    pub fn new_hex(v: &Vec<char>) -> Self {
        let mut value: u64 = 0;
        for c in v.iter().skip(2) {
            value = (value << 4)
                + u64::from(match c {
                    &('0'..='9') => u32::from(*c) - u32::from('0'),
                    &('a'..='f') => u32::from(*c) - u32::from('a') + 10,
                    &('A'..='F') => u32::from(*c) - u32::from('A') + 10,
                    _ => return LexNumberValue::Invalid,
                });
        }

        LexNumberValue::UInt(value)
    }

    pub fn new_dec(v: &Vec<char>) -> Self {
        let mut value: u64 = 0;
        for c in v.iter() {
            value = (value * 10)
                + u64::from(match c {
                    &('0'..='9') => u32::from(*c) - u32::from('0'),
                    _ => return LexNumberValue::Invalid,
                });
        }

        LexNumberValue::UInt(value)
    }

    pub fn is_invalid(&self) -> bool {
        match self {
            &LexNumberValue::Invalid => true,
            _ => false,
        }
    }

    pub fn as_int(&self) -> u64 {
        match self {
            &LexNumberValue::UInt(value) => value,
            &LexNumberValue::Float(value) => value as u64,
            _ => 0,
        }
    }
}
