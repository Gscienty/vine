pub enum LexNumberValue {
    Invalid,
    UInt(u64),
    Float(f64),
}

impl LexNumberValue {
    pub fn new(v: &Vec<char>) -> Self {
        if v.starts_with(&['0', 'b']) {
            LexNumberValue::new_binary(v)
        } else if v.starts_with(&['0', 'x']) {
            LexNumberValue::new_hex(v)
        } else {
            LexNumberValue::new_dec(v)
        }
    }

    pub fn new_binary(v: &Vec<char>) -> Self {
        if !v.starts_with(&['0', 'b']) {
            return LexNumberValue::Invalid;
        }

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
        if !v.starts_with(&['0', 'x']) {
            return LexNumberValue::Invalid;
        }

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
        let mut frac: f64 = 0f64;
        let mut frac_base: f64 = 0.1f64;
        let mut is_int: bool = true;
        for c in v.iter() {
            if c == &'.' {
                if !is_int {
                    return LexNumberValue::Invalid;
                }

                is_int = false;
                continue;
            }

            if is_int {
                value = (value * 10)
                    + u64::from(match c {
                        &('0'..='9') => u32::from(*c) - u32::from('0'),
                        _ => return LexNumberValue::Invalid,
                    });
            } else {
                frac = frac
                    + frac_base
                        * f64::from(match c {
                            &('0'..='9') => u32::from(*c) - u32::from('0'),
                            _ => return LexNumberValue::Invalid,
                        });
                frac_base = frac_base / 10f64;
            }
        }

        if is_int {
            LexNumberValue::UInt(value)
        } else {
            LexNumberValue::Float(value as f64 + frac)
        }
    }

    pub fn is_invalid(&self) -> bool {
        match self {
            &LexNumberValue::Invalid => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            &LexNumberValue::UInt(_) => true,
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

    pub fn as_float(&self) -> f64 {
        match self {
            &LexNumberValue::UInt(value) => value as f64,
            &LexNumberValue::Float(value) => value,
            _ => 0f64,
        }
    }
}

impl PartialEq for LexNumberValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UInt(l0), Self::UInt(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LexNumberValue;

    fn str_to_vec(s: &str) -> Vec<char> {
        let mut ret = Vec::new();
        for chr in s.chars() {
            ret.push(chr);
        }
        ret
    }

    fn assert_binary_eq(s: &str, value: u64) {
        let actual = LexNumberValue::new_binary(&str_to_vec(s));

        assert!(!actual.is_invalid());
        assert_eq!(actual.as_int(), value);
        assert!(actual.is_int());
    }

    fn assert_binary_invalid(s: &str) {
        let actual = LexNumberValue::new_binary(&str_to_vec(s));

        assert!(actual.is_invalid());
        assert_eq!(actual.as_int(), 0);
    }

    fn assert_hex_eq(s: &str, value: u64) {
        let actual = LexNumberValue::new_hex(&str_to_vec(s));

        assert!(!actual.is_invalid());
        assert_eq!(actual.as_int(), value);
        assert!(actual.is_int());
    }

    fn assert_hex_invalid(s: &str) {
        let actual = LexNumberValue::new_hex(&str_to_vec(s));

        assert!(actual.is_invalid());
        assert_eq!(actual.as_int(), 0);
    }

    fn assert_dec_eq_int(s: &str, value: u64) {
        let actual = LexNumberValue::new_dec(&str_to_vec(s));

        assert!(!actual.is_invalid());
        assert_eq!(actual.as_int(), value);
        assert!(actual.is_int());
    }

    fn assert_dec_eq_float(s: &str, value: f64) {
        let actual = LexNumberValue::new_dec(&str_to_vec(s));

        assert!(!actual.is_invalid());
        assert_eq!(actual.as_float(), value);
        assert!(!actual.is_int());
    }

    fn assert_dec_invalid(s: &str) {
        let actual = LexNumberValue::new_dec(&str_to_vec(s));

        assert!(actual.is_invalid());
        assert!(!actual.is_int());
        assert_eq!(actual.as_int(), 0);
    }

    #[test]
    fn test_lex_number_binary() {
        assert_binary_eq("0b11010011", 0xd3);
        assert_binary_eq("0b000000000000000000111", 0x07);

        assert_binary_invalid("0x11011111");
        assert_binary_invalid("11010011");
        assert_binary_invalid("0b11010012");
    }

    #[test]
    fn test_lex_number_hex() {
        assert_hex_eq("0xd3", 0xd3);
        assert_hex_eq("0xD3", 0xd3);
        assert_hex_eq("0x00d3D3", 0xd3d3);
        assert_hex_eq("0x001234", 0x1234);
        assert_hex_eq("0x100000", 0x100000);

        assert_hex_invalid("0b1111101");
        assert_hex_invalid("abcd");
        assert_hex_invalid("0xag");
    }

    #[test]
    fn test_lex_number_int_dec() {
        assert_dec_eq_int("100", 100);
        assert_dec_eq_int("00123", 123);
        assert_dec_eq_float("100.0", 100f64);
        assert_dec_eq_float(".0", 0f64);
        assert_dec_eq_float("123.45", 123.45f64);

        assert_dec_invalid("100.0.0");
        assert_dec_invalid("..");
        assert_dec_invalid("1a34");
        assert_dec_invalid("0.b123");
    }
}
