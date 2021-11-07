mod chr {
    pub const CNTRL: u8 = 0x01;
    pub const SPACE: u8 = 0x02;
    pub const PUNCT: u8 = 0x04;
    pub const DIGIT: u8 = 0x08;
    pub const XDIGIT: u8 = 0x10;
    pub const UPPER: u8 = 0x20;
    pub const LOWER: u8 = 0x40;
    pub const IDENT: u8 = 0x80;
    pub const ALPHA: u8 = LOWER | UPPER;
    pub const ALNUM: u8 = ALPHA | DIGIT;
    pub const GRAPH: u8 = ALNUM | PUNCT;
    pub const BITS: [u8; 257] = [
        0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 152, 152, 152, 152, 152, 152, 152,
        152, 152, 152, 4, 4, 4, 4, 4, 4, 4, 176, 176, 176, 176, 176, 176, 160, 160, 160, 160, 160,
        160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 4, 4, 4, 4, 132,
        4, 208, 208, 208, 208, 208, 208, 192, 192, 192, 192, 192, 192, 192, 192, 192, 192, 192,
        192, 192, 192, 192, 192, 192, 192, 192, 192, 4, 4, 4, 4, 1, 128, 128, 128, 128, 128, 128,
        128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
        128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
        128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
        128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
        128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
        128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
        128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    ];

    #[macro_export]
    macro_rules! is_alpha {
        ($x: expr, $y: expr) => {
            (BITS[1 + $x as usize] & $y) != 0
        };
    }

    #[macro_export]
    macro_rules! to_upper {
        ($x: expr) => {
            $x - (BITS[$x] >> 1)
        };
    }

    #[macro_export]
    macro_rules! to_lower {
        ($x: expr) => {
            $x + BITS[$x]
        };
    }
}

#[cfg(test)]
mod tests {
    use super::chr::*;
    use crate::is_alpha;

    #[test]
    fn is_space() {
        assert_eq!(is_alpha!(' ', SPACE), true);
        assert_eq!(is_alpha!('\t', SPACE), true);
        assert_eq!(is_alpha!('\n', SPACE), true);
        assert_eq!(is_alpha!('\r', SPACE), true);
    }

    #[test]
    fn is_not_space() {
        assert_eq!(is_alpha!('a', SPACE), false);
        assert_eq!(is_alpha!('1', SPACE), false);
        assert_eq!(is_alpha!('.', SPACE), false);
        assert_eq!(is_alpha!(';', SPACE), false);
    }

    #[test]
    fn is_digit() {
        for c in '0'..'9' {
            assert_eq!(is_alpha!(c, DIGIT), true);
        }
    }

    #[test]
    fn is_not_digit() {
        for c in 'a'..'z' {
            assert_eq!(is_alpha!(c, DIGIT), false);
        }
        for c in 'A'..'Z' {
            assert_eq!(is_alpha!(c, DIGIT), false);
        }
    }

    #[test]
    fn is_upper() {
        for c in 'A'..'Z' {
            assert_eq!(is_alpha!(c, UPPER), true);
        }
    }

    #[test]
    fn is_not_upper() {
        for c in 'a'..'z' {
            assert_eq!(is_alpha!(c, UPPER), false);
        }

        for c in '0'..'9' {
            assert_eq!(is_alpha!(c, UPPER), false);
        }
    }

    #[test]
    fn is_lower() {
        for c in 'a'..'z' {
            assert_eq!(is_alpha!(c, LOWER), true);
        }
    }

    #[test]
    fn is_not_lower() {
        for c in 'A'..'Z' {
            assert_eq!(is_alpha!(c, LOWER), false);
        }

        for c in '0'..'9' {
            assert_eq!(is_alpha!(c, LOWER), false);
        }
    }
}
